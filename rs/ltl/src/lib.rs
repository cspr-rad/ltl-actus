#![allow(dead_code)]
use log::debug;
use std::collections::HashMap;
use std::fmt::Debug;

use prusti_contracts::*;
use serde::Deserialize;
use time::OffsetDateTime;

pub mod logic;
pub mod types;
use crate::logic::{Prop, TemporalProp};
use crate::types::TermSet;

#[derive(Debug, Eq, Hash, PartialEq, Ord, PartialOrd, Clone, Copy, Deserialize)]
pub struct Timestamp(OffsetDateTime);

impl Timestamp {
    pub fn new(x: Option<OffsetDateTime>) -> Self {
        match x {
            Some(t) => Timestamp(t),
            None => Timestamp(OffsetDateTime::now_utc()),
        }
    }
}

/** TrueWhen represents the interval in which a proposition is true.
 *
 * When `end_t` is `None`, it means that it's true forever into the future.
 * */
#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Copy)]
pub struct TrueWhen {
    start_t: Timestamp,
    end_t: Option<Timestamp>,
}

impl TrueWhen {
    pub fn new(start_t: Timestamp, end_t: Option<Timestamp>) -> Self {
        TrueWhen { start_t, end_t }
    }

    fn contains(&self, timestamp: &Timestamp) -> bool {
        match self.end_t {
            Some(end_t) => *timestamp >= self.start_t && *timestamp <= end_t,
            None => *timestamp >= self.start_t,
        }
    }
}

#[derive(Debug, Clone)]
pub struct StateStore<T>
where
    T: TermSet,
{
    states: HashMap<Prop<T>, Vec<TrueWhen>>,
}

impl<T> Default for StateStore<T>
where
    T: TermSet,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> StateStore<T>
where
    T: TermSet,
{
    pub fn new() -> Self {
        StateStore {
            states: HashMap::new(),
        }
    }

    pub fn add_state(&mut self, prop: Prop<T>, interval: TrueWhen) {
        self.states.entry(prop).or_default().push(interval);
    }

    fn is_true_at(&self, prop: &Prop<T>, timestamp: &Timestamp) -> bool {
        self.states.get(prop).map_or(false, |intervals| {
            intervals
                .iter()
                .any(|interval| interval.contains(timestamp))
        })
    }

    fn check_always(&self, prop: &Prop<T>, current_t: Timestamp) -> bool {
        self.states.get(prop).map_or(false, |intervals| {
            intervals.iter().any(|interval| {
                // Check if the interval is ongoing (end_t is None or in the future)
                // and started at or before the current time.
                interval.start_t <= current_t
                    && (interval.end_t.is_none() || interval.end_t.unwrap() > current_t)
            })
        })
    }

    fn check_eventually(&self, prop: &Prop<T>, current_t: Timestamp) -> bool {
        self.states.get(prop).map_or(false, |intervals| {
            intervals
                .iter()
                .all(|interval| (interval.end_t.is_none() || interval.end_t.unwrap() > current_t))
        })
    }

    /// TODO: refactor for overlapping intervals
    /**
     * If ψ is never true, φ must always be true.
     * Find the first interval where ψ is true
     * φ must be true for all times before and including the first ψ true interval
     * If there's no ψ interval starting before current_t, the condition is not satisfied
     * */
    fn check_release(&self, phi: &Prop<T>, psi: &Prop<T>, current_t: Timestamp) -> bool {
        let phi_intervals = self.states.get(phi).unwrap_or(&Vec::new()).clone();
        let psi_intervals = self.states.get(psi).unwrap_or(&Vec::new()).clone();

        if psi_intervals.is_empty() {
            return self.check_always(phi, current_t);
        }

        let first_psi_true = psi_intervals
            .iter()
            .filter(|interval| interval.start_t <= current_t)
            .min_by_key(|interval| interval.start_t);

        match first_psi_true {
            Some(first_psi_interval) => phi_intervals.iter().any(|interval| {
                interval.start_t <= current_t
                    && interval.end_t.unwrap_or(Timestamp(
                        OffsetDateTime::from_unix_timestamp(i64::MAX)
                            .expect("max timestamp panicked"),
                    )) >= first_psi_interval.start_t
            }),
            None => false,
        }
    }

    /// TODO: refactor for overlapping intervals
    /**
     * Find the earliest interval where φ is true after the current timestamp
     * Check if ψ is true continuously from current_t until the start of first_phi_interval
     * Found a gap where ψ is not true
     * - ψ holds continuously until φ becomes true
     * - ψ holds continuously until the end of time
     * - ψ does not hold continuously until φ
     * If φ never becomes true, check if ψ is always true from current_t
     * */
    fn check_until(&self, phi: &Prop<T>, psi: &Prop<T>, current_t: Timestamp) -> bool {
        let phi_intervals = self.states.get(phi).unwrap_or(&Vec::new()).clone();
        let psi_intervals = self.states.get(psi).unwrap_or(&Vec::new()).clone();

        let first_phi_true = phi_intervals
            .iter()
            .filter(|interval| interval.start_t >= current_t)
            .min_by_key(|interval| interval.start_t);

        match first_phi_true {
            Some(first_phi_interval) => {
                let mut last_psi_end = current_t;
                for interval in psi_intervals.iter() {
                    if interval.start_t > last_psi_end {
                        return false;
                    }
                    match interval.end_t {
                        Some(end_t) => {
                            if end_t >= first_phi_interval.start_t {
                                return true;
                            }
                            last_psi_end = end_t;
                        }
                        None => {
                            return true;
                        }
                    }
                }
                false
            }
            None => self.check_always(psi, current_t),
        }
    }
}

impl<T> PartialEq for StateStore<T>
where
    T: TermSet,
{
    fn eq(&self, other: &Self) -> bool {
        self.states
            .keys()
            .all(|k| self.states.get(k) == other.states.get(k))
    }
}

impl<T> Prop<T>
where
    T: TermSet,
{
    pub fn new(x: T) -> Prop<T> {
        Prop::Var(x)
    }

    pub fn check(&self, state: &StateStore<T>, current_t: &Timestamp) -> bool {
        match self {
            Prop::Var(_) => state.is_true_at(self, current_t),
            Prop::Eq(x, y) => x == y,
            Prop::Not(p) => !p.check(state, current_t),
            Prop::Or(p, q) => p.check(state, current_t) || q.check(state, current_t),
        }
    }
}

impl<T> TemporalProp<T>
where
    T: TermSet,
{
    pub fn new(p: Prop<T>) -> TemporalProp<T> {
        TemporalProp::Term(p)
    }
    pub fn unlift_prop(&self) -> Option<Prop<T>> {
        match self {
            TemporalProp::Term(p) => Some(p.clone()),
            TemporalProp::Always(tp) => tp.unlift_prop(),
            TemporalProp::Eventually(tp) => tp.unlift_prop(),
            _ => None, // TODO
        }
    }
    pub fn check(&self, state: &StateStore<T>, current_t: &Timestamp) -> bool {
        debug!("---recurring!------------------------------");
        debug!("Executing temporal prop: {}", self);
        match self {
            TemporalProp::Term(p) => {
                debug!("Executing term: {}", p);
                p.check(state, current_t)
            }
            TemporalProp::And(p, q) => {
                debug!("Executing and: {} and {}", p, q);
                p.check(state, current_t) && q.check(state, current_t)
            }
            TemporalProp::Always(tp) => match &**tp {
                TemporalProp::Term(x) => {
                    let always_check = state.check_always(x, *current_t);
                    debug!("Always check at {}: {}", &x, always_check);
                    always_check
                }
                _ => {
                    debug!("Descending into {} ...", tp);
                    tp.check(state, current_t)
                }
            },
            TemporalProp::Eventually(tp) => match &**tp {
                TemporalProp::Term(x) => {
                    let eventually_check = state.check_eventually(x, *current_t);
                    debug!("Eventually check at {}: {}", &x, eventually_check);
                    eventually_check
                }
                _ => {
                    debug!("Descending into {} ...", tp);
                    tp.check(state, current_t)
                }
            },
            TemporalProp::Release(_tp, _tq) => todo!(),
            // match (&**tp, &**tq) {
            //    (TemporalProp::Term(x), _) => state.check_release(&x, &**tq, *current_t),
            //    (_, TemporalProp::Term(y)) => state.check_release(&**tp, &y, *current_t),
            //    (_, _) => tp.eval(state, current_t) && tq.eval(state, current_t),
            //},
            TemporalProp::Until(_tp, _tq) => todo!(),
        }
    }
}

/*
 * helpers
 */
pub fn tt<T: TermSet>(x: T) -> TemporalProp<T> {
    TemporalProp::<T>::Term(Prop::<T>::Or(
        Box::new(Prop::<T>::Var(x.clone())),
        Box::new(Prop::<T>::Var(x.clone())),
    ))
}

pub fn ff<T: TermSet>(x: T) -> TemporalProp<T> {
    TemporalProp::<T>::Term(Prop::<T>::Not(Box::new(tt(x).unlift_prop().unwrap())))
}

pub fn var<T: TermSet>(x: T) -> TemporalProp<T> {
    TemporalProp::<T>::Term(Prop::<T>::Var(x))
}

pub fn not<T: TermSet>(p: &TemporalProp<T>) -> TemporalProp<T> {
    TemporalProp::<T>::Term(Prop::<T>::Not(Box::new(p.unlift_prop().unwrap())))
}

pub fn or<T: TermSet>(p: &TemporalProp<T>, q: &TemporalProp<T>) -> TemporalProp<T> {
    TemporalProp::<T>::Term(Prop::<T>::Or(
        Box::new(p.unlift_prop().unwrap()),
        Box::new(q.unlift_prop().unwrap()),
    ))
}

pub fn and<T: TermSet>(p: &TemporalProp<T>, q: &TemporalProp<T>) -> TemporalProp<T> {
    TemporalProp::<T>::And(Box::new(p.clone()), Box::new(q.clone()))
}

pub fn implies<T: TermSet>(p: &TemporalProp<T>, q: &TemporalProp<T>) -> TemporalProp<T> {
    or(&not(p), q)
}

pub fn iff<T: TermSet>(p: &TemporalProp<T>, q: &TemporalProp<T>) -> TemporalProp<T> {
    and(&implies(p, q), &implies(q, p))
}

pub fn always<T: TermSet>(p: TemporalProp<T>) -> TemporalProp<T> {
    TemporalProp::<T>::Always(Box::new(p))
}

pub fn eventually<T: TermSet>(p: TemporalProp<T>) -> TemporalProp<T> {
    TemporalProp::<T>::Eventually(Box::new(p))
}

pub fn release<T: TermSet>(p: TemporalProp<T>, q: TemporalProp<T>) -> TemporalProp<T> {
    TemporalProp::<T>::Release(Box::new(p), Box::new(q))
}

pub fn until<T: TermSet>(p: TemporalProp<T>, q: TemporalProp<T>) -> TemporalProp<T> {
    TemporalProp::<T>::Until(Box::new(p), Box::new(q))
}

/*
 * Tests
 */
