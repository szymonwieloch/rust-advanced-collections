#![allow(dead_code)]

#[derive(Clone, Debug, Copy, Eq, PartialEq, Hash)]
pub enum IntervalImpl<T> where T: Ord {
    Empty,
    Open(T, T),
    LowerClosed(T, T),
    UpperClosed(T, T),
    Closed(T, T)

}

#[derive(Clone, Debug, Copy, Eq, PartialEq, Hash)]
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
}

#[derive(Clone, Debug, Copy, Eq, PartialEq, Hash)]
pub struct NonEmptyInterval<T> where T: Ord {
    pub lo: T,
    pub up: T,
    pub loc: bool,
    pub upc: bool
}

impl <T>  NonEmptyInterval<T> where T:Ord {
    pub fn upper(&self) -> Bound<T>{
        Bound{
            v: &self.up,
            c: self.upc
        }
    }
    pub fn lower(&self) -> Bound<T>{
        Bound{
            v: &self.lo,
            c: self.loc
        }
    }
}