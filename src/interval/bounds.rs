use std::cmp::Ordering;
use self::Ordering::*;

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

    pub fn val(&self) -> &T {
        &self.val
    }

    pub fn is_closed(&self) -> bool {
        self.is_closed
    }

    pub fn new(val: T, is_closed: bool) -> Self {
        Self {
            val,
            is_closed
        }
    }

    pub fn into_tuple(self) -> (T, bool) {
        (self.val, self.is_closed)
    }
}

impl<T> Ord for LowerBound<T> where T: Ord {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.val.cmp(&other.val) {
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
        match self.val.cmp(&other.val) {
            Greater => Some(Greater),
            Less => Some(Less),
            Equal => {
                if self.is_closed == other.is_closed {
                    Some(Equal)
                } else {
                    if self.is_closed {
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
        match self.val.cmp(&other.val) {
            Greater => false,
            Less => true,
            Equal => !self.is_closed || !other.is_closed()
        }
    }

    fn le(&self, other: &UpperBound<T>) -> bool {
        self.val <= *other.val()
    }

    fn gt(&self, other: &UpperBound<T>) -> bool {
        match self.val.cmp(&other.val) {
            Greater => true,
            Less => false,
            Equal => !self.is_closed || !other.is_closed()
        }
    }

    fn ge(&self, other: &UpperBound<T>) -> bool {
        self.val >= *other.val()
    }
}

impl <T> PartialEq<UpperBound<T>> for LowerBound<T> where T: Ord {
    fn eq(&self, other: &UpperBound<T>) -> bool {
        false
    }

    fn ne(&self, other: &UpperBound<T>) -> bool {
        false
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

    pub fn val(&self) -> &T {
        &self.val
    }

    pub fn is_closed(&self) -> bool {
        self.is_closed
    }

    pub fn new(val: T, is_closed: bool) -> Self{
        Self {
            val, is_closed
        }
    }

    pub fn into_tuple(self) -> (T, bool) {
        (self.val, self.is_closed)
    }
}

impl<T> Ord for UpperBound<T> where T: Ord {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.val.cmp(&other.val) {
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
        match self.val.cmp(&other.val) {
            Greater => Some(Greater),
            Less => Some(Less),
            Equal => {
                if self.is_closed == other.is_closed {
                    Some(Equal)
                } else {
                    if self.is_closed {
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
        match self.val.cmp(&other.val) {
            Greater => false,
            Less => true,
            Equal => !self.is_closed || !other.is_closed()
        }
    }

    fn le(&self, other: &LowerBound<T>) -> bool {
        self.val <= *other.val()
    }

    fn gt(&self, other: &LowerBound<T>) -> bool {
        match self.val.cmp(&other.val) {
            Greater => true,
            Less => false,
            Equal => !self.is_closed || !other.is_closed()
        }
    }

    fn ge(&self, other: &LowerBound<T>) -> bool {
        self.val >= *other.val()
    }
}

impl <T> PartialEq<LowerBound<T>> for UpperBound<T> where T: Ord {
    fn eq(&self, other: &LowerBound<T>) -> bool {
        false
    }

    fn ne(&self, other: &LowerBound<T>) -> bool {
        false
    }
}

// Helpers

fn  are_separated<T>(l: &LowerBound<T>, u: &UpperBound<T>) -> bool where T: Ord{
    l.val() > u.val() || (u.val() == l.val() && !u.is_closed() && ! l.is_closed())
}


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
}