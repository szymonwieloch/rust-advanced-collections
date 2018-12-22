use std::iter::{Chain};
use std::slice::{Iter as SliceIter, IterMut as SliceIterMut};
use std::iter::Iterator;
use super::circular::CircularBuffer;


/// An iterator over `CircularBuffer<T>`.
pub type Iter<'a, T> = Chain<SliceIter<'a, T>, SliceIter<'a, T>>;

/// A mutable iterator over `CircularBuffer<T>`.
pub type IterMut<'a, T> = Chain<SliceIterMut<'a, T>, SliceIterMut<'a, T>>;

///A drainign iterator over `CircularBuffer<T>`.
pub struct Drain<'a, T>{
    buf: &'a mut CircularBuffer<T>
}

impl<'a, T> Drain<'a, T>{
    pub fn new(buf: &'a mut CircularBuffer<T>) -> Self{
        Drain{
            buf
        }
    }
}

impl <'a, T> Iterator for Drain<'a, T>{
    type Item = T;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        self.buf.pop_front()
    }
}

///A into iterator over `CircularBuffer<T>`.
pub struct IntoIter<T>{
    buf: CircularBuffer<T>
}

impl<T> IntoIter<T>{
    pub fn new(buf: CircularBuffer<T>) -> Self{
        Self{
            buf
        }
    }
}

impl <T> Iterator for IntoIter<T>{
    type Item = T;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        self.buf.pop_front()
    }
}