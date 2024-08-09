
Require Import Coq.Numbers.BinNums.

Structure RegionSlot := { 
    tt : N;      (* The type of this region *)
    tt_byte : N; (* The type-byte of this slot *)
}.

Structure StackRegion := {
    known : Z -> RegionSlot;
}.
