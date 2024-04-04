class TermSet (T : Type) extends BEq T, Hashable T

structure Party where
  name : String
  balance : Int -- Money
  deriving BEq, Hashable, Repr

def Execution : Type -> Type := IO deriving Functor, Applicative, Monad

structure Money where
  amount : Int
  deriving BEq, Hashable, Repr

def Money.map (f : Int -> Int) (m : Money) : Money :=
  { amount := f m.amount }

structure Scalar where
  value : Int
  deriving BEq, Hashable, Repr

def Scalar.map (f : Int -> Int) (s : Scalar) : Scalar :=
  { value := f s.value }

inductive Timestamp : Type where
  | t : UInt64 -> Timestamp
  | infinity : Timestamp
  deriving BEq, Hashable, Repr

def Timestamp.map (f : UInt64 -> UInt64) (t : Timestamp) : Timestamp :=
  match t with
  | Timestamp.t time => Timestamp.t (f time)
  | Timestamp.infinity => Timestamp.infinity

structure TimeDelta where
  dt : UInt64
  deriving BEq, Hashable, Repr

def TimeDelta.map (f : UInt64 -> UInt64) (td : TimeDelta) : TimeDelta :=
  { dt := f td.dt }
