use std::fmt::Debug;

pub trait TermSet: Debug + Clone + PartialEq + Eq + std::hash::Hash {}
