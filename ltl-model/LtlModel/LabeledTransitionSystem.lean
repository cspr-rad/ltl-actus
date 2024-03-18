-- the typechecker does not enforce duplicate removal, with lists. they should be sets.
structure LTS where
  state : Type
  action : Type
  initial : List state
  transition : state -> action -> state -> Prop
  atomic_proposition : Type
  labelling : state -> List atomic_proposition
