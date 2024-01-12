use std::collections::HashMap;

use ltl::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Trace<T>(Vec(Timestamp, T));

impl<T> Trace<T> {
    fn new() -> Self {
        Trace(Vec::new())
    }

    fn push(&mut self, timestamp: Timestamp, value: T) {
        self.0.push((timestamp, value));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Execution<T> {
    start_date: Timestamp,
    terms: T,
    contract: TemporalProp<T>,
    store: StateStore<T>,
}

impl<T> Execution<T> {
    fn new(terms: T) -> Self {
        Execution {
            terms,
            store: StateStore::new(),
        }
    }

    pub fn run(&mut self) -> Trace<T> {
        let mut trace = Trace::new();

        trace
    }
}
