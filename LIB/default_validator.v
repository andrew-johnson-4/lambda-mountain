
From Coq Require Import ZArith String List.
From MMaps Require Import MMaps.
Open Scope string.
Open Scope Z.

Module ZM := MMaps.RBT.Make(BinInt.Z).

(* Memory Is Denominated in Bytes *)
Record RegionByte := mkRegionByte { 
   tt : nat;      (* The type of this region represented as an Ordinal *)
   tt_byte : nat; (* The type-byte-index of this byte *)
}.
Definition beq_rb (l: RegionByte)(r: RegionByte): bool :=
   (Nat.eqb l.(tt) r.(tt)) && (Nat.eqb l.(tt_byte) r.(tt_byte)).
Definition beqo_rb (l: option RegionByte)(r: RegionByte): bool := match l with
   | Some lo => (Nat.eqb lo.(tt) r.(tt)) && (Nat.eqb lo.(tt_byte) r.(tt_byte))
   | None => false end.

(* Knowledge of a Memory Region is a Partial Function *)
Record Region := mkRegion {
   known : ZM.t RegionByte;
}.

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

(* An Instruction is defined by its mnemonic and effect *)
Record Instruction := mkInstruction {
   mnemonic : string;
   effect : MemoryState -> MemoryState;
}.

(* A Basic Block is a list of instructions with no branches *)
Record BasicBlock := mkBasicBlock {
   instructions : list Instruction;
}.

(* A Jmp Instruction *)
Record JmpInstruction := mkJmp {
   dst : string;
   condition : MemoryState;
}.

(* A Control Flow Graph is a list of basic blocks with transitions *)
Record ControlFlowGraph := mkCFG {
   blocks : ZM.t BasicBlock;
   labels : ZM.t string;
   transitions : list JmpInstruction;
}.

(* The Type of an unknown RegionByte is Ordinal 0 *)
Definition region_lookup (r: Region)(i: BinInt.Z): RegionByte := 
   match ZM.find i r.(known) with
   | Some x => x
   | None => mkRegionByte 0 0
   end.

(* Initially nothing is known about the memory state *)
Definition initial_memory_state := mkMemoryState (mkRegion ZM.empty) (mkRegion ZM.empty) (mkRegion ZM.empty) (mkRegion ZM.empty).

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

Check eq_refl : ((pop_stack initial_memory_state) = (_ , {| tt := 0; tt_byte := 0; |})).
Check eq_refl : (pop_stack (push_stack initial_memory_state 123 456) = (_ , {| tt := 123; tt_byte := 456; |})).

(* Check if one memory state is a subset of another memory state *)
Definition mem_is_subset (lo: MemoryState)(hi: MemoryState): bool := 
   let rt_register_state := ZM.fold (fun k e b => b && (beqo_rb (ZM.find k hi.(register_state).(known)) e)) lo.(register_state).(known) true in
   let rt_stack_state := ZM.fold (fun k e b => b && (beqo_rb (ZM.find k hi.(stack_state).(known)) e)) lo.(stack_state).(known) true in
   let rt_frame_state := ZM.fold (fun k e b => b && (beqo_rb (ZM.find k hi.(frame_state).(known)) e)) lo.(frame_state).(known) true in
   let rt_heap_state := ZM.fold (fun k e b => b && (beqo_rb (ZM.find k hi.(heap_state).(known)) e)) lo.(heap_state).(known) true in
   rt_register_state && rt_stack_state && rt_frame_state && rt_heap_state.

Check eq_refl : (mem_is_subset initial_memory_state initial_memory_state) = true.

