extern crate ltl;

use fpdec::{Dec, Decimal};
use serde::Deserialize;
// use time::{Duration, PrimitiveDateTime};

use ltl::*;

use crate::exec::*;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Deserialize, Eq, Hash)]
pub struct PamTerms {
    principal: Decimal,
    interest_rate: Decimal,
    months: usize,
}

pub struct PamEvent {
    payment: Decimal,
}

impl PamTerms {
    fn new(principal: Decimal, interest_rate: Decimal, months: usize) -> Self {
        PamTerms {
            principal,
            interest_rate,
            months,
        }
    }

    // fn monthly_payment(&self) -> Decimal {
    //     let rate = self.interest_rate;
    //     let numerator = rate * self.principal;
    //     let denominator = Dec!(1.0) - (Dec!(1.0) + rate).powi(-(self.months as i64));
    //     numerator / denominator
    // }

    /// simplified monthly payment that mutates principal
    fn payment(&mut self, payment: Decimal) {
        self.principal = (1 + self.interest_rate) * self.principal - payment;
    }
}

type PamProp = TemporalProp<PamTerms>;
