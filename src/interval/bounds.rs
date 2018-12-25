use std::cmp::Ordering;
use self::Ordering::*;



pub trait BoundTrait<T> : Ord where T: Ord {
    fn val(&self) -> & T;
    fn is_closed(&self) -> bool;
    fn new(val: T, is_closed: bool) -> Self;
}


//LowerBound ======================================================

#[derive(Clone, Debug, Copy, Eq, PartialEq, Hash)]
pub struct LowerBound<T> where T: Ord {
    val: T,
    is_closed: bool
}

impl <T> LowerBound<T> where T: Ord {

    pub fn is_separated_from(&self, other: &UpperBound<T>) -> bool {
        are_separated(self, other)
    }
}

impl<T> BoundTrait<T> for LowerBound<T> where T: Ord {
    fn val(&self) -> &T {
        &self.val
    }

    fn is_closed(&self) -> bool {
        self.is_closed
    }

    fn new(val: T, is_closed: bool) -> Self {
        Self {
            val, is_closed
        }
    }
}

impl<T> Ord for LowerBound<T> where T: Ord {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.cmp(other) {
            Greater => Greater,
            Less => Less,
            Equal => {
                if self.is_closed == other.is_closed {
                    Equal
                } else {
                    if self.is_closed {
                        Less
                    } else {
                        Greater
                    }
                }
            }
        }
    }
}

impl<T> PartialOrd for LowerBound<T> where T: Ord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


//UpperBound ======================================================

#[derive(Clone, Debug, Copy, Eq, PartialEq, Hash)]
pub struct UpperBound<T> where T: Ord {
    val: T,
    is_closed: bool
}

impl <T> UpperBound<T> where T: Ord {
    pub fn is_separated_from(&self, other: &LowerBound<T>) -> bool {
        are_separated(other, self)
    }
}

impl<T> BoundTrait<T> for UpperBound<T> where T: Ord {
    fn val(&self) -> &T {
        &self.val
    }

    fn is_closed(&self) -> bool {
        self.is_closed
    }

    fn new(val: T, is_closed: bool) -> Self{
        Self {
            val, is_closed
        }
    }
}

impl<T> Ord for UpperBound<T> where T: Ord {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.cmp(other) {
            Greater => Greater,
            Less => Less,
            Equal => {
                if self.is_closed == other.is_closed {
                    Equal
                } else {
                    if self.is_closed {
                        Greater
                    } else {
                        Less
                    }
                }
            }
        }
    }
}

impl<T> PartialOrd for UpperBound<T> where T: Ord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Helpers

fn  are_separated<T>(l: &LowerBound<T>, u: &UpperBound<T>) -> bool where T: Ord{
    l.val() > u.val() || (u.val() == l.val() && !u.is_closed() && ! l.is_closed())
}
