#![allow(dead_code)]
use super::interval::Interval;
use std::cmp::{Ordering, PartialOrd, PartialEq, Ord};

use self::Ordering::*;

impl<T> PartialOrd<T> for Interval<T> where T: Ord{
    fn partial_cmp(&self, val: &T) -> Option<Ordering> {
        if self < val {
            return Some(Less);
        }
        if self > val {
            return Some(Greater);
        }

        if self == val {
            return Some(Equal);
        }
        None
    }

    fn lt(&self, val: &T) -> bool {
        match self.upper(){
            None => false,
            Some(up) => match up.val().cmp(&val){
                Greater => false,
                Less => true,
                Equal => ! up.is_closed()
            }
        }
    }

    fn le(&self, val: &T) -> bool {
        match self.upper() {
            None => false,
            Some(up) => up.val() <= &val
        }
    }

    fn gt(&self, val: &T) -> bool {
        match self.lower(){
            None => false,
            Some(lo) => match lo.val().cmp(&val){
                Less => false,
                Greater => true,
                Equal => !lo.is_closed()
            }
        }
    }

    fn ge(&self, val: &T) -> bool {
        match self.lower() {
            None => false,
            Some(low) => low.val() >= &val
        }
    }
}

impl<T> PartialEq<T> for Interval<T>where T: Ord{
    fn eq(&self, val: &T) -> bool {
        match self.bounds(){
            None => false,
            Some((a,b)) => a.val() == &val && b.val() == &val
        }
    }
}

impl<T> PartialOrd for Interval<T> where T: Ord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self > other {
            return Some(Greater);
        }
        if self < other {
            return Some(Less);
        }
        if self == other {
            return Some(Equal);
        }
        None
    }

    fn lt(&self, other: &Self) -> bool {
        let up = match self.upper() {
            None => return false,
            Some(a) => a
        };
        let olow = match other.lower() {
            None => return false,
            Some(a) => a
        };
        match up.val().cmp(olow.val()) {
            Less => true,
            Greater => false,
            Equal => !(up.is_closed() && olow.is_closed())
        }
    }

    fn le(&self, other: &Self) -> bool {
        let up = match self.upper() {
            None => return false,
            Some(a) => a
        };
        let olow = match other.lower() {
            None => return false,
            Some(a) => a
        };
        up.val() <= olow.val()
    }

    fn gt(&self, other: &Self) -> bool {
        let low = match self.lower() {
            None => return false,
            Some(a) => a
        };
        let oup = match other.upper() {
            None => return false,
            Some(a) => a
        };
        match low.val().cmp(oup.val()) {
            Less => false,
            Greater => true,
            Equal => !(low.is_closed() && oup.is_closed())
        }
    }

    fn ge(&self, other: &Self) -> bool {
        let low = match self.lower() {
            None => return false,
            Some(a) => a
        };
        let oup = match other.upper() {
            None => return false,
            Some(a) => a
        };
        low.val() >= oup.val()
    }
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_eq(){
        let s = Interval::single(5);
        let i = Interval::closed(4,6);
        assert_eq!(s, 5);
        assert_ne!(s, 4);
        assert_ne!(i, 4);
        assert_ne!(i, 5);
        assert_ne!(i, 6);
    }

    #[test]
    fn test_val_cmp_ne(){
        let o = Interval::open(3,5);
        let c = Interval::closed(3,5);
        assert!(o> 2);
        assert!(o> 3);
        assert!(!(o> 4));
        assert!(o < 6);
        assert!(o < 5);
        assert!(!(o < 4));

        assert!(c > 2);
        assert!(!(c> 3));
        assert!(!(c> 4));
        assert!(c < 6);
        assert!(!(c < 5));
        assert!(!(c < 4));
    }

    #[test]
    fn test_val_cmp_eq(){
        let o = Interval::open(3,5);
        let c = Interval::closed(3,5);
        assert!(o>= 2);
        assert!(o>= 3);
        assert!(!(o>= 4));

        assert!(o <= 6);
        assert!(o <= 5);
        assert!(!(o <= 4));

        assert!(c >= 2);
        assert!(c>= 3);
        assert!(!(c>= 4));

        assert!(c <= 6);
        assert!(c <= 5);
        assert!(!(c <= 4));
    }

    #[test]
    fn test_intv_cmp_nq(){
        let o = Interval::open(3,5);
        let c = Interval::closed(3,5);

        assert!(o>Interval::open(1,2));
        assert!(o>Interval::open(1,3));
        assert!(!(o>Interval::open(1,4)));

        assert!(c>Interval::open(1,2));
        assert!(c>Interval::open(1,3));
        assert!(!(c>Interval::open(1,4)));

        assert!(o>Interval::closed(1,2));
        assert!(o>Interval::closed(1,3));
        assert!(!(o>Interval::closed(1,4)));

        assert!(c>Interval::closed(1, 2));
        assert!(!(c>Interval::closed(1,3)));
        assert!(!(c>Interval::closed(1,4)));


        assert!(o<Interval::open(6,7));
        assert!(o<Interval::open(5,7));
        assert!(!(o<Interval::open(4,7)));

        assert!(c<Interval::open(6,7));
        assert!(c<Interval::open(5,7));
        assert!(!(c<Interval::open(4,7)));

        assert!(o<Interval::closed(6,7));
        assert!(o<Interval::closed(5,7));
        assert!(!(o<Interval::closed(4,7)));

        assert!(c<Interval::closed(6,7));
        assert!(!(c<Interval::closed(5,7)));
        assert!(!(c<Interval::closed(4,7)));
    }

    #[test]
    fn test_intv_cmp_eq(){
        let o = Interval::open(3,5);
        let c = Interval::closed(3,5);

        assert!(o>=Interval::open(1,2));
        assert!(o>=Interval::open(1,3));
        assert!(!(o>=Interval::open(1,4)));

        assert!(c>=Interval::open(1,2));
        assert!(c>=Interval::open(1,3));
        assert!(!(c>=Interval::open(1,4)));

        assert!(o>=Interval::closed(1,2));
        assert!(o>=Interval::closed(1,3));
        assert!(!(o>=Interval::closed(1,4)));

        assert!(c>=Interval::closed(1, 2));
        assert!(!(!(c>=Interval::closed(1,3))));
        assert!(!(c>=Interval::closed(1,4)));


        assert!(o<=Interval::open(6,7));
        assert!(o<=Interval::open(5,7));
        assert!(!(o<=Interval::open(4,7)));

        assert!(c<=Interval::open(6,7));
        assert!(c<=Interval::open(5,7));
        assert!(!(c<=Interval::open(4,7)));

        assert!(o<=Interval::closed(6,7));
        assert!(o<=Interval::closed(5,7));
        assert!(!(o<=Interval::closed(4,7)));

        assert!(c<=Interval::closed(6,7));
        assert!(c<=Interval::closed(5,7));
        assert!(!(c<=Interval::closed(4,7)));
    }


}

