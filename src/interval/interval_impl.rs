#![allow(dead_code)]

#[derive(Clone, Debug, Copy, Eq, PartialEq, Hash)]
pub enum IntervalImpl<T> where T: Ord {
    Empty,
    Open(T, T),
    LowerClosed(T, T),
    UpperClosed(T, T),
    Closed(T, T)

}