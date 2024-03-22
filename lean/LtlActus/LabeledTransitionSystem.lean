import Lean.Data.HashSet
import LtlActus.Types

structure LTS where
  state : Type
  action : Type
  initial : List state
  transition : state -> action -> state -> Prop
  atomic_proposition : Type
  ap_termset : TermSet atomic_proposition
  labelling : state -> Lean.HashSet atomic_proposition
