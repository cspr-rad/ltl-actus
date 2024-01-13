/// For the purposes of january demo, this is dead code. No time for fully general execution
// use std::collections::HashMap;
use std::iter::Iterator;

use ltl::{StateStore, TemporalProp, TermSet, Timestamp};

pub trait Event: Clone + PartialEq + Eq {
    fn timestamp(&self) -> Timestamp;
    fn action(&self) -> Self;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Trace<T>(Vec<(Timestamp, T)>);

impl<T> Trace<T> {
    fn new() -> Self {
        Trace(Vec::new())
    }

    fn push(&mut self, timestamp: Timestamp, value: T) {
        self.0.push((timestamp, value));
    }
}

#[derive(Debug, PartialEq)]
pub struct Execution<T>
where
    T: TermSet,
{
    start_t: Timestamp,
    terms: T,
    contract: TemporalProp<T>,
    store: StateStore<T>,
}

impl<T> Execution<T>
where
    T: TermSet,
{
    fn new(terms: T, contract: TemporalProp<T>, start_t: Timestamp) -> Self {
        Execution {
            start_t,
            terms,
            contract,
            store: StateStore::new(),
        }
    }

    /**
     * The execution has multiple counterparties.
     * For simplicity, we'll start with two.
     * */
    pub fn run<E: Event>(&mut self, events: impl Iterator<Item = E>) -> Trace<T> {
        events
            .map(|event| {
                let timestamp = event.timestamp();

                // Update state and evaluate contract
                self.update_state_store(&event);
                self.evaluate_contract(&event);

                // Return the current state with the timestamp
                (timestamp, self.terms.clone())
            })
            .fold(Trace::new(), |mut trace, (timestamp, state)| {
                // Add each state to the trace
                trace.push(timestamp, state);
                trace
            })
    }
    fn update_state_store<E: Event>(&mut self, event: &E) {
        // Logic to update state store based on the event
        // This could involve modifying the `states` HashMap in `StateStore<T>`
    }

    fn evaluate_contract<E: Event>(&mut self, event: &E) {
        // Logic to evaluate how the event affects the contract
        // This involves interpreting `TemporalProp<T>` based on the event
    }
}
