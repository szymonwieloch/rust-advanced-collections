use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};
use super::interval::Interval;

impl<T, U> Add<U> for Interval<T> where T:Ord+Add<U, Output=T>, U:Clone {
    type Output = Self;

    fn add(self, rhs: U) -> <Self as Add<U>>::Output {
        match self.into_tuple() {
            None => Self::empty(),
            Some((l, lc, u, uc)) => Self::create_friendly(l+rhs.clone(), lc, u+rhs, uc)
        }
    }
}

/*
impl<T> AddAssign<T> for Interval<T> where T:Ord+AddAssign + Clone {
    fn add_assign(&mut self, rhs: T) {
        if let Some((a, b)) = self.range_mut(){
            *a += rhs.clone();
            *b += rhs;
        }
    }
}
*/

impl<T, U> Sub<U> for Interval<T> where T:Ord+Sub<U, Output=T>, U:Clone {
    type Output = Self;

    fn sub(self, rhs: U) -> <Self as Sub<U>>::Output {
        match self.into_tuple(){
            None => Self::empty(),
            Some((l, lc, u, uc)) => Self::create_friendly(l-rhs.clone(), lc, u-rhs, uc)
        }
    }
}
/*
impl<T> SubAssign<T> for Interval<T> where T:Ord + SubAssign + Clone {
    fn sub_assign(&mut self, rhs: T) {
        if let Some((a, b)) = self.range_mut(){
            *a -= rhs.clone();
            *b -= rhs;
        }
    }
}
*/

impl<T, U> Mul<U> for Interval<T> where T:Ord+Mul<U, Output=T>, U: Clone {
    type Output = Self;

    fn mul(self, rhs: U) -> <Self as Mul<U>>::Output {
        match self.into_tuple(){
            None => Self::empty(),
            Some((l, lc, u, uc)) => Self::create_friendly(l*rhs.clone(), lc, u*rhs, uc)
        }
    }
}

/*
impl<T> MulAssign<T> for Interval<T> where T:Ord+MulAssign+Clone {
    fn mul_assign(&mut self, rhs: T) {
        if let Some((a, b)) = self.range_mut(){
            *a *= rhs.clone();
            *b *= rhs;
        }
    }
}

*/

impl<T, U> Div<U> for Interval<T> where T:Ord+Div<U, Output=T>, U:Clone {
    type Output = Self;

    fn div(self, rhs: U) -> <Self as Div<U>>::Output {
        match self.into_tuple(){
            None => Self::empty(),
            Some((l, lc, u, uc)) => Self::create_friendly(l/rhs.clone(), lc, u/rhs, uc)
        }
    }
}

/*
impl<T> DivAssign<T> for Interval<T> where T: Ord+DivAssign + Clone {
    fn div_assign(&mut self, rhs: T) {
        if let Some((a, b)) = self.range_mut(){
            *a /= rhs.clone();
            *b /= rhs;
        }
    }
}
*/

impl<T> Neg for Interval<T> where T: Ord + Neg<Output=T> {
    type Output = Self;

    fn neg(self) -> <Self as Neg>::Output {
        match self.into_tuple() {
            None => Self::empty(),
            Some((l, lc, u, uc)) => Self::create_friendly(-u, uc, -l, lc)
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;


    #[test]
    fn test_add(){
        let i = Interval::lower_closed(3i32,6);
        assert_eq!(i+3, Interval::lower_closed(6,9));

    }

    #[test]
    fn test_sub(){
        let i = Interval::lower_closed(3i32,6);
        assert_eq!(i-3, Interval::lower_closed(0,3));
    }

    #[test]
    fn test_mul(){
        let i = Interval::lower_closed(3i32,6);
        assert_eq!(i*3, Interval::lower_closed(9,18));
        assert_eq!(i*-1, Interval::upper_closed(-6, -3));
    }

    #[test]
    fn test_div(){
        let i = Interval::lower_closed(3i32,6);
        assert_eq!(i/3, Interval::lower_closed(1,2));
        assert_eq!(i/-1, Interval::upper_closed(-6, -3));
        let i = Interval::open(2,3);
        assert_eq!(i/2, Interval::empty())

    }
}
