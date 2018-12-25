#![allow(dead_code)]

use super::bounds::{LowerBound, UpperBound};

#[derive(Clone, Debug, Copy, Eq, PartialEq, Hash)]
pub enum IntervalImpl<T> where T: Ord {
    Empty,
    Open(T, T),
    LowerClosed(T, T),
    UpperClosed(T, T),
    Closed(T, T)

}

#[derive(Clone, Debug, Copy, Eq, PartialEq, Hash)]
pub struct NonEmptyInterval<T> where T: Ord {
    pub lo: T,
    pub up: T,
    pub loc: bool,
    pub upc: bool
}

impl <T>  NonEmptyInterval<T> where T:Ord {
    pub fn upper(&self) -> UpperBound<&T>{
        UpperBound::new(&self.up, self.upc)
    }
    pub fn lower(&self) -> LowerBound<&T>{
        LowerBound::new(&self.lo, self.loc)
    }

    pub fn into_bounds(self) -> (LowerBound<T>, UpperBound<T>) {
        (LowerBound::new(self.lo, self.loc), UpperBound::new(self.up, self.upc))
    }
}