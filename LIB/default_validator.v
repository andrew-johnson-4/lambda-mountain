
Require Coq.Numbers.BinNums.
From MMaps Require Import MMaps.
Module ZM := MMaps.RBT.Make(BinInt.Z).

(* Memory Is Denominated in Bytes *)
Record RegionByte := mkRegionByte { 
    tt : nat;      (* The type of this region represented as an Ordinal *)
    tt_byte : nat; (* The type-byte-index of this byte *)
}.

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
Definition pop_stack (st: MemoryState)(tt: nat)(tt_byte: nat): (MemoryState * RegionByte) :=
   let rb := region_lookup st.(stack_state) (BinInt.Z.zero) in
   let new_stack := mkRegion (ZM.fold (fun k e m => ZM.add (BinInt.Z.sub k (BinInt.Z.one)) e m) st.(stack_state).(known) ZM.empty) in
   let st := mkMemoryState st.(register_state) new_stack st.(frame_state) st.(heap_state) in
   (st , rb).


