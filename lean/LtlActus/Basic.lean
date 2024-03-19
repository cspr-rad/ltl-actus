import LtlActus.TimeFFI
import LtlActus.Logic
import Lean.Data.AssocList

namespace Time
  inductive Timestamp : Type where
    | time : UInt64 → Timestamp
    | infinity : Timestamp
  structure IntervalTrue where
    start_t : Timestamp
    end_t : Timestamp

  open Timestamp (time infinity)

  def contains (interval : IntervalTrue) (timestamp : Timestamp) : Bool :=
    match timestamp, interval.start_t, interval.end_t with
    | infinity, _, infinity => true
    | infinity, _, _ => false
    | time _, infinity, infinity => true
    | time x, time s, infinity => s <= x
    | time x, infinity, time e => x <= e
    | time x, time s, time e => s <= x && x <= e

end Time

-- namespace State
--   open Time
--   variable (T : Type) [TermSet T]
--   def StateStore : Type := Lean.AssocList (Proposition T) (List IntervalTrue)
--
--   def isTrueAt (state : StateStore T) (p : Proposition T) (t : Timestamp) : Bool :=
--     match state.find? p with
--     | some intervals => intervals.any (fun interval => Time.contains interval t)
--     | none => false
-- end State

variable (T : Type)
open Time
open LinearTemporalLogic

def hello := "world"
