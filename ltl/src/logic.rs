use std::fmt::{Debug, Display};

use crate::types::TermSet;

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum Prop<T>
where
    T: TermSet,
{
    Var(T),
    Eq(T, T),
    Not(Box<Prop<T>>),
    Or(Box<Prop<T>>, Box<Prop<T>>),
}

impl<T> Display for Prop<T>
where
    T: TermSet,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Prop::Var(x) => write!(f, "{:?}", x),
            Prop::Eq(x, y) => write!(f, "{:?} = {:?}", x, y),
            Prop::Not(p) => write!(f, "¬({})", p),
            Prop::Or(p, q) => write!(f, "({}) ∨ ({})", p, q),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub enum TemporalProp<T>
where
    T: TermSet,
{
    Term(Prop<T>),
    And(Box<TemporalProp<T>>, Box<TemporalProp<T>>),
    Always(Box<TemporalProp<T>>),
    Eventually(Box<TemporalProp<T>>),
    Release(Box<TemporalProp<T>>, Box<TemporalProp<T>>),
    Until(Box<TemporalProp<T>>, Box<TemporalProp<T>>),
}

impl<T> Display for TemporalProp<T>
where
    T: TermSet,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TemporalProp::Term(p) => Display::fmt(p, f),
            TemporalProp::Always(p) => {
                write!(f, "□(");
                let _ = Display::fmt(&**p, f);
                write!(f, ")")
            }
            TemporalProp::And(p, q) => {
                write!(f, "(")?;
                let _ = Display::fmt(&**p, f)?;
                write!(f, ") ∧ (")?;
                let _ = Display::fmt(&**q, f)?;
                write!(f, ")")
            }
            TemporalProp::Eventually(p) => {
                write!(f, "◇(");
                let _ = Display::fmt(&**p, f);
                write!(f, ")")
            }
            TemporalProp::Release(p, q) => {
                write!(f, "(")?;
                let _ = Display::fmt(&**p, f)?;
                write!(f, ") R (")?;
                let _ = Display::fmt(&**q, f)?;
                write!(f, ")")
            }
            TemporalProp::Until(p, q) => {
                write!(f, "(")?;
                let _ = Display::fmt(&**p, f)?;
                write!(f, ") U (")?;
                let _ = Display::fmt(&**q, f)?;
                write!(f, ")")
            }
        }
    }
}
