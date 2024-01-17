extern crate ltl;

use fpdec::{Dec, Decimal};
use serde::Deserialize;

use ltl::logic::{Prop, TemporalProp};
use ltl::types::TermSet;
use ltl::*;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Hash)]
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
    total_repayment: Decimal,
}

impl PamState {
    fn new(total_repayment: Decimal) -> Self {
        PamState { total_repayment }
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

#[derive(Debug, Clone, Copy, Eq, PartialEq, Deserialize, Hash)]
pub enum Pam {
    Terms(PamTerms),
    Event(PamEvent),
    State(PamState),
}

impl TermSet for Pam {}

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
    let term_set = always(TemporalProp::Term(Prop::Var(Pam::Terms(t))));
    let total_repayment = principal * (Dec!(1) + interest_rate / Dec!(12) * Dec!(24));
    let total_repayment_final = eventually(TemporalProp::Term(Prop::Var(Pam::State(
        PamState::new(total_repayment),
    ))));
    and(&term_set, &total_repayment_final)
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::{debug, LevelFilter};
    use simplelog::{Config, WriteLogger};
    use std::fs::File;
    use std::sync::Once;
    use time::{Duration, OffsetDateTime};

    static INIT: Once = Once::new();

    fn setup() {
        INIT.call_once(|| {
            let _ = WriteLogger::init(
                LevelFilter::Debug,
                Config::default(),
                File::create("./../execution_trace.log").unwrap(),
            );
        });
    }

    // A valid execution trace
    #[test]
    fn test_pam_positive() {
        setup();
        debug!("______________________test_pam_positive______________________");
        let principal = Dec!(1000);
        let interest_rate = Dec!(0.05);
        let months = 24;
        let terms = PamTerms::new(principal, interest_rate, months);
        let contract = contract(principal, interest_rate, months);
        println!("contract: {}", contract);
        let mut state = StateStore::new();
        let always_interval = TrueWhen::new(Timestamp::new(None), None);
        let end_contract_stamp = OffsetDateTime::now_utc() + Duration::weeks(4 * months as i64);
        let active_after_end_contract_interval =
            TrueWhen::new(Timestamp::new(Some(end_contract_stamp)), None);
        state.add_state(Prop::Var(Pam::Terms(terms)), always_interval);
        let total_repayment = principal * (Dec!(1) + interest_rate / Dec!(12) * Dec!(24));
        state.add_state(
            Prop::Var(Pam::State(PamState::new(total_repayment))),
            active_after_end_contract_interval,
        );
        let executed = contract.check(&state, &Timestamp::new(Some(end_contract_stamp)));
        assert!(executed);
    }

    // An invalid execution trace
    #[test]
    fn test_pam_negative() {
        setup();
        debug!("______________________test_pam_negative______________________");
        let principal = Dec!(1000);
        let interest_rate = Dec!(0.05);
        let months = 24;
        let terms = PamTerms::new(principal, interest_rate, months);
        let contract = contract(principal, interest_rate, months);
        println!("contract: {}", contract);
        let mut state = StateStore::new();
        let always_interval = TrueWhen::new(Timestamp::new(None), None);
        let before_contract_stamp = OffsetDateTime::now_utc() - Duration::weeks(2 * months as i64);
        let start_contract_stamp = OffsetDateTime::now_utc();
        let pre_contract_interval = TrueWhen::new(
            Timestamp::new(Some(before_contract_stamp)),
            Some(Timestamp::new(Some(start_contract_stamp))),
        );
        state.add_state(Prop::Var(Pam::Terms(terms)), always_interval);
        let total_repayment = principal * (Dec!(1) + interest_rate / Dec!(12) * Dec!(24));
        state.add_state(
            Prop::Var(Pam::State(PamState::new(total_repayment))),
            pre_contract_interval,
        );
        let executed = contract.check(&state, &Timestamp::new(Some(start_contract_stamp)));
        assert!(!executed);
    }
}
