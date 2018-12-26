use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};
use super::interval::Interval;

impl<T, U> Add<U> for Interval<T> where T:Ord+Add<U, Output=T>, U:Clone {
    type Output = Self;

    fn add(mut self, rhs: U) -> <Self as Add<U>>::Output {
        if let Some(mut a) = self.imp {
            a.up = a.up + rhs.clone();
            a.lo = a.lo + rhs;
            let mut result = Self {imp: Some(a)};
            result.fix_after_modification();
            result
        } else {
            self
        }

    }
}


impl<T> AddAssign<T> for Interval<T> where T:Ord+AddAssign + Clone {
    fn add_assign(&mut self, rhs: T) {
        if let Some(ref mut a) = self.imp{
            a.lo += rhs.clone();
            a.up += rhs;
        }
        self.fix_after_modification()
    }
}


impl<T, U> Sub<U> for Interval<T> where T:Ord+Sub<U, Output=T>, U:Clone {
    type Output = Self;

    fn sub(self, rhs: U) -> <Self as Sub<U>>::Output {
        if let Some(mut a) = self.imp {
            a.up = a.up - rhs.clone();
            a.lo = a.lo - rhs;
            let mut result = Self {imp: Some(a)};
            result.fix_after_modification();
            result
        } else {
            self
        }
    }
}

impl<T> SubAssign<T> for Interval<T> where T:Ord + SubAssign + Clone {
    fn sub_assign(&mut self, rhs: T) {
        if let Some(ref mut a) = self.imp{
            a.lo -= rhs.clone();
            a.up -= rhs;
        }
        self.fix_after_modification()
    }
}


impl<T, U> Mul<U> for Interval<T> where T:Ord+Mul<U, Output=T>, U: Clone {
    type Output = Self;

    fn mul(self, rhs: U) -> <Self as Mul<U>>::Output {
        if let Some(mut a) = self.imp {
            a.up = a.up * rhs.clone();
            a.lo = a.lo * rhs;
            let mut result = Self {imp: Some(a)};
            result.fix_after_modification();
            result
        } else {
            self
        }
    }
}


impl<T> MulAssign<T> for Interval<T> where T:Ord+MulAssign+Clone {
    fn mul_assign(&mut self, rhs: T) {
        if let Some(ref mut a) = self.imp{
            a.lo *= rhs.clone();
            a.up *= rhs;
        }
        self.fix_after_modification()
    }
}



impl<T, U> Div<U> for Interval<T> where T:Ord+Div<U, Output=T>, U:Clone {
    type Output = Self;

    fn div(self, rhs: U) -> <Self as Div<U>>::Output {
        if let Some(mut a) = self.imp {
            a.up = a.up / rhs.clone();
            a.lo = a.lo / rhs;
            let mut result = Self {imp: Some(a)};
            result.fix_after_modification();
            result
        } else {
            self
        }
    }
}


impl<T> DivAssign<T> for Interval<T> where T: Ord+DivAssign + Clone {
    fn div_assign(&mut self, rhs: T) {
        if let Some(ref mut a) = self.imp{
            a.lo /= rhs.clone();
            a.up /= rhs;
        }
        self.fix_after_modification()
    }
}


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
        let mut i = Interval::lower_closed(3i32,6);
        assert_eq!(i+3, Interval::lower_closed(6,9));
        i += 3;
        assert_eq!(i, Interval::lower_closed(6,9));

    }

    #[test]
    fn test_sub(){
        let mut i = Interval::lower_closed(3i32,6);
        assert_eq!(i-3, Interval::lower_closed(0,3));
        i -= 3;
        assert_eq!(i, Interval::lower_closed(0,3));
    }

    #[test]
    fn test_mul(){
        let mut i = Interval::lower_closed(3i32,6);
        assert_eq!(i*3, Interval::lower_closed(9,18));
        assert_eq!(i*-1, Interval::upper_closed(-6, -3));

        i *= -3;
        assert_eq!(i, Interval::upper_closed(-18, -9));

    }

    #[test]
    fn test_div(){
        let mut i = Interval::lower_closed(3i32,6);
        assert_eq!(i/3, Interval::lower_closed(1,2));
        assert_eq!(i/-1, Interval::upper_closed(-6, -3));
        i /= -3;
        assert_eq!(i, Interval::upper_closed(-2, -1));


        let mut i = Interval::open(2,3);
        assert_eq!(i/2, Interval::empty());
        i /= 2;
        assert_eq!(i, Interval::empty())

    }
}
