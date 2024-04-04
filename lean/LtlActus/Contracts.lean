import LtlActus.Types
import LtlActus.Logic

namespace PAM

  structure Terms where
    principal : Int -- Money
    interest_rate : Int -- Scalar
    maturity : UInt64 -- Timestamp
    deriving BEq, Hashable, Repr

  inductive Event :=
  | Maturity : Event
  | PrincipalRepayment : Event
  | InterestPayment : Event
    deriving BEq, Hashable, Repr

  inductive T :=
  | ts : Terms -> T
  | e : Event -> T

  def Contract := LinearTemporalLogic.LTL T

  /-! some properties of a pam execution:
    * the principal is repaid at maturity
    * the interest is paid at the end of each month
    * the principal is repaid at the end of the last month
  !-/
  def pam_init : Contract :=
    □ [[(T.ts { principal := 1000, interest_rate := 10, maturity := 12 })]]

end PAM

namespace SWAPS

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

  def Contract := LinearTemporalLogic.LTL T

end SWAPS
