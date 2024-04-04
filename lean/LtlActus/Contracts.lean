import LtlActus.Types
import LtlActus.Logic

structure tms where
  principal : Money
  interest_rate : Scalar
  deriving BEq, Hashable, Repr

/-! # ACTUS Contracts -/
/- A contract is of signaturea -/
/- * (Terms Event : Type) -/
/- * (T : Type) which wraps the first two.-/
/- * A linear temporal logic formula over atomic propositions T -/
namespace PAM

  structure Terms where
    principal : Money -- Money
    interest_rate : Scalar -- Scalar
    maturity : Timestamp -- Timestamp
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
    □ [[T.ts { principal := { amount := 1000 }, interest_rate := { value := 10 }, maturity := Timestamp.t 12 }]]
  def pam_monthly : Contract :=
    ◯ [[T.e Event.InterestPayment]]

end PAM

namespace SWAPS

  structure Terms where
    notional : Int
    fixed_rate : Int
    floating_rate : Int
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
