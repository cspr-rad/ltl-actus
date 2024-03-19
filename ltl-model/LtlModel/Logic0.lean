-- A rough draft, closest to the rust implementation

import Lean.Data.HashSet
import LtlModel.LabeledTransitionSystem

class TermSet (T : Type u) extends BEq T, Hashable T

variable [BEq T] [Hashable T] {T : Type} [TermSet T]

inductive Proposition (T : Type) : Type where
  | tt : Proposition T
  | var : T → Proposition T
  -- | cmp : T → T → Proposition T
  | iseq : T → T → Proposition T
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
  -- syntax:60 term "<<" term : term
  syntax:60 term "===" term : term
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
    -- | `($p << $q) => `(TemporalProposition.term (Proposition.cmp $p $q))
    | `($p === $q) => `(TemporalProposition.term (Proposition.iseq $p $q))
    | `($p and $q) => `(TemporalProposition.and $p $q)
    | `(□ $p) => `(TemporalProposition.always $p)
    | `(◇ $p) => `(TemporalProposition.eventually $p)
    | `($p R $q) => `(TemporalProposition.release $p $q)
    | `($p U $q) => `(TemporalProposition.until $p $q)

  def depth_aux (p : Proposition T) : Nat :=
  match p with
  | Proposition.tt => 0
  | Proposition.var _ => 0
  | Proposition.iseq _ _ => 1
  | Proposition.not x => 1 + depth_aux x
  | Proposition.or x y => 1 + depth_aux x + depth_aux y

  def depth (phi : TemporalProposition T) : Nat :=
  match phi with
  | [[p]] => depth_aux p
  | p and q => 1 + depth p + depth q
  | □ p => 1 + depth p
  | ◇ p => 1 + depth p
  | p R q => 1 + depth p + depth q
  | p U q => 1 + depth p + depth q

end TemporalLogic

namespace Semantics
  open TemporalLogic
  variable (T : Type) [TermSet T]

  def Sigma : Type := List (Lean.HashSet T)

  def satisfaction (sigma : Sigma T) (phi : TemporalProposition T) : Prop :=
  match phi with
  | [[p]] => match p with
    | Proposition.tt => true
    | Proposition.var p => (sigma.get? 0 >>= fun a => pure (a.contains p)).getD false
    | Proposition.iseq p q => (sigma.get? 0 >>= fun _ => pure (p = q)).getD false
    | Proposition.not p => ¬ satisfaction sigma [[p]]
    | Proposition.or p q => satisfaction sigma [[p]] ∨ satisfaction sigma [[q]]
  | p and q => satisfaction sigma p ∧ satisfaction sigma q
  | □ p => ∀ (i : Nat), satisfaction (sigma.drop i) p
  | ◇ p => ∃ (i : Nat), satisfaction (sigma.drop i) p
  | p R q => ∃ (i : Nat), satisfaction (sigma.drop i) q ∧ ∀ (j : Nat), j < i → satisfaction (sigma.drop j) p
  | p U q => ∀ (i : Nat), satisfaction (sigma.drop i) q ∧ ∃ (j : Nat), j < i → satisfaction (sigma.drop j) p

  def words (phi : TemporalProposition T) (sigma : Sigma T) : Prop := satisfaction T sigma phi

end Semantics
