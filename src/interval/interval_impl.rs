#![allow(dead_code)]

use super::bounds::{LowerBound, UpperBound};

#[derive(Clone, Debug, Copy, Eq, PartialEq, Hash)]
pub struct NonEmptyInterval<T> where T: Ord {
    pub lo: LowerBound<T>,
    pub up: UpperBound<T>
}
