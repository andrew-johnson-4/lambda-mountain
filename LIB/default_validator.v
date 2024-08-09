
Require Import Coq.Numbers.BinNums.

Structure RegionSlot := { 
    tt : N;      (* The type of this region represented as an Ordinal *)
    tt_byte : N; (* The type-byte-index of this slot *)
}.

Structure Region := {
    known : Z -> RegionSlot;
}.

