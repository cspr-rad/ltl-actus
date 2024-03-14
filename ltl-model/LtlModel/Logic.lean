class TermSet (T : Type u) extends BEq T

variable [BEq T] (T : Type) [TermSet T]

inductive Proposition (T : Type) : Type where
  | tt : Proposition T
  | var : T → Proposition T
  | cmp : T → T → Proposition T
  | not : Proposition T → Proposition T
  | or : Proposition T → Proposition T → Proposition T
deriving BEq, Hashable

inductive TemporalProposition (T : Type) : Type where
  | term : Proposition T → TemporalProposition T
  | and : TemporalProposition T → TemporalProposition T → TemporalProposition T
  | always : TemporalProposition T → TemporalProposition T
  | eventually : TemporalProposition T → TemporalProposition T
  | release : TemporalProposition T → TemporalProposition T → TemporalProposition T
  | until : TemporalProposition T → TemporalProposition T → TemporalProposition T
deriving BEq, Hashable

-- def unlift (p : TemporalProposition T) : option (Proposition T) :=
--   match p with
--   | TemporalProposition.term p => some p
--   | TemporalProposition.always p => unlift p
--   | TemporalProposition.eventually p => unlift p
--   | _ => none

namespace TemporalLogic

  syntax "tt" : term
  syntax "ff" : term
  syntax "~" term : term
  syntax:60 term "or" term : term
  syntax:60 term "<<" term : term
  syntax:60 term "and" term : term
  syntax "□" term : term
  syntax "◇" term : term
  syntax:60 term "R" term : term
  syntax:60 term "U" term : term

  open Proposition
  open TemporalProposition

  macro_rules
    | `(tt) => `(TemporalProposition.term (Proposition.tt))
    | `(ff) => `(TemporalProposition.term (Proposition.not (Proposition.tt)))
    | `(~ $p) => `(TemporalProposition.term (Proposition.not $p))
    | `($p or $q) => `(TemporalProposition.term (Proposition.or $p $q))
    | `($p << $q) => `(TemporalProposition.term (Proposition.cmp $p $q))
    | `($p and $q) => `(TemporalProposition.and $p $q)
    | `(□ $p) => `(TemporalProposition.always $p)
    | `(◇ $p) => `(TemporalProposition.eventually $p)
    | `($p R $q) => `(TemporalProposition.release $p $q)
    | `($p U $q) => `(TemporalProposition.until $p $q)

end TemporalLogic
