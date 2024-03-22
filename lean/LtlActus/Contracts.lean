import LtlActus.Types
import LtlActus.Logic

namespace Pam

  structure Terms where
    principal : Money
    interest_rate : Float64
    months : UInt

  inductive Event :=
  | Maturity : Event
  | PrincipalRepayment : Event
  | InterestPayment : Event

end Pam

namespace Swap

  structure Terms where
    notional : Money
    fixed_rate : Float64
    floating_rate : Float64
    maturity : UInt

  inductive Event :=
  | Maturity : Event
  | FixedLegPayment : Event
  | FloatingLegPayment : Event

end Swap
