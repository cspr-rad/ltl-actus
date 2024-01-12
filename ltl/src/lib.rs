use std::collections::HashMap;
use time::OffsetDateTime;

fn filter_hashmap_by_key<K, V>(map: &HashMap<K, V>, predicate: impl Fn(&K) -> bool) -> HashMap<K, V>
where
    K: Clone + Eq + std::hash::Hash,
    V: Clone,
{
    map.iter()
        .filter(|(&ref key, _)| predicate(&key))
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect()
}

#[derive(Debug, PartialEq, Clone)]
pub enum Prop<T: Clone + PartialEq> {
    True,
    False,
    Var(T),
    Not(Box<Prop<T>>),
    Or(Box<Prop<T>>, Box<Prop<T>>),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Tick(usize);

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Clone, Copy)]
pub struct Timestamp(OffsetDateTime);

#[derive(Debug, Clone)]
pub struct StateStore<T: Clone + PartialEq> {
    states: HashMap<Timestamp, Vec<Prop<T>>>,
    current_t: Timestamp,
}

/*
impl<T: Clone + PartialEq> Prop<T> {
    pub fn eval(&self, state: &StateStore<T>) -> bool {
        match self {
            Prop::True => true,
            Prop::False => false,
            Prop::Var(x) => state.check_always(&Prop::Var(x)),
            Prop::Not(p) => !p.eval(state),
            Prop::Or(p, q) => p.eval(state) || q.eval(state),
        }
    }
}
*/

impl<T: Clone + PartialEq> PartialEq for StateStore<T> {
    fn eq(&self, other: &Self) -> bool {
        self.current_t == other.current_t
            && self
                .states
                .keys()
                .all(|k| self.states.get(k) == other.states.get(k))
    }
}

impl<T: Clone + PartialEq> StateStore<T> {
    pub fn new() -> StateStore<T> {
        StateStore {
            states: HashMap::new(),
            current_t: Timestamp(OffsetDateTime::now_utc()),
        }
    }

    pub fn add_state(&mut self, t: Timestamp, p: Prop<T>) {
        let v = self.states.entry(t).or_insert_with(|| Vec::new());
        v.push(p);
    }

    pub fn get_state(&self, t: Timestamp) -> Option<&Vec<Prop<T>>> {
        self.states.get(&t)
    }

    pub fn update_current_t(&mut self, t: Timestamp) {
        self.current_t = t;
    }

    pub fn future_states(&self) -> HashMap<Timestamp, Vec<Prop<T>>> {
        filter_hashmap_by_key(&self.states, |&t| t >= self.current_t)
    }

    pub fn check_always(&self, prop: &Prop<T>) -> bool {
        self.future_states()
            .values()
            .all(|props| props.contains(prop))
    }

    pub fn check_eventually(&self, p: &Prop<T>) -> bool {
        self.future_states().values().any(|props| props.contains(p))
    }

    pub fn check_release(&self, psi: &Prop<T>, phi: &Prop<T>) -> bool {
        let binding = self.future_states();
        let mut timestamps = binding
            .iter()
            .map(|(&t, props)| (t, props.contains(psi), props.contains(phi)));

        let (psi_becomes_true_at, phi_always_true_until_psi) = timestamps
            .clone() // Clone the iterator to use it twice
            .take_while(|&(_, psi_true, _)| !psi_true)
            .fold((false, true), |(_, phi_always_true), (_, _, phi_true)| {
                (true, phi_always_true && phi_true)
            });

        // If ψ becomes true, φ must have been true up to that point
        // If ψ never becomes true, φ must be true at all timestamps
        psi_becomes_true_at && phi_always_true_until_psi
            || !psi_becomes_true_at && timestamps.all(|(_, _, phi_true)| phi_true)
    }

    pub fn check_until(&self, phi: &Prop<T>, psi: &Prop<T>) -> bool {
        let binding = self.future_states();
        let timestamps = binding
            .iter()
            .map(|(_, props)| (props.contains(phi), props.contains(psi)));

        let mut psi_not_found = true;
        timestamps
            .take_while(|&(_, psi_true)| {
                psi_not_found = !psi_true;
                psi_not_found // Continue until psi is found
            })
            .all(|(phi_true, _)| phi_true)
            && !psi_not_found
    }

    pub fn state(&self) -> &HashMap<Timestamp, Vec<Prop<T>>> {
        &self.states
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TemporalProp<T: Clone + PartialEq> {
    Term(Prop<T>, Timestamp),
    Always(Box<TemporalProp<T>>),
    Eventually(Box<TemporalProp<T>>),
    Release(Box<TemporalProp<T>>, Box<TemporalProp<T>>),
    Until(Box<TemporalProp<T>>, Box<TemporalProp<T>>),
}

pub fn drop_timestamp<T: Clone + PartialEq>(x: &TemporalProp<T>) -> Prop<T> {
    match x {
        TemporalProp::Term(b, _) => b.clone(),
        TemporalProp::Always(p) => drop_timestamp(&p).clone(),
        TemporalProp::Eventually(p) => drop_timestamp(&p).clone(),
        TemporalProp::Release(p, q) => Prop::Or(
            Box::new(drop_timestamp(&p).clone()),
            Box::new(drop_timestamp(&q).clone()),
        ),
        TemporalProp::Until(p, q) => Prop::Or(
            Box::new(drop_timestamp(&p).clone()),
            Box::new(drop_timestamp(&q).clone()),
        ),
    }
}

pub fn apply_modal<T: Clone + PartialEq>(
    f: TemporalProp<T>,
    store: &StateStore<T>, // Add reference to StateStore
) -> TemporalProp<T> {
    match f {
        TemporalProp::Term(p, t) => unreachable!(),
        TemporalProp::Always(p) => {
            match *p {
                TemporalProp::Term(p, t) => {
                    // Implement logic considering all timestamps in the state store
                    if store.check_always(&p) {
                        return TemporalProp::Term(Prop::True, store.current_t); // If true for all timestamps
                    } else {
                        return TemporalProp::Term(Prop::False, store.current_t);
                        // If not true for all timestamps
                    }
                }
                TemporalProp::Always(p) => apply_modal(*p, store),
                TemporalProp::Eventually(p) => apply_modal(*p, store),
                TemporalProp::Release(p, q) => TemporalProp::Release(
                    Box::new(apply_modal(*p, store)),
                    Box::new(apply_modal(*q, store)),
                ),
                TemporalProp::Until(p, q) => TemporalProp::Until(
                    Box::new(apply_modal(*p, store)),
                    Box::new(apply_modal(*q, store)),
                ),
            }
        }
        TemporalProp::Eventually(p) => {
            match *p {
                TemporalProp::Term(p, _) => {
                    // Implement logic considering all timestamps in the state store
                    if store.check_eventually(&p) {
                        TemporalProp::Term(Prop::True, store.current_t) // If true for some timestamp
                    } else {
                        TemporalProp::Term(Prop::False, store.current_t) // If not true for some timestamp
                    }
                }
                TemporalProp::Always(p) => apply_modal(*p, store),
                TemporalProp::Eventually(p) => apply_modal(*p, store),
                TemporalProp::Release(p, q) => TemporalProp::Release(
                    Box::new(apply_modal(*p, store)),
                    Box::new(apply_modal(*q, store)),
                ),
                TemporalProp::Until(p, q) => TemporalProp::Until(
                    Box::new(apply_modal(*p, store)),
                    Box::new(apply_modal(*q, store)),
                ),
            }
        }
        _ => todo!(), //    TemporalProp::Release(p, q) => {
                      //        // Logic for Release operator
                      //        match (*p, *q) {
                      //            (TemporalProp::Term(p1, tp), q1) => {
                      //                // Implement logic considering all timestamps in the state store
                      //                if store.check_release(&p1, &drop_timestamp(&q1)) {
                      //                    TemporalProp::Term(Prop::True, store.current_t) // If true for some timestamp
                      //                } else {
                      //                    TemporalProp::Term(Prop::False, store.current_t) // If not true for some timestamp
                      //                }
                      //            }
                      //            (p1, TemporalProp::Term(q1, tq)) => {
                      //                if store.check_release(&drop_timestamp(&p1), &q1) {
                      //                    TemporalProp::Term(Prop::True, store.current_t) // If true for some timestamp
                      //                } else {
                      //                    TemporalProp::Term(Prop::False, store.current_t) // If not true for some timestamp
                      //                }
                      //            }
                      //        }
                      //    }
                      //    TemporalProp::Until(p, q) => {
                      //        // Logic for Until operator
                      //        todo!()
                      //    } // ... other cases ...
    }
}

/*
 * helpers
 */
pub fn tt<T: Clone + PartialEq>(t: Timestamp) -> TemporalProp<T> {
    TemporalProp::<T>::Term(Prop::<T>::True, t)
}

pub fn ff<T: Clone + PartialEq>(t: Timestamp) -> TemporalProp<T> {
    TemporalProp::<T>::Term(Prop::<T>::False, t)
}

// maybe we'll do these.
// pub fn timed_var<T>(t: Timestamp) -> impl Fn(T) -> TemporalProp<T> {
//    move |x| TemporalProp::<T>::Term(Prop::<T>::Var(x), t)
//}

pub fn var<T: Clone + PartialEq>(x: T, t: Timestamp) -> TemporalProp<T> {
    TemporalProp::<T>::Term(Prop::<T>::Var(x), t)
}

pub fn not<T: Clone + PartialEq>(p: &TemporalProp<T>, t: Timestamp) -> TemporalProp<T> {
    TemporalProp::<T>::Term(Prop::<T>::Not(Box::new(drop_timestamp(&p).clone())), t)
}

pub fn or<T: Clone + PartialEq>(
    p: &TemporalProp<T>,
    q: &TemporalProp<T>,
    t: Timestamp,
) -> TemporalProp<T> {
    TemporalProp::<T>::Term(
        Prop::<T>::Or(
            Box::new(drop_timestamp(&p).clone()),
            Box::new(drop_timestamp(&q).clone()),
        ),
        t,
    )
}

pub fn and<T: Clone + PartialEq>(
    p: &TemporalProp<T>,
    q: &TemporalProp<T>,
    t: Timestamp,
) -> TemporalProp<T> {
    not(&or(&not(p, t), &not(q, t), t), t)
}

pub fn implies<T: Clone + PartialEq>(
    p: &TemporalProp<T>,
    q: &TemporalProp<T>,
    t: Timestamp,
) -> TemporalProp<T> {
    or(&not(p, t), q, t)
}

pub fn iff<T: Clone + PartialEq>(
    p: &TemporalProp<T>,
    q: &TemporalProp<T>,
    t: Timestamp,
) -> TemporalProp<T> {
    and(&implies(p, q, t), &implies(q, p, t), t)
}

pub fn always<T: Clone + PartialEq>(p: TemporalProp<T>) -> TemporalProp<T> {
    TemporalProp::<T>::Always(Box::new(p))
}

pub fn eventually<T: Clone + PartialEq>(p: TemporalProp<T>) -> TemporalProp<T> {
    TemporalProp::<T>::Eventually(Box::new(p))
}

pub fn release<T: Clone + PartialEq>(p: TemporalProp<T>, q: TemporalProp<T>) -> TemporalProp<T> {
    TemporalProp::<T>::Release(Box::new(p), Box::new(q))
}

/*
 * Tests
 */
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
