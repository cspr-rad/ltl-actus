/// Copied from Avi's `actus-rs` project.
use std::fmt::Display;

use prusti_contracts::*;
use serde::{Deserialize, Serialize};

/// Different types of events.
/// Different types have their own business logic in terms of payoff and state transition functions.
///
/// The order of events occurring at the same time is defined by the ordering of the enum variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum EventType {
    /// AD: Monitoring of contract. Evaluates all contract states.
    #[serde(rename = "AD")]
    Monitoring,

    /// IED: Scheduled date of initial exchange of e.g. principal value in fixed income products.
    #[serde(rename = "IED")]
    InitialExchange,

    /// FP: Scheduled fee payments.
    #[serde(rename = "FP")]
    FeePayment,

    /// PR: Scheduled principal redemption payment.
    #[serde(rename = "PR")]
    PrincipalRedemption,

    /// PD: Drawing of principal amount e.g. in a credit line.
    #[serde(rename = "PD")]
    PrincipalDrawing,

    /// PRF: Scheduled fixing of principal payment amount.
    #[serde(rename = "PRF")]
    PrincipalPaymentAmountFixing,

    /// PY: FIXME: the actus tests don't contain this event.
    /// Scheduled payment of a penalty.
    #[serde(rename = "PY")]
    PenalytPayment,

    /// PP: Unscheduled early repayment of principal.
    #[serde(rename = "PP")]
    PrincipalPrepayment,

    /// IP: Scheduled interest payment.
    #[serde(rename = "IP")]
    InterestPayment,

    /// IPCI: Scheduled capitalization of accrued interest.
    #[serde(rename = "IPCI")]
    InterestCapitalization,

    /// CE: Credit event of counterparty to a contract.
    #[serde(rename = "CE")]
    CreditEvent,

    /// RRF: Scheduled fixing of variable rate with known new rate.
    #[serde(rename = "RRF")]
    RateResetFixed,

    /// RR: Scheduled fixing of variable rate with unknown new rate.
    #[serde(rename = "RR")]
    RateResetVariable,

    /// DV: Payment of dividends.
    #[serde(rename = "DV")]
    DividendPayment,

    /// PRD: Purchase of a contract.
    #[serde(rename = "PRD")]
    Purchase,

    /// MR: Scheduled margin call.
    #[serde(rename = "MR")]
    MarginCall,

    /// TD: Termination of a contract.
    #[serde(rename = "TD")]
    Termination,

    /// SC: Scheduled fixing of a scaling index.
    #[serde(rename = "SC")]
    ScalingIndexFixing,

    /// IPCB: Scheduled fixing of the interest calculation base.
    #[serde(rename = "IPCB")]
    InterestCalculationBaseFixing,

    /// MD: Maturity of a contract.
    #[serde(rename = "MD")]
    Maturity,

    /// XD: Exercise of a contractual feature such as an optionality.
    #[serde(rename = "XD")]
    Exercise,

    /// STD: Settlement of an exercised contractual claim.
    #[serde(rename = "STD")]
    Settlement,

    /// IPFX: TODO: Figure out what the hell IPFX stands for?
    #[serde(rename = "IPFX")]
    IPFX,

    /// IPFL: TODO: Figure out what the hell IPFL stands for?
    #[serde(rename = "IPFL")]
    IPFL,

    /// PI: TODO: Figure out what the hell IP stands for?
    #[serde(rename = "PI")]
    PI,
}

impl Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                EventType::Monitoring => "Monitoring",
                EventType::InitialExchange => "InitialExchange",
                EventType::FeePayment => "FeePayment",
                EventType::PrincipalRedemption => "PrincipalRedemption",
                EventType::PrincipalDrawing => "PrincipalDrawing",
                EventType::PrincipalPaymentAmountFixing => "PrincipalPaymentAmountFixing",
                EventType::PenalytPayment => "PenaltyPayment",
                EventType::PrincipalPrepayment => "PrincipalPrepayment",
                EventType::InterestPayment => "InterestPayment",
                EventType::InterestCapitalization => "InterestCapitalization",
                EventType::CreditEvent => "CreditEvent",
                EventType::RateResetFixed => "RateResetFixed",
                EventType::RateResetVariable => "RateResetVariable",
                EventType::DividendPayment => "DividendPayment",
                EventType::Purchase => "Purchase",
                EventType::MarginCall => "MarginCall",
                EventType::Termination => "Termination",
                EventType::ScalingIndexFixing => "ScalingIndexFixing",
                EventType::InterestCalculationBaseFixing => "InterestCalculationBaseFixing",
                EventType::Maturity => "Maturity",
                EventType::Exercise => "Exercise",
                EventType::Settlement => "Settlement",
                EventType::IPFX => "IPFX",
                EventType::IPFL => "IPFL",
                EventType::PI => "PI",
            },
        )
    }
}
