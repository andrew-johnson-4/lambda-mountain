
Require Import Coq.Numbers.BinNums.
From MMaps Require Import MMaps.
Module ZM := MMaps.RBT.Make(BinInt.Z).

(* Memory Is Denominated in Bytes *)
Record RegionByte := mkRegionByte { 
    tt : nat;      (* The type of this region represented as an Ordinal *)
    tt_byte : nat; (* The type-byte-index of this byte *)
}.

(* Knowledge of a Memory Region is a Partial Function *)
Record Region := mkRegion {
    known : ZM.t BinInt.Z;
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

Definition push_stack (st: MemoryState)(tt: N)(tt_byte: N): MemoryState :=
   st.

Definition initial_memory_state := mkMemoryState (mkRegion ZM.empty) (mkRegion ZM.empty) (mkRegion ZM.empty) (mkRegion ZM.empty).
