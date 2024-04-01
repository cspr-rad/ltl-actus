import LtlActus.Types
import LtlActus.Logic

namespace Pam

  structure Terms where
    principal : Money
    interest_rate : Scalar
    months : TimeDelta
    deriving BEq, Hashable, Repr

  inductive Event :=
  | Maturity : Event
  | PrincipalRepayment : Event
  | InterestPayment : Event
    deriving BEq, Hashable, Repr

  inductive T :=
  | ts : Terms -> T
  | e : Event -> T

end Pam

namespace Swap

  structure Terms where
    notional : Int
    fixed_rate : Int
    floating_rate : Int
    maturity : UInt64
    deriving Repr, BEq, Hashable

  inductive Event :=
  | Maturity : Event
  | FixedLegPayment : Event
  | FloatingLegPayment : Event

  inductive T :=
  | ts : Terms -> T
  | e : Event -> T

end Swap
