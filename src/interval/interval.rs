use std::cmp::{Ord};
use std::fmt::{Formatter, Display, Result as FmtResult};
use super::bounds::{LowerBound, UpperBound};
use std::mem::swap;

/*
Non empty interval - For internal usage only
Non empty intervals can be converted into empty intervals during mathematical operations
For example (2,3) /2 = empty in the i32 domain
This is why this structure cannot be directly accessible to users.
*/
#[derive(Clone, Debug, Copy, Eq, PartialEq, Hash)]
pub struct NonEmptyInterval<T> where T: Ord {
    pub lo: LowerBound<T>,
    pub up: UpperBound<T>
}


/**
Mathematical interval.

Implementation of mathematical intervals that suppors any ordered types.
It supports common interval operations such as merging, intersection and span.
It also supports mathematical and comparison operations.

# Example
```
use advanced_collections::interval::Interval;
fn main() {
    //there are seveal ways to create an interval:
    let e:Interval<i32> = Interval::empty();
    let mut o = Interval::open(1,3);
    let c = Interval::closed(2,7);

    //Intervals support containment checks
    assert!(o.contains_val(&2));
    assert!(!o.contains_interval(&c));

    //but also span, merge and intersection
    o.merge(c).unwrap();
    assert_eq!(o, Interval::upper_closed(1,7));

    o.intersection(Interval::open(0,4));
    assert_eq!(o, Interval::open(1,4));

    o.span(Interval::open(7,8));
    assert_eq!(o, Interval::open(1,8));

    //intervals support common comparison operations.
    //behavior was modeled after the C++ boost.org ```interval``` library.

    //one interval is greater than other when all points belonging to it
    //are greater than those from the other interval
    let a = Interval::open(1,3);
    let b = Interval::closed(3,5);
    assert!(a<b);

    //one interval is greater or equal if there is at most one point that is common
    let a = Interval::closed(1,3);
    assert!(a<=b);

    //interval are equal if their bonds are equal
    assert_eq!(Interval::open(1,3), Interval::open(1,3));

    //finally, intervals support common mathematical operations
    let mut a = Interval::open(1,3);
    a += 1;
    assert_eq!(a, Interval::open(2,4));
    assert_eq!(a*2, Interval::open(4,8));
}
```
*/
#[derive(Clone, Debug, Copy, Eq, PartialEq, Hash)]
pub struct Interval<T> where T: Ord {
    pub (super) imp: Option<NonEmptyInterval<T>>
}

impl<T> Interval<T>  where T: Ord  {

//construction and destruction ====================================================================
    /**
    Creates a new non-empty interval from primitive types.

    Panics if the lower bound is greater than the lower bound
    or if the interval is empty.

    # Example
    ```
    use advanced_collections::interval::Interval;
    fn main() {
        let i = Interval::new(3,false, 5, true);
        assert_eq!(i, Interval::upper_closed(3,5));
    }
    ```
    */
    pub fn new(lower: T, lower_closed: bool, upper: T, upper_closed: bool) -> Self {
        Self::create_checked(lower, lower_closed, upper, upper_closed)
    }

    /**
    Creates a new non-empty interval from lower and upper bounds.

    Panics if the lower bound is greater than the lower bound
    or if the interval is empty.

    # Example

    ```
    use advanced_collections::interval::{Interval, LowerBound, UpperBound};
    fn main() {
        let l = LowerBound::new(3,false);
        let u = UpperBound::new(5,true);
        let i = Interval::from_bounds(l, u);
        assert_eq!(i, Interval::upper_closed(3,5));
    }
    ```
    */
    pub fn from_bounds(lo: LowerBound<T>, up: UpperBound<T>) -> Self {
        let (l, lc) = lo.into_tuple();
        let (u, uc) = up.into_tuple();
        Self::create_checked(l, lc, u, uc)
    }

    ///Create a new interval, panics if the provided data is invalid.
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

    ///Creates a new interval from provided data, reverses the interval or converts into empty
    /// if data is invalid.
    pub (super) fn create_friendly(lo: T, loc: bool, up: T, upc: bool) -> Self {
        let mut i = Self {
            imp: Some( NonEmptyInterval {
                lo: LowerBound::new(lo, loc),
                up: UpperBound::new(up, upc)
        })};
        i.fix_after_modification();
        i
    }

    /**
    Creates an interval that contains only a single value.

    # Example
    ```
    use advanced_collections::interval::Interval;
    fn main() {
        let i = Interval::single(3);
        assert_eq!(i, Interval::closed(3,3));
    }
    ```
    */
    pub fn single(val: T) -> Self where T: Clone{
        Self{
            imp: Some(NonEmptyInterval{
                lo: LowerBound::new(val.clone(), true),
                up: UpperBound::new(val, true)
            })
        }
    }

    /**
    Shortcut for creating an interval with open bounds.

    # Example
    ```
    use advanced_collections::interval::Interval;
    fn main() {
        let i = Interval::open(3, 5);
        assert_eq!(i.lower().unwrap().val(), &3);
        assert!(!i.lower().unwrap().is_closed());
        assert_eq!(i.upper().unwrap().val(), &5);
        assert!(!i.upper().unwrap().is_closed());
    }
    ```
    */
    pub fn open(low: T, up: T) -> Self {
        Self::create_checked(low, false, up, false)
    }

    /**
    Shortcut for creating an interval with closed bounds.

    # Example
    ```
    use advanced_collections::interval::Interval;
    fn main() {
        let i = Interval::closed(3, 5);
        assert_eq!(i.lower().unwrap().val(), &3);
        assert!(i.lower().unwrap().is_closed());
        assert_eq!(i.upper().unwrap().val(), &5);
        assert!(i.upper().unwrap().is_closed());
    }
    ```
    */
    pub fn closed(low: T, up: T) -> Self {
        Self::create_checked(low, true, up, true)
    }

    /**
    Shortcut for creating an interval with the lower bound open and the upper one closed.

    # Example
    ```
    use advanced_collections::interval::Interval;
    fn main() {
        let i = Interval::lower_closed(3, 5);
        assert_eq!(i.lower().unwrap().val(), &3);
        assert!(i.lower().unwrap().is_closed());
        assert_eq!(i.upper().unwrap().val(), &5);
        assert!(!i.upper().unwrap().is_closed());
    }
    ```
    */
    pub fn lower_closed(low: T, up: T) -> Self {
        Self::create_checked(low, true, up, false)
    }

    /**
    Shortcut for creating an interval with the lower bound closed and the upper one open.

    # Example
    ```
    use advanced_collections::interval::Interval;
    fn main() {
        let i = Interval::upper_closed(3, 5);
        assert_eq!(i.lower().unwrap().val(), &3);
        assert!(!i.lower().unwrap().is_closed());
        assert_eq!(i.upper().unwrap().val(), &5);
        assert!(i.upper().unwrap().is_closed());
    }
    ```
    */
    pub fn upper_closed(low: T, up: T) -> Self {
        Self::create_checked(low, false, up, true)
    }

    /**
    Creates an empty interval.

    # Example

    ```
    use advanced_collections::interval::Interval;
    fn main() {
        let i:Interval<i32> = Interval::empty();
        assert!(i.is_empty());
    }
    ```
    */
    pub fn empty() -> Self {
        Self {
            imp: None
        }
    }

    /**
    Destructs the interval and converts it into a tuple with primitive types.

    Returns ```None``` if the interval is empty.

     # Example

    ```
    use advanced_collections::interval::Interval;
    fn main() {
        let i = Interval::open(3,5);
        let t = i.into_tuple();
        assert_eq!(t, Some((3,false,5,false)));
    }
    ```
    */
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

    /**
    Destructs the interval and converts it into a pair of bounds.

    Returns ```None``` if the interval is empty.

     # Example

    ```
    use advanced_collections::interval::{Interval, LowerBound, UpperBound};
    fn main() {
        let i = Interval::open(3,5);
        let t = i.into_bounds();
        let l = LowerBound::new(3,false);
        let u = UpperBound::new(5,false);
        assert_eq!(t, Some((l, u)));
    }
    ```
    */
    pub fn into_bounds(self) -> Option<(LowerBound<T>, UpperBound<T>)> {
        match self.imp {
            None => None,
            Some(a) => Some((a.lo, a.up))
        }
    }

    pub (super) fn fix_after_modification(&mut self){
        let mut set_empty = false;
        if let Some(ref mut a) = self.imp {
            if a.lo.val() > a.up.val() {
                a.lo.swap(&mut a.up)
            }
            if a.lo.val() == a.up.val() && (!a.lo.is_closed() || !a.up.is_closed()){
                set_empty = true;
            }
        }
        if set_empty {
            self.imp = None;
        }
    }

//accessors =======================================================================================
    /**
    Get the upper bound of the interval.

    Returns ```None``` if the interval is empty.

    # Example

    ```
    use advanced_collections::interval::{Interval, UpperBound};
    fn main() {
       let i = Interval::open(3,5);
       let u = i.upper();;
       assert_eq!(u, Some(&UpperBound::new(5, false)));
    }
    ```
    */
    pub fn upper(&self) -> Option<&UpperBound<T>> {
        match &self.imp{
            None => None,
            Some(a) => Some(&a.up)
        }
    }

    /**
    Get the lower bound of the interval.

    Returns ```None``` if the interval is empty.

    # Example

    ```
    use advanced_collections::interval::{Interval, LowerBound};
    fn main() {
       let i = Interval::open(3,5);
       let l = i.lower();;
       assert_eq!(l, Some(&LowerBound::new(3, false)));
    }
    ```
    */
    pub fn lower(&self) -> Option<&LowerBound<T>> {
        match &self.imp{
            None => None,
            Some(a) => Some(&a.lo)
        }
    }

    /**
    Get references to both bounds of the interval.

    Returns ```None``` if the interval is empty.

    # Example

    ```
    use advanced_collections::interval::{Interval, LowerBound, UpperBound};
    fn main() {
       let i = Interval::open(3,5);
       let u = i.lower();
       let l = LowerBound::new(3, false);
       let u = UpperBound::new(5, false);
       assert_eq!(i.bounds(), Some((&l, &u)));
    }
    ```
    */
    pub fn bounds(&self) -> Option<(&LowerBound<T>, &UpperBound<T>)> {
        match &self.imp{
            None => None,
            Some(a) => Some((
                &a.lo,
                &a.up
            ))
        }
    }

    /**
    Checks if the interval is empty.

     # Example

    ```
    use advanced_collections::interval::Interval;
    fn main() {
        let i:Interval<i32> = Interval::empty();
        assert!(i.is_empty());
    }
    ```
    */
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.imp.is_none()
    }

    /**
    Checks if the interval is just a single value.

    # Example

    ```
    use advanced_collections::interval::Interval;
    fn main() {
        let i = Interval::closed(5,5);
        assert!(i.is_single());
    }
    ```
    */
    pub fn is_single(&self) -> bool {
        if let Some(ref a) = self.imp {
            a.lo.val() == a.up.val()
        } else {
            false
        }
    }

    /**
    Checks if the lower bound of an interval is closed.

    Returns ```None``` if the interval is empty.

    # Example

    ```
    use advanced_collections::interval::Interval;
    fn main() {
       let i = Interval::closed(3,5);
       assert_eq!(i.is_lower_closed(), Some(true));
    }
    ```
    */
    pub fn is_lower_closed(&self) -> Option<bool> {
        match &self.imp{
            None => None,
            Some(a) => Some(a.lo.is_closed())
        }
    }

    /**
    Checks if the upper bound of an interval is closed.

    Returns ```None``` if the interval is empty.

    # Example

    ```
    use advanced_collections::interval::Interval;
    fn main() {
       let i = Interval::closed(3,5);
       assert_eq!(i.is_upper_closed(), Some(true));
    }
    ```
    */
    pub fn is_upper_closed(&self) -> Option<bool> {
        match &self.imp{
           None => None,
            Some(a) => Some(a.up.is_closed())
        }
    }

//operations ============================================

//contains
    /**
    Checks if an interval contains the given value.

    # Example

    ```
    use advanced_collections::interval::Interval;
    fn main() {
       let i = Interval::closed(3,5);
       assert!(i.contains_val(&4));
    }
    ```
    */
    pub fn contains_val(&self, val: &T) -> bool {
        !(self > val || self < val)
    }

    /**
    Checks if an interval contains another interval.

    # Example

    ```
    use advanced_collections::interval::Interval;
    fn main() {
       let a = Interval::closed(3,7);
       let b = Interval::closed(4,6);
       assert!(a.contains_interval(&b));
    }
    ```
    */
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

//merge
    /**
    Checks if two intervals can be merged into one.

    # Example

    ```
    use advanced_collections::interval::Interval;
    fn main() {
       let a = Interval::closed(2,4);
       let b = Interval::closed(4,6);
       assert!(a.can_be_merged(&b));
    }
    ```
    */
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

    /**
    Merges two intervals into one.

    If the merge is unsuccessful, two original intervals are returned.

    # Example

    ```
    use advanced_collections::interval::Interval;
    fn main() {
       let a = Interval::closed(2,4);
       let b = Interval::closed(4,6);
       assert_eq!(a.into_merged(b), Ok(Interval::closed(2,6)));
    }
    ```
    */
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

    /**
    Merges an interval with another one.

    # Example

    ```
    use advanced_collections::interval::Interval;
    fn main() {
       let mut a = Interval::closed(2,4);
       let b = Interval::closed(4,6);
       a.merge(b).unwrap();
       assert_eq!(a, Interval::closed(2,6));
    }
    ```
    */
    pub fn merge(&mut self, mut other: Self) -> Result<(), Self>{
        if ! self.can_be_merged(&other) {
            return Err(other)
        }
        let s = match self.imp {
            None => {
                swap(self, &mut other);
                return Ok(());
            },
            Some(ref mut a) => a
        };
        let o =  match other.imp {
            None => return Ok(()),
            Some(ref mut a) => a
        };

        if o.lo <s.lo {
            swap(&mut o.lo, &mut s.lo)
        }
        if o.up > s.up {
            swap(&mut o.up, &mut s.up)
        }
        Ok(())
    }

//intersection
    /**
    Checks if two intervals intersect.

    # Example

    ```
    use advanced_collections::interval::Interval;
    fn main() {
       let a = Interval::closed(2,4);
       let b = Interval::closed(4,6);
       //4 belongs to both a and b
       assert!(a.intersects(&b));
    }
    ```
    */
    pub fn intersects(&self, other: &Self) -> bool {
        !(self > other || self < other)
    }

    /**
    Converts two intervals into their intersection.

    # Example

    ```
    use advanced_collections::interval::Interval;
    fn main() {
       let a = Interval::closed(2,5);
       let b = Interval::closed(3,6);
       assert_eq!(a.into_intersection(b), Interval::closed(3,5));
    }
    ```
    */
    pub fn into_intersection(self, other: Self) -> Self {
        if self.is_empty() {
            return self;
        }
        if other.is_empty(){
            return other;
        }

        if ! self.intersects(&other) {
            return Self::empty();
        }
        let (l1, u1) =  self.into_bounds().unwrap();
        let (l2, u2) = other.into_bounds().unwrap();
        Self::from_bounds(l1.max(l2), u1.min(u2))
    }

    /**
    Makes an interval an intersection of it an another one.

    # Example

    ```
    use advanced_collections::interval::Interval;
    fn main() {
       let mut a = Interval::closed(2,5);
       let b = Interval::closed(3,6);
       a.intersection(b);
       assert_eq!(a, Interval::closed(3,5));
    }
    ```
    */
    pub fn intersection(&mut self, other: Self) {
        if self.is_empty() || other.is_empty() || *self>other || *self < other {
            self.imp = None;
            return;
        }
        let s = self.imp.as_mut().unwrap();
        let mut o = other.imp.unwrap();

        if s.lo < o.lo {
            swap(&mut s.lo, &mut o.lo)
        }

        if s.up > o.up {
            swap(&mut s.up, &mut o.up)
        }

    }

    //span
    /**
    Converts two intervals into one that spans both of them.

    # Example

    ```
    use advanced_collections::interval::Interval;
    fn main() {
       let a = Interval::closed(2,3);
       let b = Interval::closed(5,6);
       assert_eq!(a.into_span(b), Interval::closed(2,6));
    }
    ```
    */
    pub fn into_span(self, other: Self) -> Self {
        if other.is_empty(){
            return self;
        }
        if self.is_empty(){
            return other;
        }
        let (l1, u1) =  self.into_bounds().unwrap();
        let (l2, u2) = other.into_bounds().unwrap();

        Self::from_bounds(l1.min(l2), u1.max(u2))
    }

    /**
    Extends an interval to span it and another one.

    # Example

    ```
    use advanced_collections::interval::Interval;
    fn main() {
       let mut a = Interval::closed(2,3);
       let b = Interval::closed(5,6);
       a.span(b);
       assert_eq!(a, Interval::closed(2,6));
    }
    ```
    */
    pub fn span(&mut self, mut other: Self) {
        if other.is_empty() {
            return;
        }

        if self.is_empty() {
            swap(self, &mut other);
            return;
        }

        let s = self.imp.as_mut().unwrap();
        let o = other .imp.as_mut().unwrap();

        if o.lo < s.lo {
            swap(&mut o.lo, &mut s.lo);
        }
        if o.up > s.up {
            swap(&mut s.up, &mut o.up);
        }
    }
}

/**
Displays an interval in the form of [2,3).

# Example

```
use advanced_collections::interval::Interval;
fn main() {
   let mut a = Interval::lower_closed(2,3);
   let d = format!("{}", &a);
   assert_eq!(d, "[2,3)");
}
```
*/
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

    #[test]
    fn test_can_be_merged(){
        assert!(Interval::open(4,7).can_be_merged(&Interval::open(5, 9)));
        assert!(Interval::open(4,7).can_be_merged(&Interval::closed(7, 9)));
        assert!(Interval::closed(4,7).can_be_merged(&Interval::open(7, 9)));
        assert!(!Interval::open(4,7).can_be_merged(&Interval::open(7, 9)));
        assert!(!Interval::open(4,7).can_be_merged(&Interval::open(8, 9)));

        assert!(Interval::open(4,7).can_be_merged(&Interval::open(2, 6)));
        assert!(Interval::open(4,7).can_be_merged(&Interval::closed(2, 4)));
        assert!(Interval::closed(4,7).can_be_merged(&Interval::open(2, 4)));
        assert!(!Interval::open(4,7).can_be_merged(&Interval::open(2, 4)));
        assert!(!Interval::open(4,7).can_be_merged(&Interval::open(2, 3)));
    }

    #[test]
    fn test_into_merged(){
        assert_eq!(Interval::closed(3,4).into_merged(Interval::closed(4,5)), Ok(Interval::closed(3,5)));
        assert_eq!(Interval::open(3,4).into_merged(Interval::closed(4,5)), Ok(Interval::upper_closed(3,5)));
        assert_eq!(Interval::closed(3,4).into_merged(Interval::open(4,5)), Ok(Interval::lower_closed(3,5)));
        assert_eq!(Interval::open(3,4).into_merged(Interval::open(4,5)), Err((Interval::open(3,4), Interval::open(4,5))));
    }

    #[test]
    fn test_merge(){
        let mut i = Interval::empty();
        assert_eq!(i.merge(Interval::open(3,5)), Ok(()));
        assert_eq!(i, Interval::open(3,5));

        assert_eq!(i.merge(Interval::closed(5, 8)), Ok(()));
        assert_eq!(i, Interval::upper_closed(3,8));

        assert_eq!(i.merge(Interval::closed(1,2)), Err(Interval::closed(1,2)));
        assert_eq!(i, Interval::upper_closed(3,8));

        assert_eq!(i.merge(Interval::empty()), Ok(()));
        assert_eq!(i, Interval::upper_closed(3,8));
    }

    #[test]
    fn test_span(){
        let mut i = Interval::empty();

        i.span(Interval::open(3,5));
        assert_eq!(i, Interval::open(3,5));

        i.span(Interval::empty());
        assert_eq!(i, Interval::open(3,5));

        i.span(Interval::closed(3,5));
        assert_eq!(i, Interval::closed(3,5));

        i.span(Interval::closed(7,9));
        assert_eq!(i, Interval::closed(3,9));
    }

    #[test]
    fn test_into_span(){
        assert_eq!(Interval::empty().into_span(Interval::open(3,5)), Interval::open(3,5));

        assert_eq!(Interval::open(3,5).into_span(Interval::open(3,5)), Interval::open(3,5));

        assert_eq!(Interval::open(3,5).into_span(Interval::closed(3,5)), Interval::closed(3,5));

        assert_eq!(Interval::closed(3,5).into_span(Interval::closed(7,9)), Interval::closed(3,9));
    }

    #[test]
    fn test_intersection(){
        let mut i = Interval::open(3,9);

        i.intersection(Interval::closed(1,8));
        assert_eq!(i, Interval::upper_closed(3,8));

        i.intersection(Interval::lower_closed(4, 7));
        assert_eq!(i, Interval::lower_closed(4,7));

        i.intersection(i.clone());
        assert_eq!(i, Interval::lower_closed(4,7));

        i.intersection(Interval::empty());
        assert_eq!(i, Interval::empty());
    }

    #[test]
    fn test_into_intersection(){
        assert_eq!(Interval::open(3,9).into_intersection(Interval::closed(1,8)), Interval::upper_closed(3,8));
        assert_eq!(Interval::upper_closed(3,8).into_intersection(Interval::lower_closed(4, 7)), Interval::lower_closed(4,7));
        assert_eq!(Interval::lower_closed(4,7).into_intersection(Interval::lower_closed(4,7)), Interval::lower_closed(4,7));
        assert_eq!(Interval::lower_closed(4,7).into_intersection(Interval::empty()), Interval::empty());
    }

}

