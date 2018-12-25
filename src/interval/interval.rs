#![allow(dead_code)]

use std::cmp::{Ord, Ordering};
use super::interval_impl::{NonEmptyInterval, Bound};
use std::fmt::{Formatter, Display, Result as FmtResult};

use self::Ordering::*;

#[derive(Clone, Debug, Copy, Eq, PartialEq, Hash)]
pub struct Interval<T> where T: Ord {
    pub (super) imp: Option<NonEmptyInterval<T>>
}

impl<T> Interval<T>  where T: Ord  {

//creation =================================

    pub fn new(lower: T, lower_closed: bool, upper: T, upper_closed: bool) -> Self {
        Self::create_checked(lower, lower_closed, upper, upper_closed)
}

    pub(super) fn create_checked(lo: T, loc: bool, up: T, upc: bool) -> Self {
        if lo > up {
            panic!("Lower bound of an interval needs to be less than the upper one.");
        }

        if lo == up && (!loc || !upc) {
            panic!("Single elements need to have closed bounds.");
        }

        Self {
            imp: Some(NonEmptyInterval {
                lo,
                up,
                loc,
                upc,
            })
        }
    }

    pub (super) fn create_friendly(lo: T, loc: bool, up: T, upc: bool) -> Self {
    unimplemented!()
    }

    pub fn single(val: T) -> Self where T: Clone{
        Self{
            imp: Some(NonEmptyInterval{
                lo: val.clone(),
                up: val,
                loc: true,
                upc: true
            })
        }
    }

    pub fn open(low: T, up: T) -> Self {
        Self::create_checked(low, false, up, false)
    }

    pub fn closed(low: T, up: T) -> Self {
        Self::create_checked(low, true, up, true)
    }

    pub fn lower_closed(low: T, up: T) -> Self {
        Self::create_checked(low, true, up, false)
    }

    pub fn upper_closed(low: T, up: T) -> Self {
        Self::create_checked(low, false, up, true)
    }

    pub fn empty() -> Self {
        Self {
            imp: None
        }
    }

//accessors ===================================

    pub fn upper(&self) -> Option<Bound<T>> {
        match &self.imp{
            None => None,
            Some(a) => Some(a.upper())
        }
    }

    pub fn lower(&self) -> Option<Bound<T>> {
        match &self.imp{
            None => None,
            Some(a) => Some(a.lower())
        }
    }

    pub fn bounds(&self) -> Option<(Bound<T>, Bound<T>)> {
        match &self.imp{
            None => None,
            Some(a) => Some((
                a.lower(),
                a.upper()
            ))
        }
    }

    pub fn is_empty(&self) -> bool {
        self.imp.is_none()
    }

    pub fn is_single(&self) -> bool {
        if let Some(ref a) = self.imp {
            a.lo == a.up
        } else {
            false
        }
    }

    pub fn is_lower_closed(&self) -> Option<bool> {
        match &self.imp{
            None => None,
            Some(a) => Some(a.loc)
        }
    }

    pub fn is_upper_closed(&self) -> Option<bool> {
        match &self.imp{
           None => None,
            Some(a) => Some(a.upc)
        }
    }

    //contains

    pub fn contains_val(&self, val: &T) -> bool {
        !(self > val || self < val)
    }

    pub fn contains_interval(&self, other: &Self) -> bool {
        let (l, u) = match self.bounds() {
            None => return other.is_empty(),
            Some(a) => a
        };

        let (ol, ou) = match other.bounds() {
            None => return true,
            Some(a) => a
        };

        if l.val() > ol.val() || (l.val() == ol.val() && !l.is_closed() && ol.is_closed()){
            return false;
        }
        if u.val() < ou.val() || (u.val() == ou.val() && !u.is_closed() && ou.is_closed()){
            return false;
        }
        true
    }

    pub fn into_tuple(self) -> Option<(T, bool, T, bool)>{
        match self.imp {
            None => None,
            Some(a) => Some((a.lo, a.loc, a.up, a.upc))
        }
    }

    //merge

    pub fn can_be_merged(&self, other: &Self) -> bool {
        let (lo, up) = match self.bounds(){
            None => return true,
            Some(a) => a
        };

        let (olo, oup) = match other.bounds() {
            None => return true,
            Some(a) => a
        };
        if lo.val() > oup.val() || (lo.val() == oup.val() && !lo.is_closed() && !oup.is_closed()){
            return false;
        }

        if up.val() < olo.val() || (up.val() == olo.val() && ! up.is_closed() && ! olo.is_closed()){
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
        match &self.imp{
            None => write!(f, "Ø"),
            Some(a)=> {
                let l = if a.loc {'['} else {'('};
                let r = if a.upc {']'} else {')'};
                write!(f, "{}{},{}{}", l, a.lo, a.up, r)
            }
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
        assert!(i.is_lower_closed().is_none());
        assert!(i.is_upper_closed().is_none());
        assert_eq!(i.lower(), None);
        assert_eq!(i.upper(), None);
        assert!(i.is_empty());
    }

    #[test]
    fn test_create_single(){
        let i = Interval::single(5);
        assert_eq!(i.is_lower_closed(), Some(true));
        assert_eq!(i.is_upper_closed(), Some(true));
        assert_eq!(i.lower().unwrap().val(), &5);
        assert_eq!(i.upper().unwrap().val(), &5);
        assert!(!i.is_empty());
    }

    #[test]
    fn test_create_closed(){
        let i = Interval::closed(3,5);
        assert_eq!(i.is_lower_closed(), Some(true));
        assert_eq!(i.is_upper_closed(), Some(true));
        assert_eq!(i.lower().unwrap().val(), &3);
        assert_eq!(i.upper().unwrap().val(), &5);
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
        assert_eq!(i.is_lower_closed(), Some(true));
        assert_eq!(i.is_upper_closed(), Some(false));
        assert_eq!(i.lower().unwrap().val(), &3);
        assert_eq!(i.upper().unwrap().val(), &5);
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
        assert_eq!(i.is_lower_closed(), Some(false));
        assert_eq!(i.is_upper_closed(), Some(true));
        assert_eq!(i.lower().unwrap().val(), &3);
        assert_eq!(i.upper().unwrap().val(), &5);
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

