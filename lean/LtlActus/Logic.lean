import Lean.Data.HashSet
import LtlActus.Types

namespace LinearTemporalLogic
  variable {T : Type} [TermSet T]

  inductive LTL (T : Type) : Type where
  | ltt : LTL T
  | atom : T → LTL T
  | negate : LTL T → LTL T
  | conjunct : LTL T → LTL T → LTL T
  | next : LTL T → LTL T
  | until : LTL T → LTL T → LTL T
  deriving Hashable

  instance : Inhabited (LTL T) where
    default := LTL.negate LTL.ltt

  notation "[[" x "]]" => LTL.atom x
  notation "~" φ => LTL.negate φ
  notation φ "and" ψ => LTL.conjunct φ ψ
  notation "◯" φ => LTL.next φ
  notation φ "U" ψ => LTL.until φ ψ
  notation "ltt" => LTL.ltt

  def eventually (φ : LTL T) : LTL T := ltt U φ
  def always (φ : LTL T) : LTL T := ~ (eventually (~ φ))
  def or (φ ψ : LTL T) : LTL T := ~ (~ φ and ~ ψ)
  def implies (φ ψ : LTL T) : LTL T := or (~ φ) ψ
  def release (φ ψ : LTL T) : LTL T := ~ ((~ φ) U (~ ψ))

  notation "◇" φ => eventually φ
  notation "□" φ => always φ
  notation φ "or" ψ => or φ ψ
  notation φ "implies" ψ => implies φ ψ
  notation φ "R" ψ => release φ ψ

end LinearTemporalLogic

namespace LTLSemantics
  open LinearTemporalLogic
  variable (T : Type) [TermSet T]

  def Sigma : Type := List (Lean.HashSet T)

  def satisfaction (σ: Sigma T) (φ : LTL T) : Prop :=
  match φ with
  | ltt => true
  | [[x]] => (σ.get? 0 >>= fun a => pure (a.contains x)).getD false
  | ~ p => ¬ (satisfaction σ p)
  | p and q => satisfaction σ p ∧ satisfaction σ q
  | ◯ p => satisfaction (σ.drop 0) p
  | p U q => ∃ (j : Nat), satisfaction (σ.drop j) q ∧ (∀ (i : Nat), i < j -> satisfaction (σ.drop i) p)

  notation:70 σ "⊨" φ => satisfaction σ φ

end LTLSemantics
