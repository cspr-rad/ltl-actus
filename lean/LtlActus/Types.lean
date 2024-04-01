class TermSet (T : Type) extends BEq T, Hashable T

structure Money where
  amount : Int
  deriving BEq, Hashable, Repr

structure Scalar where
  value : Int
  deriving BEq, Hashable, Repr

structure Timestamp where
  time : UInt64
  deriving BEq, Hashable, Repr

structure TimeDelta where
  dt : UInt64
  deriving BEq, Hashable, Repr
