
From Coq Require Import ZArith String List.
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

(* Memory Is Denominated in Bytes *)
Record RegionByte := mkRegionByte { 
   tt : nat;      (* The type of this region represented as an Ordinal *)
   tt_byte : nat; (* The type-byte-index of this byte *)
}.
Definition unknown_region_byte := mkRegionByte 0 0.
Definition beq_rb (l: RegionByte)(r: RegionByte): bool :=
   (Nat.eqb l.(tt) r.(tt)) && (Nat.eqb l.(tt_byte) r.(tt_byte)).
Definition beqo_rb (l: option RegionByte)(r: RegionByte): bool := match l with
   | Some lo => (Nat.eqb lo.(tt) r.(tt)) && (Nat.eqb lo.(tt_byte) r.(tt_byte))
   | None => false end.

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
   | Register : string -> InstructionArgument.
Definition print_arg (arg: InstructionArgument): string :=
   match arg with
   | Register r => "%" ++ r
   end.

(* An Instruction is defined by its mnemonic and effect *)
Record Instruction := mkInstruction {
   mnemonic : string;
   effect : MemoryState -> MemoryState;
}.

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
Definition push_stack (st: MemoryState)(tt: nat)(tt_byte: nat): MemoryState :=
   let rb := mkRegionByte tt tt_byte in
   let new_stack := (ZM.fold (fun k e m => ZM.add (BinInt.Z.add k BinInt.Z.one) e m) st.(stack_state).(known) ZM.empty) in
   let new_stack := mkRegion (ZM.add BinInt.Z.zero rb new_stack) in
   mkMemoryState st.(register_state) new_stack st.(frame_state) st.(heap_state).

(* This is for internal use, it does not directly correspond to the actual instruction *)
Definition pop_stack (st: MemoryState): (MemoryState * RegionByte) :=
   let rb := region_lookup st.(stack_state) (BinInt.Z.zero) in
   let new_stack := mkRegion (ZM.fold (fun k e m => ZM.add (BinInt.Z.sub k (BinInt.Z.one)) e m) st.(stack_state).(known) ZM.empty) in
   let st := mkMemoryState st.(register_state) new_stack st.(frame_state) st.(heap_state) in
   (st , rb).

Check eq_refl : ((pop_stack empty_memory_state) = (_ , {| tt := 0; tt_byte := 0; |})).
Check eq_refl : (pop_stack (push_stack empty_memory_state 123 456) = (_ , {| tt := 123; tt_byte := 456; |})).

(* Check if one memory state is a subset of another memory state *)
Definition mem_is_subset (lo: MemoryState)(hi: MemoryState): bool := 
   let rt_register_state := ZM.fold (fun k e b => b && (beqo_rb (ZM.find k hi.(register_state).(known)) e)) lo.(register_state).(known) true in
   let rt_stack_state := ZM.fold (fun k e b => b && (beqo_rb (ZM.find k hi.(stack_state).(known)) e)) lo.(stack_state).(known) true in
   let rt_frame_state := ZM.fold (fun k e b => b && (beqo_rb (ZM.find k hi.(frame_state).(known)) e)) lo.(frame_state).(known) true in
   let rt_heap_state := ZM.fold (fun k e b => b && (beqo_rb (ZM.find k hi.(heap_state).(known)) e)) lo.(heap_state).(known) true in
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
