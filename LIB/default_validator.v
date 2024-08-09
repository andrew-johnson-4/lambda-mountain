
Require Import Coq.Numbers.BinNums.
Require Import MMaps.

Module ZM := MMaps.RBT.Make(Z).

Structure RegionSlot := { 
    tt : N;      (* The type of this region *)
    tt_byte : N; (* The type-byte of this slot *)
}.

Structure StackRegion := {
    known : ZM;
}.
