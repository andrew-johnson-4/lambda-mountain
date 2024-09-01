
From Coq Require Import ZArith String List.
Require Import Coq.Program.Tactics.
From MMaps Require Import MMaps.
Open Scope string.
Open Scope Z.

Variable K V : Type.

Module ZM := MMaps.RBT.Make(BinInt.Z).

Fixpoint list_assoc (kv: list (K * V)) (beq: K -> K -> bool) (k: K) (default: V): V := 
   match kv with
   | cons (lk , lv) kvs => if beq k lk then lv else (list_assoc kvs beq k default)
   | nil => default
   end.

Fixpoint list_contains (known: list string) (expect: string): bool :=
   match known with
   | cons kk kvs => if String.eqb expect kk then true else (list_contains kvs expect)
   | nil => false
   end.

Fixpoint list_implies (known: list string) (expect: list string): bool :=
   match expect with
   | cons kk kvs => if list_contains known kk then (list_implies known kvs) else false
   | nil => true
   end.

(* Memory Is Denominated in Bytes *)
Record RegionByte := mkRegionByte { 
   tt : list string; (* The types of this region *)
   byte_no : nat; (* What byte is this *)
}.
Definition unknown_region_byte := mkRegionByte nil 0.
Definition rb_implies (known: RegionByte) (expect: RegionByte): bool :=
   (Nat.eqb known.(byte_no) expect.(byte_no)) &&
   (list_implies known.(tt) expect.(tt)).
Definition rbo_implies (known: option RegionByte) (expect: RegionByte): bool :=
   match known with
   | Some rb => rb_implies rb expect
   | None => false
   end.

(* Knowledge of a Memory Region is a Partial Function *)
Record Region := mkRegion {
   known : ZM.t RegionByte;
}.
Definition empty_region := mkRegion ZM.empty.

(* Simplified Memory State assumes that
   1. stack space is sufficient (effectively infinite)
   2. sys_brk always succeeds at acquiring more memory (effectively infinite)
 *)
Record MemoryState := mkMemoryState {
   register_state : Region;
   stack_state : Region;
   frame_state : Region;
   heap_state : Region;
}.
Definition empty_memory_state := mkMemoryState empty_region empty_region empty_region empty_region.

(* An Instruction argument can be part of an opcode *)
Inductive InstructionArgument : Type :=
   | Address : string -> InstructionArgument
   | RawValue : string -> InstructionArgument
   | Register : string -> InstructionArgument.
Definition print_arg (arg: InstructionArgument): string :=
   match arg with
   | Address r => "$" ++ r
   | RawValue r => r
   | Register r => "%" ++ r
   end.

(* An Instruction is defined by its mnemonic and effect *)
Record Instruction := mkInstruction {
   mnemonic : string;
   effect : MemoryState -> MemoryState;
}.
Definition unknown_effect (st: MemoryState): MemoryState :=
   empty_memory_state.

(* A Jmp Instruction *)
Record JmpInstruction := mkJmp {
   dst : string;
   condition : MemoryState;
}.

(* A Data Block is a list of bytes *)
Record BasicData := mkData {
   bytes : list nat;
}.
Definition empty_data := mkData nil.

(* A Basic Block is a list of instructions with no branches *)
Record BasicBlock := mkBasicBlock {
   instructions : list Instruction;
   tail : list JmpInstruction;
}.
Definition empty_block := mkBasicBlock nil nil.

Inductive ProgramSection : Type :=
  | TextSection
  | DataSection.

(* A Control Flow Graph is a set of labelled blocks *)
Record ControlFlowGraph := mkCFG {
   section : ProgramSection;
   current_label : string;
   blocks : list (string * BasicBlock);
   data : list (string * BasicData);
   globals : list string;
}.
Definition empty_control_flow_graph := mkCFG TextSection "_start" nil nil nil.

(* The Type of an unknown RegionByte is Ordinal 0 *)
Definition region_lookup (r: Region)(i: BinInt.Z): RegionByte := 
   match ZM.find i r.(known) with
   | Some x => x
   | None => unknown_region_byte
   end.

(* This is for internal use, it does not directly correspond to the actual instruction *)
Definition push_stack (st: MemoryState)(rb: RegionByte): MemoryState :=
   let new_stack := (ZM.fold (fun k e m => ZM.add (BinInt.Z.add k BinInt.Z.one) e m) st.(stack_state).(known) ZM.empty) in
   let new_stack := mkRegion (ZM.add BinInt.Z.zero rb new_stack) in
   mkMemoryState st.(register_state) new_stack st.(frame_state) st.(heap_state).

(* This is for internal use, it does not directly correspond to the actual instruction *)
Definition pop_stack (st: MemoryState): (MemoryState * RegionByte) :=
   let rb := region_lookup st.(stack_state) (BinInt.Z.zero) in
   let new_stack := mkRegion (ZM.fold (fun k e m => ZM.add (BinInt.Z.sub k (BinInt.Z.one)) e m) st.(stack_state).(known) ZM.empty) in
   let st := mkMemoryState st.(register_state) new_stack st.(frame_state) st.(heap_state) in
   (st , rb).

Check eq_refl : ((pop_stack empty_memory_state) = (_ , {| tt := nil; |})).
Check eq_refl : (pop_stack (push_stack empty_memory_state (mkRegionByte (cons "A" nil) 3)) = (_ , {| tt := (cons "A" nil); byte_no := 3; |})).

(* Check if one memory state is a subset of another memory state *)
Definition mem_is_subset (lo: MemoryState)(hi: MemoryState): bool := 
   let rt_register_state := ZM.fold (fun k e b => b && (rbo_implies (ZM.find k hi.(register_state).(known)) e)) lo.(register_state).(known) true in
   let rt_stack_state := ZM.fold (fun k e b => b && (rbo_implies (ZM.find k hi.(stack_state).(known)) e)) lo.(stack_state).(known) true in
   let rt_frame_state := ZM.fold (fun k e b => b && (rbo_implies (ZM.find k hi.(frame_state).(known)) e)) lo.(frame_state).(known) true in
   let rt_heap_state := ZM.fold (fun k e b => b && (rbo_implies (ZM.find k hi.(heap_state).(known)) e)) lo.(heap_state).(known) true in
   rt_register_state && rt_stack_state && rt_frame_state && rt_heap_state.

Check eq_refl : (mem_is_subset empty_memory_state empty_memory_state) = true.

Definition declare_global (cfg: ControlFlowGraph) (glb: string): ControlFlowGraph :=
   mkCFG cfg.(section) cfg.(current_label) cfg.(blocks) cfg.(data) (cons glb cfg.(globals)).

(* This changes the program section to text mode *)
Definition declare_text (cfg: ControlFlowGraph) := 
   mkCFG TextSection cfg.(current_label) cfg.(blocks) cfg.(data) cfg.(globals).

(* This changes the program section to text mode *)
Definition declare_data (cfg: ControlFlowGraph) := 
   mkCFG DataSection cfg.(current_label) cfg.(blocks) cfg.(data) cfg.(globals).

Definition declare_label (cfg: ControlFlowGraph) (glb: string): ControlFlowGraph :=
   match cfg.(section) with
   | TextSection => mkCFG cfg.(section) glb (cons (glb , empty_block) cfg.(blocks)) cfg.(data) cfg.(globals)
   | DataSection => mkCFG cfg.(section) glb cfg.(blocks) (cons (glb , empty_data) cfg.(data)) cfg.(globals)
   end.

Definition commit_ascii (cfg: ControlFlowGraph) (lit: string): ControlFlowGraph :=
   cfg.

Definition commit_zero (cfg: ControlFlowGraph) (len: nat): ControlFlowGraph :=
   cfg.

Definition register (register_name: string): InstructionArgument :=
   Register register_name.

Definition raw_value (value_name: string): InstructionArgument :=
   RawValue value_name.

Definition address (value_name: string): InstructionArgument :=
   Address value_name.

Definition append_instruction (cfg: ControlFlowGraph) (ins: Instruction): ControlFlowGraph :=
   cfg.

Definition zero_op (cfg: ControlFlowGraph) (ins: string): ControlFlowGraph :=
   let i := match ins with
   | _ => mkInstruction "?" unknown_effect
   end in append_instruction cfg i.

Definition unary_op (cfg: ControlFlowGraph) (ins: string) (arg1: InstructionArgument): ControlFlowGraph :=
   let i := match (ins, arg1) with
   | _ => mkInstruction "?" unknown_effect
   end in append_instruction cfg i.

Definition binary_op (cfg: ControlFlowGraph) (ins: string) (arg1: InstructionArgument) (arg2: InstructionArgument): ControlFlowGraph :=
   let i := match (ins, arg1, arg2) with
   | _ => mkInstruction "?" unknown_effect
   end in append_instruction cfg i.

Definition control_flow_graph_assertions_reachable (cfg: ControlFlowGraph): bool :=
   true.
