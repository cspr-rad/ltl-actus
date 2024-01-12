extern crate ltl;

use serde::Deserialize;
use time::{Duration, OffsetDateTime};

use ltl::*;

// use self::exec::*;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Deserialize)]
pub struct PamTerms {
    principal: f64,
    interest_rate: f64,
    months: usize,
}

impl PamTerms {
    fn new(principal: f64, interest_rate: f64, months: usize) -> Self {
        PamTerms {
            principal,
            interest_rate,
            months,
        }
    }
}

type PamProposition = TemporalProp<PamTerms>;
