use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};
use super::interval::Interval;
/*
impl<T, U> Add<U> for Interval<T> where T:Ord+Add<U, Output=T>, U:Clone {
    type Output = Self;

    fn add(self, rhs: U) -> <Self as Add<U>>::Output {
        match self.into_tuple() {
            None => Self::empty(),
            Some((l, lc, u, uc)) => Self::new(l+rhs.clone(), lc, u+rhs, uc)
        }
    }
}


impl<T> AddAssign<T> for Interval<T> where T:Ord+AddAssign + Clone {
    fn add_assign(&mut self, rhs: T) {
        if let Some((a, b)) = self.range_mut(){
            *a += rhs.clone();
            *b += rhs;
        }
    }
}

impl<T> Sub<T> for Interval<T> where T:Ord+Sub<Output=T>+Clone {
    type Output = Self;

    fn sub(self, rhs: T) -> <Self as Sub<T>>::Output {
        match self.into_tuple(){
            None => Self::empty(),
            Some((l, lc, u, uc)) => Self::new(l-rhs.clone(), lc, u-rhs, uc)
        }
    }
}

impl<T> SubAssign<T> for Interval<T> where T:Ord + SubAssign + Clone {
    fn sub_assign(&mut self, rhs: T) {
        if let Some((a, b)) = self.range_mut(){
            *a -= rhs.clone();
            *b -= rhs;
        }
    }
}

impl<T> Mul<T> for Interval<T> where T:Ord+Mul<Output=T>+Clone {
    type Output = Self;

    fn mul(self, rhs: T) -> <Self as Mul<T>>::Output {
        match self.into_tuple(){
            None => Self::empty(),
            Some((l, lc, u, uc)) => Self::new(l*rhs.clone(), lc, u*rhs, uc)
        }
    }
}

impl<T> MulAssign<T> for Interval<T> where T:Ord+MulAssign+Clone {
    fn mul_assign(&mut self, rhs: T) {
        if let Some((a, b)) = self.range_mut(){
            *a *= rhs.clone();
            *b *= rhs;
        }
    }
}

impl<T> Div<T> for Interval<T> where T:Ord+Div<Output=T>+Clone {
    type Output = Self;

    fn div(self, rhs: T) -> <Self as Div<T>>::Output {
        match self.into_tuple(){
            None => Self::empty(),
            Some((l, lc, u, uc)) => Self::new(l/rhs.clone(), lc, u/rhs, uc)
        }
    }
}

impl<T> DivAssign<T> for Interval<T> where T: Ord+DivAssign + Clone {
    fn div_assign(&mut self, rhs: T) {
        if let Some((a, b)) = self.range_mut(){
            *a /= rhs.clone();
            *b /= rhs;
        }
    }
}

impl<T> Neg for Interval<T> where T: Ord + Neg<Output=T> {
    type Output = Self;

    fn neg(self) -> <Self as Neg>::Output {
        match self.into_tuple() {
            None => Self::empty(),
            Some((l, lc, u, uc)) => Self::new(-u, uc, -l, lc)
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add(){
        let mut i = Interval::lower_closed(3,6);
        assert_eq!(i+3, Interval::lower_closed(6,9));
        i+= 2;
        assert_eq!(i, Interval::lower_closed(5,8))
    }
}
*/