import LtlModel.Time

inductive Proposition (T : Type) : Type where
  | var : T → Proposition T
  | cmp : T → T → Proposition T
  | not : Proposition T → Proposition T
  | or : Proposition T → Proposition T → Proposition T

inductive TemporalProposition (T : Type)
  | term : Proposition T → TemporalProposition T
  | and : TemporalProposition T → TemporalProposition T → TemporalProposition T
  | always : TemporalProposition T → TemporalProposition T
  | eventually : TemporalProposition T → TemporalProposition T
  | release : TemporalProposition T → TemporalProposition T → TemporalProposition T
  | until : TemporalProposition T → TemporalProposition T → TemporalProposition T

structure Timestamp where
  time : Int64

namespace Timestamp

open IO

-- def new (optTime : Option Int64) : IO Timestamp :=
--   match optTime with
--   | some t => pure { time := t }
--   | none => do
--     t <- currentTime
--     pure { time := t }

end Timestamp

#check id

def hello := "world"
