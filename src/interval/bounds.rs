use std::cmp::Ordering;
use self::Ordering::*;
use std::mem::swap;

use std::ops::{Add, AddAssign, Sub, SubAssign,  Mul, MulAssign, Div, DivAssign};


// Bound ==========================================================================================

///Represent the common part of LowerBound and UpperBound, not intended to be used directly.
#[derive(Clone, Debug, Copy, Eq, PartialEq, Hash)]
pub struct Bound<T> where T: Ord {
    val: T,
    is_closed: bool
}

impl <T> Bound<T> where T: Ord {


    ///Returns value of the given bound.
    #[inline]
    pub fn val(&self) -> &T {
        &self.val
    }


    ///Indicates if the given bound is closed.
    #[inline]
    pub fn is_closed(&self) -> bool {
        self.is_closed
    }

    ///Creates a new instance
    pub fn new(val: T, is_closed: bool) -> Self {
        Self {
            val,
            is_closed
        }
    }

    ///A destructor that converts Bound into primitive types
    pub fn into_tuple(self) -> (T, bool) {
        (self.val, self.is_closed)
    }
}

/*Math operations ---------------------------------------------------------------------------------
Bound has a set of mathematical operations
that are common to both LowerBound and UpperBound
*/
impl<T, U> Add<U> for Bound<T> where T: Ord + Add<U, Output=T> {
    type Output = Bound<T>;

    fn add(self, rhs: U) -> <Self as Add<U>>::Output {
        Bound {
            val: self.val + rhs,
            is_closed: self.is_closed
        }
    }
}

impl<T, U> Sub<U> for Bound<T> where T: Ord + Sub<U, Output=T> {
    type Output = Bound<T>;

    fn sub(self, rhs: U) -> <Self as Sub<U>>::Output {
        Bound {
            val: self.val - rhs,
            is_closed: self.is_closed
        }
    }
}

impl<T, U> Mul<U> for Bound<T> where T: Ord + Mul<U, Output=T> {
    type Output = Bound<T>;

    fn mul(self, rhs: U) -> <Self as Mul<U>>::Output {
        Bound {
            val: self.val * rhs,
            is_closed: self.is_closed
        }
    }
}

impl<T, U> Div<U> for Bound<T> where T: Ord + Div<U, Output=T> {
    type Output = Bound<T>;

    fn div(self, rhs: U) -> <Self as Div<U>>::Output {
        Bound {
            val: self.val / rhs,
            is_closed: self.is_closed
        }
    }
}

impl<T, U> AddAssign<U> for Bound<T> where T:Ord + AddAssign<U> {
    fn add_assign(&mut self, rhs: U) {
        self.val += rhs;
    }
}

impl<T, U> SubAssign<U> for Bound<T> where T:Ord + SubAssign<U> {
    fn sub_assign(&mut self, rhs: U) {
        self.val -= rhs;
    }
}

impl<T, U> MulAssign<U> for Bound<T> where T:Ord + MulAssign<U> {
    fn mul_assign(&mut self, rhs: U) {
        self.val *= rhs;
    }
}

impl<T, U> DivAssign<U> for Bound<T> where T:Ord + DivAssign<U> {
    fn div_assign(&mut self, rhs: U) {
        self.val /= rhs;
    }
}

//LowerBound ======================================================================================
/**
Represents the lower bound of an interval.

 # Example

```
use advanced_collections::interval::LowerBound;

fn main(){

    //Lower and upper bounds are different types to prevent mistakes
    //and because they behave in a diffent way in certain situations, for example when comparing.

    //A bound has two features: its value and information if it is closed or not.
    let mut l = LowerBound::new(5, true);
    assert!(l.is_closed());
    assert_eq!(l.val(), &5);

    //Bounds support common comparison operations

    assert!(l < LowerBound::new(5, false));

    //and common mathematical operations

    l += 3;
    assert_eq!(l, LowerBound::new(8, true));
}
```
*/
#[derive(Clone, Debug, Copy, Eq, PartialEq, Hash)]
pub struct LowerBound<T> where T: Ord {
    pub(self) bound: Bound<T>
}

impl <T> LowerBound<T> where T: Ord {

    /**
    Checks if there is some space between bounds.

     # Example

    ```
    use advanced_collections::interval::{LowerBound, UpperBound};

    fn main(){
        let l = LowerBound::new(5, true);
        assert!(l.is_separated_from(&UpperBound::new(3, true)));
        assert!(!l.is_separated_from(&UpperBound::new(5, true)));
        assert!(!l.is_separated_from(&UpperBound::new(7, true)));
    }
    ```
    */
    pub fn is_separated_from(&self, other: &UpperBound<T>) -> bool {
        are_separated(self, other)
    }


    /**
    Creates a new lower bound.

     # Example

    ```
    use advanced_collections::interval::LowerBound;

    fn main(){
        //A bound has two features: its value and information if it is closed or not.
        let mut l = LowerBound::new(5, true);
        assert!(l.is_closed());
        assert_eq!(l.val(), &5);
    }
    ```
    */
    pub fn new(val: T, is_closed: bool) -> Self {
        Self {
            bound: Bound::new(val, is_closed)
        }
    }

    /**
    Returns the bound value.

    # Example

    ```
    use advanced_collections::interval::LowerBound;

    fn main(){
        let mut l = LowerBound::new(5, true);
        assert_eq!(l.val(), &5);
    }
    ```
    */
    pub fn val(&self) -> &T{
        &self.bound.val()
    }

    /**
    Indicates if the given bound is closed.

    # Example

    ```
    use advanced_collections::interval::LowerBound;

    fn main(){
        let l = LowerBound::new(5, true);
        assert!(l.is_closed());
    }
    ```
    */
    pub fn is_closed(&self) -> bool {
        self.bound.is_closed()
    }

    pub(super) fn swap(&mut self, other: &mut UpperBound<T>) {
        swap(&mut self.bound, &mut other.bound)
    }

    /**
    Destroys bound and coverts it into primitive types.

    # Example

    ```
    use advanced_collections::interval::LowerBound;

    fn main(){
        let l = LowerBound::new(5, true);
        let (v, c) = l.into_tuple();
        assert_eq!(v, 5);
        assert_eq!(c, true);
    }
    ```
    */
    pub fn into_tuple(self) -> (T, bool) {
        self.bound.into_tuple()
    }
}

/*Comparison operators ----------------------------------------------------------------------------
LowerBound support comparing with itself, UpperBound and a single value

The behavior is modeled after C++ boost.org interval library.
*/

impl<T> Ord for LowerBound<T> where T: Ord {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.val().cmp(other.val()) {
            Greater => Greater,
            Less => Less,
            Equal => {
                if self.is_closed() == other.is_closed() {
                    Equal
                } else {
                    if self.is_closed() {
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
        match self.val().cmp(other.val()) {
            Greater => Some(Greater),
            Less => Some(Less),
            Equal => {
                if self.is_closed() == other.is_closed() {
                    Some(Equal)
                } else {
                    if self.is_closed() {
                        Some(Less)
                    } else {
                        Some(Greater)
                    }
                }
            }
        }
    }
}

impl<T> PartialOrd<UpperBound<T>> for LowerBound<T> where T: Ord {
    fn partial_cmp(&self, other: &UpperBound<T>) -> Option<Ordering> {
        if self < other {
            return Some(Less);
        }
        if self > other {
            return Some(Greater);
        }
        None
    }

    fn lt(&self, other: &UpperBound<T>) -> bool {
        match self.val().cmp(&other.val()) {
            Greater => false,
            Less => true,
            Equal => !self.is_closed() || !other.is_closed()
        }
    }

    fn le(&self, other: &UpperBound<T>) -> bool {
        self.val() <= other.val()
    }

    fn gt(&self, other: &UpperBound<T>) -> bool {
        match self.val().cmp(&other.val()) {
            Greater => true,
            Less => false,
            Equal => !self.is_closed() || !other.is_closed()
        }
    }

    fn ge(&self, other: &UpperBound<T>) -> bool {
        self.val() >= other.val()
    }
}

impl <T> PartialEq<UpperBound<T>> for LowerBound<T> where T: Ord {
    fn eq(&self, _other: &UpperBound<T>) -> bool {
        false
    }

    fn ne(&self, _other: &UpperBound<T>) -> bool {
        false
    }
}


impl <T> PartialOrd<T> for LowerBound<T> where T: Ord {
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        match self.val().cmp(other) {
            Greater => Some(Greater),
            Less => Some(Less),
            Equal => if self.is_closed() {
                Some(Equal)
            } else {
                Some(Greater)
            }
        }
    }

    fn lt(&self, other: &T) -> bool {
        self.val() < other
    }

    fn le(&self, other: &T) -> bool {
        self.val() < other || self.val() == other && self.is_closed()
    }

    fn gt(&self, other: &T) -> bool {
        self.val() > other || self.val() == other && !self.is_closed()
    }

    fn ge(&self, other: &T) -> bool {
        self.val() >= other
    }
}

impl<T> PartialEq<T> for LowerBound<T> where T: Ord {
    fn eq(&self, other: &T) -> bool {
        self.val() == other && self.is_closed()
    }
}

/*Mathematical operations -------------------------------------------------------------------------
LowerBound supports all common mathematical operations.
*/

impl<T, U> Add<U> for LowerBound<T> where T: Ord + Add<U, Output=T> {
    type Output = Self;

    fn add(self, rhs: U) -> <Self as Add<U>>::Output {
        Self{
            bound: self.bound + rhs
        }
    }
}

impl<T, U> AddAssign<U> for LowerBound<T> where T: Ord + AddAssign<U> {
    fn add_assign(&mut self, rhs: U) {
        self.bound += rhs;
    }
}

impl<T, U> Sub<U> for LowerBound<T> where T: Ord + Sub<U, Output=T> {
    type Output = Self;

    fn sub(self, rhs: U) -> <Self as Sub<U>>::Output {
        Self{
            bound: self.bound - rhs
        }
    }
}

impl<T, U> SubAssign<U> for LowerBound<T> where T: Ord + SubAssign<U> {
    fn sub_assign(&mut self, rhs: U) {
        self.bound -= rhs;
    }
}

impl<T, U> Mul<U> for LowerBound<T> where T: Ord + Mul<U, Output=T> {
    type Output = Self;

    fn mul(self, rhs: U) -> <Self as Mul<U>>::Output {
        Self{
            bound: self.bound * rhs
        }
    }
}

impl<T, U> MulAssign<U> for LowerBound<T> where T: Ord + MulAssign<U> {
    fn mul_assign(&mut self, rhs: U) {
        self.bound *= rhs;
    }
}

impl<T, U> Div<U> for LowerBound<T> where T: Ord + Div<U, Output=T> {
    type Output = Self;

    fn div(self, rhs: U) -> <Self as Div<U>>::Output {
        Self{
            bound: self.bound / rhs
        }
    }
}

impl<T, U> DivAssign<U> for LowerBound<T> where T: Ord + DivAssign<U> {
    fn div_assign(&mut self, rhs: U) {
        self.bound /= rhs;
    }
}


//UpperBound ======================================================================================
/**
Represents the upper bound of an interval.

 # Example

```
use advanced_collections::interval::UpperBound;

fn main(){

    //Lower and upper bounds are different types to prevent mistakes
    //and because they behave in a diffent way in certain situations, for example when comparing.

    //A bound has two features: its value and information if it is closed or not.
    let mut u = UpperBound::new(5, true);
    assert!(u.is_closed());
    assert_eq!(u.val(), &5);

    //Bounds support common comparison operations

    assert!(u > UpperBound::new(5, false));

    //and common mathematical operations

    u += 3;
    assert_eq!(u, UpperBound::new(8, true));
}
```
*/
#[derive(Clone, Debug, Copy, Eq, PartialEq, Hash)]
pub struct UpperBound<T> where T: Ord {
    bound: Bound<T>
}

impl <T> UpperBound<T> where T: Ord {

    /**
    Checks if there is some space between bounds.

     # Example

    ```
    use advanced_collections::interval::{LowerBound, UpperBound};

    fn main(){
        let u = UpperBound::new(5, true);
        assert!(u.is_separated_from(&LowerBound::new(7, true)));
        assert!(!u.is_separated_from(&LowerBound::new(5, true)));
        assert!(!u.is_separated_from(&LowerBound::new(3, true)));
    }
    ```
    */
    pub fn is_separated_from(&self, other: &LowerBound<T>) -> bool {
        are_separated(other, self)
    }

    pub fn val(&self) -> &T{
        &self.bound.val()
    }

    /**
    Indicates if the given bound is closed.

    # Example

    ```
    use advanced_collections::interval::UpperBound;

    fn main(){
        let u = UpperBound::new(5, true);
        assert!(u.is_closed());
    }
    ```
    */
    pub fn is_closed(&self) -> bool {
        self.bound.is_closed()
    }

    /**
    Creates a new upper bound.

     # Example

    ```
    use advanced_collections::interval::UpperBound;

    fn main(){
        //A bound has two features: its value and information if it is closed or not.
        let mut u = UpperBound::new(5, true);
        assert!(u.is_closed());
        assert_eq!(u.val(), &5);
    }
    ```
    */
    pub fn new(val: T, is_closed: bool) -> Self{
        Self {
            bound: Bound::new(val, is_closed)
        }
    }

    /**
    Destroys bound and coverts it into primitive types.

    # Example

    ```
    use advanced_collections::interval::LowerBound;

    fn main(){
        let l = LowerBound::new(5, true);
        let (v, c) = l.into_tuple();
        assert_eq!(v, 5);
        assert_eq!(c, true);
    }
    ```
    */
    pub fn into_tuple(self) -> (T, bool) {
        self.bound.into_tuple()
    }
}


impl<T> Ord for UpperBound<T> where T: Ord {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.val().cmp(&other.val()) {
            Greater => Greater,
            Less => Less,
            Equal => {
                if self.is_closed() == other.is_closed() {
                    Equal
                } else {
                    if self.is_closed() {
                        Greater
                    } else {
                        Less
                    }
                }
            }
        }
    }
}

/*Comparison operators ----------------------------------------------------------------------------
LowerBound support comparing with itself, UpperBound and a single value

The behavior is modeled after C++ boost.org interval library.
*/

impl<T> PartialOrd for UpperBound<T> where T: Ord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.val().cmp(&other.val()) {
            Greater => Some(Greater),
            Less => Some(Less),
            Equal => {
                if self.is_closed() == other.is_closed() {
                    Some(Equal)
                } else {
                    if self.is_closed() {
                        Some(Greater)
                    } else {
                        Some(Less)
                    }
                }
            }
        }
    }
}

impl<T> PartialOrd<LowerBound<T>> for UpperBound<T> where T: Ord {
    fn partial_cmp(&self, other: &LowerBound<T>) -> Option<Ordering> {
        if self < other {
            return Some(Less);
        }
        if self > other {
            return Some(Greater);
        }
        None
    }

    fn lt(&self, other: &LowerBound<T>) -> bool {
        match self.val().cmp(&other.val()) {
            Greater => false,
            Less => true,
            Equal => !self.is_closed() || !other.is_closed()
        }
    }

    fn le(&self, other: &LowerBound<T>) -> bool {
        self.val() <= other.val()
    }

    fn gt(&self, other: &LowerBound<T>) -> bool {
        match self.val().cmp(other.val()) {
            Greater => true,
            Less => false,
            Equal => !self.is_closed() || !other.is_closed()
        }
    }

    fn ge(&self, other: &LowerBound<T>) -> bool {
        self.val() >= other.val()
    }
}

impl <T> PartialEq<LowerBound<T>> for UpperBound<T> where T: Ord {
    fn eq(&self, _other: &LowerBound<T>) -> bool {
        false
    }

    fn ne(&self, _other: &LowerBound<T>) -> bool {
        false
    }
}

impl <T> PartialOrd<T> for UpperBound<T> where T: Ord {
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        match self.val().cmp(other) {
            Greater => Some(Greater),
            Less => Some(Less),
            Equal => if self.is_closed() {
                Some(Equal)
            } else {
                Some(Less)
            }
        }
    }

    fn lt(&self, other: &T) -> bool {
        self.val() < other || self.val() == other && !self.is_closed()
    }

    fn le(&self, other: &T) -> bool {
        self.val() <= other
    }

    fn gt(&self, other: &T) -> bool {
        self.val() > other
    }

    fn ge(&self, other: &T) -> bool {
        self.val() > other || self.val() == other && self.is_closed()
    }
}

impl<T> PartialEq<T> for UpperBound<T> where T: Ord {
    fn eq(&self, other: &T) -> bool {
        self.val() == other && self.is_closed()
    }
}

/*Mathematical operations -------------------------------------------------------------------------
UpperBound supports all common mathematical operations.
*/

impl<T, U> Add<U> for UpperBound<T> where T: Ord + Add<U, Output=T> {
    type Output = Self;

    fn add(self, rhs: U) -> <Self as Add<U>>::Output {
        Self{
            bound: self.bound + rhs
        }
    }
}

impl<T, U> AddAssign<U> for UpperBound<T> where T: Ord + AddAssign<U> {
    fn add_assign(&mut self, rhs: U) {
        self.bound += rhs;
    }
}

impl<T, U> Sub<U> for UpperBound<T> where T: Ord + Sub<U, Output=T> {
    type Output = Self;

    fn sub(self, rhs: U) -> <Self as Sub<U>>::Output {
        Self{
            bound: self.bound - rhs
        }
    }
}

impl<T, U> SubAssign<U> for UpperBound<T> where T: Ord + SubAssign<U> {
    fn sub_assign(&mut self, rhs: U) {
        self.bound -= rhs;
    }
}

impl<T, U> Mul<U> for UpperBound<T> where T: Ord + Mul<U, Output=T> {
    type Output = Self;

    fn mul(self, rhs: U) -> <Self as Mul<U>>::Output {
        Self{
            bound: self.bound * rhs
        }
    }
}

impl<T, U> MulAssign<U> for UpperBound<T> where T: Ord + MulAssign<U> {
    fn mul_assign(&mut self, rhs: U) {
        self.bound *= rhs;
    }
}

impl<T, U> Div<U> for UpperBound<T> where T: Ord + Div<U, Output=T> {
    type Output = Self;

    fn div(self, rhs: U) -> <Self as Div<U>>::Output {
        Self{
            bound: self.bound / rhs
        }
    }
}

impl<T, U> DivAssign<U> for UpperBound<T> where T: Ord + DivAssign<U> {
    fn div_assign(&mut self, rhs: U) {
        self.bound /= rhs;
    }
}

// Helpers ========================================================================================

fn  are_separated<T>(l: &LowerBound<T>, u: &UpperBound<T>) -> bool where T: Ord{
    l.val() > u.val() || (u.val() == l.val() && !u.is_closed() && ! l.is_closed())
}


// Tests ==========================================================================================
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_lower_create_check_destroy(){
        let b = LowerBound::new(5, true);
        assert_eq!(b.val(), &5);
        assert_eq!(b.is_closed(), true);
        assert_eq!(b.into_tuple(), (5, true))
    }

    #[test]
    fn test_upper_create_check_destroy(){
        let b = UpperBound::new(5, true);
        assert_eq!(b.val(), &5);
        assert_eq!(b.is_closed(), true);
        assert_eq!(b.into_tuple(), (5, true))
    }

    #[test]
    fn test_is_separated_from() {
        assert!(UpperBound::new(5,true).is_separated_from(&LowerBound::new(6,false)));
        assert!(UpperBound::new(5,true).is_separated_from(&LowerBound::new(6,true)));
        assert!(!UpperBound::new(5,true).is_separated_from(&LowerBound::new(5,false)));
        assert!(!UpperBound::new(5,true).is_separated_from(&LowerBound::new(5,true)));
    }

    #[test]
    fn test_lower_cmp(){
        assert!(LowerBound::new(5, true)<LowerBound::new(6, true));
        assert!(LowerBound::new(5, true)<LowerBound::new(5, false));
        assert_eq!(LowerBound::new(5, true), LowerBound::new(5, true));
        assert_eq!(LowerBound::new(5, false), LowerBound::new(5, false));
        assert!(LowerBound::new(5, false)>LowerBound::new(5, true));
        assert!(LowerBound::new(5, false)>LowerBound::new(4, false));
    }

    #[test]
    fn test_upper_cmp(){
        assert!(UpperBound::new(5, false)<UpperBound::new(6, false));
        assert!(UpperBound::new(5, false)<UpperBound::new(5, true));
        assert_eq!(UpperBound::new(5, true), UpperBound::new(5, true));
        assert_eq!(UpperBound::new(5, false), UpperBound::new(5, false));
        assert!(UpperBound::new(5, true)>UpperBound::new(5, false));
        assert!(UpperBound::new(5, true)>UpperBound::new(4, true));
    }

    #[test]
    fn test_lower_upper_cmp(){
        assert!(LowerBound::new(5, true)<UpperBound::new(6, true));
        assert!(LowerBound::new(5, true)<UpperBound::new(5, false));
        assert_ne!(LowerBound::new(5, true), UpperBound::new(5, true));
        assert_ne!(LowerBound::new(5, false), UpperBound::new(5, false));
        assert!(LowerBound::new(5, false)>UpperBound::new(5, true));
        assert!(LowerBound::new(5, false)>UpperBound::new(4, false));
    }

    #[test]
    fn test_upper_lower_cmp(){
        assert!(UpperBound::new(5, false)<LowerBound::new(6, false));
        assert!(UpperBound::new(5, false)<LowerBound::new(5, true));
        assert_ne!(UpperBound::new(5, true), LowerBound::new(5, true));
        assert_ne!(UpperBound::new(5, false), LowerBound::new(5, false));
        assert!(UpperBound::new(5, true)>LowerBound::new(5, false));
        assert!(UpperBound::new(5, true)>LowerBound::new(4, true));
    }

    #[test]
    fn test_add(){
        let mut l = LowerBound::new(6, false);
        assert_eq!(l+4, LowerBound::new(10, false));
        l+= 3;
        assert_eq!(l, LowerBound::new(9, false));

        let mut u = UpperBound::new(6, false);
        assert_eq!(u+4, UpperBound::new(10, false));
        u+= 3;
        assert_eq!(u, UpperBound::new(9, false));
    }

    #[test]
    fn test_sub(){
        let mut l = LowerBound::new(6, false);
        assert_eq!(l-2, LowerBound::new(4, false));
        l-= 3;
        assert_eq!(l, LowerBound::new(3, false));

        let mut u = UpperBound::new(6, false);
        assert_eq!(u-2, UpperBound::new(4, false));
        u-= 3;
        assert_eq!(u, UpperBound::new(3, false));
    }

    #[test]
    fn test_mul(){
        let mut l = LowerBound::new(6, false);
        assert_eq!(l*2, LowerBound::new(12, false));
        l*= -3;
        assert_eq!(l, LowerBound::new(-18, false));

        let mut u = UpperBound::new(6, false);
        assert_eq!(u*2, UpperBound::new(12, false));
        u*= -3;
        assert_eq!(u, UpperBound::new(-18, false));
    }

    #[test]
    fn test_div(){
        let mut l = LowerBound::new(6, false);
        assert_eq!(l/2, LowerBound::new(3, false));
        l/= -3;
        assert_eq!(l, LowerBound::new(-2, false));

        let mut u = UpperBound::new(6, false);
        assert_eq!(u/2, UpperBound::new(3, false));
        u/= -3;
        assert_eq!(u, UpperBound::new(-2, false));
    }

    #[test]
    fn test_lower_val_cmp(){
        let o = LowerBound::new(5, false);

        assert_ne!(o, 5);

        assert!(o>4);
        assert!(o>5);
        assert!(!(o>6));

        assert!(o>=4);
        assert!(o>=5);
        assert!(!(o>=6));

        assert!(!(o<4));
        assert!(!(o<5));
        assert!(o<6);

        assert!(!(o<=4));
        assert!(!(o<=5));
        assert!(o<=6);

        let c = LowerBound::new(5, true);
        assert_eq!(c, 5);

        assert!(c>4);
        assert!(!(c>5));
        assert!(!(c>6));

        assert!(c>=4);
        assert!(c>=5);
        assert!(!(c>=6));

        assert!(!(c<4));
        assert!(!(c<5));
        assert!(c<6);

        assert!(!(c<=4));
        assert!(c<=5);
        assert!(c<=6);
    }

    #[test]
    fn test_upper_val_cmp(){
        let o = UpperBound::new(5, false);

        assert_ne!(o, 5);

        assert!(o>4);
        assert!(!(o>5));
        assert!(!(o>6));

        assert!(o>=4);
        assert!(!(o>=5));
        assert!(!(o>=6));

        assert!(!(o<4));
        assert!(o<5);
        assert!(o<6);

        assert!(!(o<=4));
        assert!(o<=5);
        assert!(o<=6);

        let c = LowerBound::new(5, true);
        assert_eq!(c, 5);

        assert!(c>4);
        assert!(!(c>5));
        assert!(!(c>6));

        assert!(c>=4);
        assert!(c>=5);
        assert!(!(c>=6));

        assert!(!(c<4));
        assert!(!(c<5));
        assert!(c<6);

        assert!(!(c<=4));
        assert!(c<=5);
        assert!(c<=6);
    }
}