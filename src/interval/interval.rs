#![allow(dead_code)]

use std::cmp::{Ord, Ordering};
use super::interval_impl::{NonEmptyInterval};
use std::fmt::{Formatter, Display, Result as FmtResult};
use super::bounds::{LowerBound, UpperBound};

use self::Ordering::*;

#[derive(Clone, Debug, Copy, Eq, PartialEq, Hash)]
pub struct Interval<T> where T: Ord {
    pub (super) imp: Option<NonEmptyInterval<T>>
}

impl<T> Interval<T>  where T: Ord  {

//construction and destruction =================================

    pub fn new(lower: T, lower_closed: bool, upper: T, upper_closed: bool) -> Self {
        Self::create_checked(lower, lower_closed, upper, upper_closed)
    }

    pub fn from_bounds(low: LowerBound<T>, up: UpperBound<T>) -> Self {
        unimplemented!()
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
                lo: LowerBound::new(lo, loc),
                up: UpperBound::new(up, upc)
            })
        }
    }

    pub (super) fn create_friendly(lo: T, loc: bool, up: T, upc: bool) -> Self {
        let (lo, up, loc, upc) = if lo < up {
            (lo, up, loc, upc)
        } else {
            (up, lo, upc, loc)
        };

        let imp = if lo == up && (!loc || !upc){
            None
        } else {
            Some(
                NonEmptyInterval {
                    lo: LowerBound::new(lo, loc),
                    up: UpperBound::new(up, upc)
            }
            )
        };
        Self {
            imp
        }
    }

    pub fn single(val: T) -> Self where T: Clone{
        Self{
            imp: Some(NonEmptyInterval{
                lo: LowerBound::new(val.clone(), true),
                up: UpperBound::new(val, true)
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

    pub fn into_tuple(self) -> Option<(T, bool, T, bool)>{
        match self.imp {
            None => None,
            Some(a) => {
                let(lo, loc) = a.lo.into_tuple();
                let (up, upc) = a.up.into_tuple();
                Some((lo, loc, up, upc))
            }
        }
    }

    pub fn into_bounds(self) -> Option<(LowerBound<T>, UpperBound<T>)> {
        match self.imp {
            None => None,
            Some(a) => Some((a.lo, a.up))
        }
    }

//accessors ===================================

    pub fn upper(&self) -> Option<&UpperBound<T>> {
        match &self.imp{
            None => None,
            Some(a) => Some(&a.up)
        }
    }

    pub fn lower(&self) -> Option<&LowerBound<T>> {
        match &self.imp{
            None => None,
            Some(a) => Some(&a.lo)
        }
    }

    pub fn bounds(&self) -> Option<(&LowerBound<T>, &UpperBound<T>)> {
        match &self.imp{
            None => None,
            Some(a) => Some((
                &a.lo,
                &a.up
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
            Some(a) => Some(a.lo.is_closed())
        }
    }

    pub fn is_upper_closed(&self) -> Option<bool> {
        match &self.imp{
           None => None,
            Some(a) => Some(a.up.is_closed())
        }
    }

//operations ============================================

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

        l <= ol && u >= ou
    }


    pub fn can_be_merged(&self, other: &Self) -> bool {
        let (lo, up) = match self.bounds(){
            None => return true,
            Some(a) => a
        };

        let (olo, oup) = match other.bounds() {
            None => return true,
            Some(a) => a
        };
        !(up.is_separated_from(&olo) || oup.is_separated_from(&lo))
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

        let (l1, u1) = self.into_bounds().unwrap();
        let (l2, u2) = other.into_bounds().unwrap();

        Ok(Self::from_bounds(l1.min(l2), u1.max(u2)))
    }

    pub fn merge(&self, other: &Self) -> Result<Self, ()> where T:Clone {
        let (l1, u1) = match self.bounds() {
            None => return Ok(other.clone()),
            Some(a) => a
        };
        let (l1, u2) =  match other.bounds() {
            None => return Ok(self.clone()),
            Some(a) => a
        };


        unimplemented!()
    }


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
                let l = if a.lo.is_closed() {'['} else {'('};
                let r = if a.up.is_closed() {']'} else {')'};
                write!(f, "{}{},{}{}", l, a.lo.val(), a.up.val(), r)
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

