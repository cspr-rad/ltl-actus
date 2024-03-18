import LtlModel.LabeledTransitionSystem

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

  syntax "[[" term "]]" : term
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
    | `([[$p]]) => `(TemporalProposition.term $p)
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

namespace WordSemantics
  open TemporalLogic
-- def satisfies (lts : LTS) (phi : TemporalProposition lts.atomic_proposition): Prop :=
-- match phi with
-- | [[x]] => match x with
--   | Proposition.tt => true
--   | Proposition.var x => x ∈ lts.state
--   | Proposition.cmp x y => exists (a : lts.action), lts.transition x a y
--   | Proposition.not x => ¬ (satisfies lts (TemporalLogic.term x))
--   | Proposition.or x y => satisfies lts (TemporalLogic.term x) ∨ satisfies lts (TemporalLogic.term y)
-- | x and y => satisfies lts x ∧ satisfies lts y
end WordSemantics

namespace PathSemantics
  open TemporalLogic
  variable (T : Type) [TermSet T]

  def Sigma : Type := List (List T)
  #check List.head [1,2,3]
  def satisfaction (sigma : Sigma T) (phi : TemporalProposition T) : Prop :=
  match phi with
  | [[p]] => match p with
    | Proposition.tt => true
    | Proposition.var p => (sigma.get? 0 >>= fun a => pure (p ∈ a)).getD false
    | Proposition.cmp p q => ∃ (a : T), a ∈ sigma.head ∧ satisfaction sigma.tail [[Proposition.var a]]
    | Proposition.not p => ¬ satisfaction sigma (TemporalProposition.term p)
    | Proposition.or p q => satisfaction sigma (TemporalProposition.term p) ∨ satisfaction sigma (TemporalProposition.term q)

end PathSemantics
