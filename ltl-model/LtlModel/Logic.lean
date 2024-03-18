import Init.Data.String.Basic

-- LTL formula datatype
inductive LTL (α : Type)
| tt                      -- true
| ff                      -- false
| atom (a : α)            -- atomic proposition
| not (φ : LTL α)         -- negation
| and (φ₁ φ₂ : LTL α)     -- conjunction
| or (φ₁ φ₂ : LTL α)      -- disjunction
| next (φ : LTL α)        -- next
| until (φ₁ φ₂ : LTL α)   -- until
| release (φ₁ φ₂ : LTL α) -- release

-- Infinite word type
def Word (α : Type) := Nat → α

-- LTL semantics
def satisfies {α : Type} (w : Word α) : LTL α → Prop
| LTL.tt           => True
| LTL.ff           => False
| (LTL.atom a)     => w 0 = a
| (LTL.not φ)      => ¬ satisfies w φ
| (LTL.and φ₁ φ₂)  => satisfies w φ₁ ∧ satisfies w φ₂
| (LTL.or φ₁ φ₂)   => satisfies w φ₁ ∨ satisfies w φ₂
| (LTL.next φ)     => satisfies (λ n => w (n+1)) φ
| (LTL.until φ₁ φ₂) => ∃ i, satisfies (λ n => w (n+i)) φ₂ ∧ ∀ j, j < i -> satisfies (λ n => w (n+j)) φ₁
| (LTL.release φ₁ φ₂) => ∀ i, satisfies (λ n => w (n+i)) φ₂ ∨ ∃ j, j < i -> satisfies (λ n => w (n+j)) φ₁

notation:50 w " ⊨ " φ => satisfies w φ

-- Example usage
def example1 : Word Nat := λ n => if n % 2 = 0 then 0 else 1

/- these examples won't work because not Decidabl propositions
#eval example1 ⊨ (LTL.until (LTL.atom 1) (LTL.atom 0)) -- true
#eval example1 ⊨ (LTL.release (LTL.atom 0) (LTL.next (LTL.atom 1))) -- true
-/

-- Define the transition system
def S : Type := -- Define the set of states
def → : S → S → Prop := -- Define the transition relation
def s₀ : S := -- Define the initial state (optional)

-- Define the set of observable events or actions
def Σ : Type := -- Define the set of observable events or actions

-- Check if a word is a valid path in the TS
def is_valid_path (w : Nat → S) : Prop :=
  -- Check if consecutive states are connected by valid transitions
  -- and optionally check if the word starts from s₀

-- Extract the trace from a valid path
def trace_of_path (w : Nat → S) (h : is_valid_path w) : List Σ :=
  -- Map each transition in the path to its corresponding observable event or action

-- Filter words to obtain valid paths
def valid_paths (words : List (Nat → S)) : List (Nat → S) :=
  words.filter is_valid_path

-- Extract traces from the valid paths
def traces_of_words (words : List (Nat → S)) : List (List Σ) :=
  valid_paths words |>.map (λ w => trace_of_path w (is_valid_path_of_valid_path w))
