
Require Import Coq.Numbers.BinNums.

Structure RegionSlot := { 
    tt : N;      (* The type of this region represented as an Ordinal *)
    tt_byte : N; (* The type-byte-index of this slot *)
}.

Structure Region := {
    known : Z -> RegionSlot;
}.

(* Simplified Memory State assumes that
   1. stack space is sufficient (effectively infinite)
   2. sys_brk always succeeds at acquiring more memory (effectively infinite)
 *)
Structure MemoryState := {
    stack_state : Region;
    heap_state : Region;
    heap_tail : N;
}.
