use std::collections::HashMap;

use ltl::{StateStore, TemporalProp, TermSet, Timestamp};

pub trait Event: Clone + PartialEq + Eq {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Trace<T>(Vec<(Timestamp, T)>);

impl<T> Trace<T> {
    fn new() -> Self {
        Trace(Vec::new())
    }

    fn push(&mut self, timestamp: Timestamp, value: T) {
        self.0.push((timestamp, value));
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Execution<T, E>
where
    T: TermSet,
    E: Event,
{
    start_t: Timestamp,
    terms: T,
    contract: TemporalProp<T>,
    store: StateStore<T>,
}

impl<T, E> Execution<T, E>
where
    T: TermSet,
    E: Event,
{
    fn new(terms: T, contract: TemporalProp<T>, start_t: Timestamp) -> Self {
        Execution {
            start_t,
            terms,
            contract,
            store: StateStore::new(),
        }
    }

    /** The execution has multiple counterparties. */
    pub fn run(&mut self) -> Trace<T> {
        let mut trace = Trace::new();
        // dummy
        trace
    }
}
