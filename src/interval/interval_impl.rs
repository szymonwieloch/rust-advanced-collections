#![allow(dead_code)]

use super::bounds::{LowerBound, UpperBound, BoundTrait};

#[derive(Clone, Debug, Copy, Eq, PartialEq, Hash)]
pub enum IntervalImpl<T> where T: Ord {
    Empty,
    Open(T, T),
    LowerClosed(T, T),
    UpperClosed(T, T),
    Closed(T, T)

}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Bound<'a, T> where T: Ord {
    v: &'a T,
    c: bool
}

impl<'a, T> Bound<'a, T> where T: Ord{
    #[inline]
    pub fn val(&self) -> &'a T {
        self.v
    }

    #[inline]
    pub fn is_closed(&self) -> bool {
        self.c
    }

    pub fn are_separated(first_up: Self, second_low: Self) -> bool {
        second_low.val() > first_up.val() || (first_up.val() == second_low.val() && !first_up.is_closed() && ! second_low.is_closed())
    }
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
}