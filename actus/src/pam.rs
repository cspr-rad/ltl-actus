extern crate ltl;

use fpdec::{Dec, Decimal};
use serde::Deserialize;
// use time::{Duration, PrimitiveDateTime};

use ltl::*;

use crate::event_types::EventType;
use crate::execution::*;

#[derive(Debug, Clone, Copy, Deserialize, Hash)]
pub struct PamTerms {
    principal: Decimal,
    interest_rate: Decimal,
    months: usize,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Deserialize, Hash)]
pub enum PamEvent {
    Maturity,
    PrincipalRepayment,
    InterestPayment,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Deserialize, Hash)]
pub struct PamState {
    repayment: Decimal,
    timestamp: Timestamp,
}

impl PamState {
    fn new(repayment: Decimal, timestamp: Timestamp) -> Self {
        PamState {
            repayment,
            timestamp,
        }
    }
}

impl Ord for PamTerms {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.principal.cmp(&other.principal)
    }
}

impl PartialOrd for PamTerms {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for PamTerms {}

impl PartialEq for PamTerms {
    fn eq(&self, other: &Self) -> bool {
        self.principal == other.principal
            && self.interest_rate == other.interest_rate
            && self.months == other.months
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Deserialize, Hash)]
pub enum Pam {
    Terms(PamTerms),
    Event(PamEvent),
    State(PamState),
}

impl TermSet for Pam {}

// [derive(Debug, Clone, Eq, PartialEq, Deserialize)]
// ub struct PamEvent {
//    payment: Decimal,
//    timestamp: Timestamp,
//
//
// mpl PamEvent {
//    fn new(payment: Decimal, timestamp: Timestamp) -> Self {
//        PamEvent { payment, timestamp }
//    }
//
//
// mpl Ord for PamEvent {
//    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//        self.timestamp.cmp(&other.timestamp)
//    }
//

// impl PartialOrd for PamEvent {
//    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//        Some(self.cmp(other))
//     }
// }

// impl Event for PamEvent {
//    fn timestamp(&self) -> Timestamp {
//        self.timestamp
//    }

//    fn action(&self) -> Self {
//        PamEvent {
//            payment: self.payment,
//            timestamp: self.timestamp,
//        }
//    }
//}

impl PamTerms {
    fn new(principal: Decimal, interest_rate: Decimal, months: usize) -> Self {
        PamTerms {
            principal,
            interest_rate,
            months,
        }
    }

    fn interest_payment(&self) -> Decimal {
        self.interest_rate / Decimal::from(12) * self.principal
    }

    fn final_payment(&self) -> Decimal {
        self.principal
    }
}

impl Pam {
    fn new(principal: Decimal, interest_rate: Decimal, months: usize) -> Self {
        Pam::Terms(PamTerms::new(principal, interest_rate, months))
    }

    // I want mutable Event and State, but immutable Terms. the getters and setters for that are:
    pub fn terms(&self) -> &PamTerms {
        match self {
            Pam::Terms(terms) => terms,
            _ => panic!("Pam is not a Terms"),
        }
    }

    pub fn state(&self) -> &PamState {
        match self {
            Pam::State(state) => state,
            _ => panic!("Pam is not a State"),
        }
    }

    pub fn update_state(&mut self, state: PamState) {
        match self {
            Pam::State(s) => *s = state,
            _ => panic!("Pam is not a State"),
        }
    }
}

type PamProp = TemporalProp<Pam>;

fn contract(principal: Decimal, interest_rate: Decimal, months: usize) -> PamProp {
    let t: PamTerms = PamTerms::new(principal, interest_rate, months);
    let term_set = TemporalProp::Always(Box::new(TemporalProp::Term(Prop::Var(Pam::Terms(t)))));
    let total_repayment = principal * (Dec!(1) + interest_rate / Dec!(12) * Dec!(24));
    let repayment_final = TemporalProp::Eventually(Box::new(TemporalProp::Term(Prop::Var(
        Pam::State(PamState::new(total_repayment, Timestamp::new(None))),
    ))));
    and(&term_set, &repayment_final)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pam() {
        let principal = Dec!(1000);
        let interest_rate = Dec!(0.05);
        let months = 24;
        let terms = PamTerms::new(principal, interest_rate, months);
        let contract = contract(principal, interest_rate, months);
        let mut state = StateStore::new();
        let always_interval = TrueWhen::new(Timestamp::new(None), None);
        state.add_state(Prop::Var(Pam::Terms(terms)), always_interval);
        let executed = contract.exec_t(&state, &Timestamp::new(None));
        assert!(executed);
    }
}
