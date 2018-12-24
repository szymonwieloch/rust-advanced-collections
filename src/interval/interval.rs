#![allow(dead_code)]

use std::cmp::{Ord, Ordering};
use super::interval_impl::IntervalImpl;
use std::fmt::{Formatter, Display, Result as FmtResult};

use self::IntervalImpl::*;
use self::Ordering::*;

#[derive(Clone, Debug, Copy, Eq, PartialEq, Hash)]
pub struct Interval<T> where T: Ord {
    pub (super) imp: IntervalImpl<T>
}

impl<T> Interval<T>  where T: Ord  {
    pub fn single(val: T) -> Self where T: Clone{
        Self{
            imp: IntervalImpl::Closed(val.clone(), val)
        }
    }

    pub fn new(low: T, low_closed: bool, up: T, up_closed: bool) -> Self {
        if low > up {
            panic!("Lower bound of an interval needs to be less than the upper one.");
        }

        if low == up &&(!low_closed || !up_closed) {
            panic!("Single elements need to have closed bounds.");
        }
        let imp = if low_closed {
            if up_closed{
                Closed(low, up)
            } else {
                LowerClosed(low, up)
            }
        } else {
            if up_closed {
                UpperClosed(low, up)
            } else {
                Open(low, up)
            }
        };
        Self{
            imp
        }
    }

    pub fn empty() -> Self {
        Self {
            imp: IntervalImpl::Empty
        }
    }

    pub fn open(low: T, up: T) -> Self {
        Self::new(low, false, up, false)
    }

    pub fn closed(low: T, up: T) -> Self {
        Self::new(low, true, up, true)
    }

    pub fn lower_closed(low: T, up: T) -> Self {
        Self::new(low, true, up, false)
    }

    pub fn upper_closed(low: T, up: T) -> Self {
        Self::new(low, false, up, true)
    }

    pub fn upper(&self) -> Option<&T> {
        match self.imp{
            Empty => None,
            Open(ref _low, ref up) | Closed(ref _low, ref up) | LowerClosed(ref _low, ref up)
            | UpperClosed(ref _low, ref up) => Some(up)
        }
    }

    pub fn lower(&self) -> Option<&T> {
        match self.imp{
            Empty => None,
            Open(ref low, ref _up) | Closed(ref low, ref _up) | LowerClosed(ref low, ref _up)
            | UpperClosed(ref low, ref _up) => Some(low)
        }
    }

    pub fn range(&self) -> Option<(&T, &T)> {
        match self.imp{
            Empty => None,
            Open(ref low, ref up) | Closed(ref low, ref up) | LowerClosed(ref low, ref up)
            | UpperClosed(ref low, ref up) => Some((low, up))
        }
    }

    pub fn is_empty(&self) -> bool {
        self.imp == IntervalImpl::Empty
    }

    pub fn is_single(&self) -> bool {
        if let IntervalImpl::Closed(ref a, ref b) = self.imp{
            a == b
        } else {
            false
        }
    }

    pub fn is_lower_closed(&self) -> bool {
        match self.imp{
            Closed(..) | LowerClosed(..) => true,
            _=> false
        }
    }

    pub fn is_upper_closed(&self) -> bool {
        match self.imp{
            Closed(..) | UpperClosed(..) => true,
            _=> false
        }
    }

    //contains

    pub fn contains_val(&self, val: &T) -> bool {
        !(self > val || self < val)
    }

    pub fn contains_interval(&self, other: &Self) -> bool {
        let (low, up) = match self.range() {
            None => return other.is_empty(),
            Some(a) => a
        };

        let (olow, oup) = match other.range() {
            None => return true,
            Some(a) => a
        };

        if low > olow || (low == olow && !self.is_lower_closed() && other.is_lower_closed()){
            return false;
        }
        if up < oup || (up == oup && !self.is_upper_closed() && other.is_upper_closed()){
            return false;
        }
        true
    }

    pub fn into_tuple(self) -> Option<(T, bool, T, bool)>{
        match self.imp {
            Empty => None,
            Open(lo, up) => Some((lo, false, up, false)),
            Closed(lo, up) => Some((lo, true, up, true)),
            LowerClosed(lo, up) => Some((lo, true, up, false)),
            UpperClosed(lo, up) => Some((lo, false, up, true))
        }
    }

    //merge

    pub fn can_be_merged(&self, other: &Self) -> bool {
        let (lo, up) = match self.range(){
            None => return true,
            Some(a) => a
        };

        let (olo, oup) = match other.range() {
            None => return true,
            Some(a) => a
        };
        if lo > oup || (lo == oup && !self.is_lower_closed() && !other.is_upper_closed()){
            return false;
        }

        if up < olo || (up == olo && ! self.is_upper_closed() && ! other.is_lower_closed()){
            return false;
        }
        true
    }

    pub fn into_merged(self, other: Self) -> Result<Self, (Self, Self)>{
        if ! self.can_be_merged(&other){
            return Err((self, other));
        }
        if other.is_empty(){
            return Ok(self);
        }
        if self.is_empty(){
            return Ok(other);
        }
        let (l1, l1c, u1, u1c) = self.into_tuple().unwrap();
        let (l2, l2c, u2, u2c) = other.into_tuple().unwrap();
        let (lo, loc) = Self::less_bound(l1, l1c, l2, l2c);
        let (up, upc) = Self::greater_bound(u1, u1c, u2, u2c);
        Ok(Self::new(lo, loc, up, upc))
    }

    //intersection

    pub fn into_intersection(self, other: Self) -> Self {
        if self.is_empty() {
            return self;
        }
        if other.is_empty(){
            return other;
        }

        if self>other || self < other {
            return Self::empty();
        }
        let (l1, l1c, u1, u1c) =  self.into_tuple().unwrap();
        let (l2, l2c, u2, u2c) = other.into_tuple().unwrap();

        let (lo, loc) = match l1.cmp(&l2){
            Less => (l2, l2c),
            Greater => (l1, l1c),
            Equal => (l1, l1c && l2c)
        };

        let (up, upc) = match u1.cmp(&u2){
            Greater => (u2, u2c),
            Less => (u1, u1c),
            Equal => (u1, u1c && u2c)
        };
        Self::new(lo, loc, up, upc)
    }

    //span
    pub fn into_span(self, other: Self) -> Self {
        if other.is_empty(){
            return self;
        }
        if self.is_empty(){
            return other;
        }
        let (l1, l1c, u1, u1c) =  self.into_tuple().unwrap();
        let (l2, l2c, u2, u2c) = other.into_tuple().unwrap();

        let (lo, loc) = Self::less_bound(l1, l1c, l2, l2c);
        let (up, upc) = Self::greater_bound(u1, u1c, u2, u2c);

        Self::new(lo, loc, up, upc)
    }

    pub(super) fn range_mut(&mut self) ->  Option<(&mut T, & mut T)> {
        match self.imp {
            Empty => None,
            Open(ref mut l, ref mut u) | Closed(ref mut l, ref mut u) | UpperClosed(ref mut l, ref mut u) | LowerClosed(ref mut l, ref mut u)
            => Some((l, u))
        }
    }

    fn less_bound(v1: T, v1c: bool, v2: T, v2c: bool) -> (T, bool){
        match v1.cmp(&v2) {
            Greater => (v2, v2c),
            Less => (v1, v1c),
            Equal => (v1, v1c || v2c)
        }
    }

    fn greater_bound(v1: T, v1c: bool, v2: T, v2c: bool) -> (T, bool){
        match v1.cmp(&v2) {
            Greater => (v1, v1c),
            Less => (v2, v2c),
            Equal => (v1, v1c || v2c)
        }
    }

}

impl<T> Display for Interval<T> where T: Ord + Display {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self.imp{
            Empty => write!(f, "Ø"),
            Open(ref a, ref b) => write!(f, "({},{})", a, b),
            Closed(ref a, ref b) => write!(f, "[{},{}]", a, b),
            LowerClosed(ref a, ref b) => write!(f, "[{},{})", a, b),
            UpperClosed(ref a, ref b) => write!(f, "({},{}]", a, b),

        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_create_empty(){
        let i: Interval<i32> = Interval::empty();
        assert!(!i.is_lower_closed());
        assert!(!i.is_upper_closed());
        assert_eq!(i.lower(), None);
        assert_eq!(i.upper(), None);
        assert!(i.is_empty());
    }

    #[test]
    fn test_create_single(){
        let i = Interval::single(5);
        assert!(i.is_lower_closed());
        assert!(i.is_upper_closed());
        assert_eq!(i.lower(), Some(&5));
        assert_eq!(i.upper(), Some(&5));
        assert!(!i.is_empty());
    }

    #[test]
    fn test_create_closed(){
        let i = Interval::closed(3,5);
        assert!(i.is_lower_closed());
        assert!(i.is_upper_closed());
        assert_eq!(i.lower(), Some(&3));
        assert_eq!(i.upper(), Some(&5));
        assert!(!i.is_empty());
    }

    #[test]
    #[should_panic]
    fn test_create_closed_fail(){
        let _i = Interval::closed(5,4);
    }

    #[test]
    fn test_create_lower_closed(){
        let i = Interval::lower_closed(3,5);
        assert!(i.is_lower_closed());
        assert!(!i.is_upper_closed());
        assert_eq!(i.lower(), Some(&3));
        assert_eq!(i.upper(), Some(&5));
        assert!(!i.is_empty());
    }

    #[test]
    #[should_panic]
    fn test_create_lower_closed_fail(){
        let _i = Interval::lower_closed(5,5);
    }

    #[test]
    fn test_create_upper_closed(){
        let i = Interval::upper_closed(3,5);
        assert!(!i.is_lower_closed());
        assert!(i.is_upper_closed());
        assert_eq!(i.lower(), Some(&3));
        assert_eq!(i.upper(), Some(&5));
        assert!(!i.is_empty());
    }

    #[test]
    #[should_panic]
    fn test_create_upper_closed_fail(){
        let _i = Interval::upper_closed(5,5);
    }

    #[test]
    fn test_contains_val(){

        let i = Interval::lower_closed(4,6);
        assert!(!i.contains_val(&3));
        assert!(i.contains_val(&4));
        assert!(i.contains_val(&5));
        assert!(!i.contains_val(&6));
        assert!(!i.contains_val(&7));
    }

    #[test]
    fn test_contains_interval(){
        let i = Interval::lower_closed(4,8);
        let e: Interval<i32> = Interval::empty();
        assert!(i.contains_interval(&Interval::open(6,7)));
        assert!(i.contains_interval(&Interval::open(4,8)));
        assert!(!i.contains_interval(&Interval::open(3,7)));
        assert!(!i.contains_interval(&Interval::open(6,9)));
        assert!(!i.contains_interval(&Interval::open(3,9)));
        assert!(!i.contains_interval(&Interval::closed(4,8)));
        assert!(i.contains_interval(&i));
        assert!(i.contains_interval(&e));
        assert!(e.contains_interval(&e));
        assert!(!e.contains_interval(&Interval::open(3,7)));

    }

}

