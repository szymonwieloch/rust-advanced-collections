use std::mem::{ManuallyDrop, uninitialized, swap, drop};
use std::ops::{Index, IndexMut};
use std::iter::{Extend, FromIterator};
use std::cmp::{Ord, PartialEq};
use std::fmt;


#[derive(Clone)]
pub struct CircularBuffer<T> {
    buffer: Box<[ManuallyDrop<T>]>,
    start: usize,
    end:usize
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }

    pub fn with_capacity(capacity: usize) -> Self {

        let mut buffer = Vec::with_capacity(capacity+1);
        for _ in 0..capacity+1 {
            buffer.push(ManuallyDrop::new(unsafe{uninitialized()}));
        }
        Self {
            buffer: buffer.into_boxed_slice(),
            start: 0,
            end: 0
        }
    }

    pub fn len(&self) -> usize {
        if self.start <= self.end {
            self.end - self.start
        } else {
            self.buffer.len() + self.end - self.start
        }
    }

    pub fn capacity(&self) -> usize {
        self.buffer.len() - 1
    }

    pub fn resize (&mut self, size: usize) {
        /*let new_buf = Vec::from_iter(self.drain().map(|x| ManuallyDrop::new(x)));
        new_buf.append(unsafe{uninitialized()});
        self.buffer = new_buf.it */
        unimplemented!()

    }

    pub fn is_empty(&self) -> bool {
        self.end == self.start
    }

    pub fn is_full(&self) -> bool {
        self.len() == self.capacity()
    }

    pub fn push_back(&mut self, val: T) {
        if self.is_full(){
            if self.capacity() == 0 {
                return;
            } else {
                self.pop_front();
            }
        }
        self.push_at(val, self.end);
        self.incr_end();
    }

    pub fn push_front(&mut self, val: T) {
        if self.is_full(){
            if self.capacity() == 0 {
                return;
            } else {
                self.pop_back();
            }
        }
        self.decr_start();
        self.push_at(val, self.start);
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.is_empty(){
            None
        } else {
            self.decr_end();
            Some(self.pop_at(self.end))
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.is_empty(){
            None
        } else {
            let tmp = self.pop_at(self.start);
            self.incr_start();
            Some(tmp)
        }
    }

    pub fn clear(&mut self) {
        while let Some(val) = self.pop_back() {
            drop(val)
        }
    }

    pub fn iter(&self)  {
        unimplemented!()
    }

    pub fn riter(&self)  {
        unimplemented!()
    }

    pub fn iter_mut(&mut self) {
        unimplemented!()
    }

    pub fn riter_mut(&mut self) {
        unimplemented!()
    }

    pub fn append(&mut self) {
        unimplemented!()
    }

    pub fn drain<R>(&mut self) {
        unimplemented!()
    }

    pub fn first(&self) -> Option<&T> {
        unimplemented!()
    }

    pub fn first_mut(&mut self) -> Option<&mut T> {
        unimplemented!()
    }

    pub fn last(&self) -> Option<&T> {
        unimplemented!()
    }

    pub fn slices(&self) -> (&[T], &[T]){
        unimplemented!()
    }

    pub fn slices_mut(&self) -> (&mut[T], &mut [T]) {
        unimplemented!()
    }

    pub fn linearize(&mut self) {
        unimplemented!()
    }

    pub fn swap(&mut self, a: usize, b: usize) {
        unimplemented!()
    }

    pub fn reverse(&mut self) {
        unimplemented!()
    }

//private helpers

    fn internal_index(&self, index: usize) -> usize {
        if index >= self.len() {
            panic!("Index outside of bound of CircularBuffer");
        }
        if self.start + index < self.buffer.len(){
            self.start + index
        } else {
            index + self.start - self.buffer.len()
        }

    }

    fn incr_end(&mut self) {
        debug_assert!(!self.is_full());
        self.end += 1;
        if self.end == self.buffer.len(){
            self.end = 0;
        }
    }

    fn decr_end(&mut self) {
        debug_assert!(!self.is_empty());
        self.end = if self.end == 0 {
            self.buffer.len() - 1
        } else {
            self.end - 1
        }
    }

    fn incr_start(&mut self) {
        debug_assert!(!self.is_empty());
        self.start +=1;
        if self.start == self.buffer.len() {
            self.start = 0;
        }
    }

    fn decr_start(&mut self){
        debug_assert!(!self.is_full());
        self.start = if self.start == 0 {
            self.buffer.len() - 1
        } else {
            self.start - 1
        }
    }

    fn pop_at(&mut self, index: usize) -> T {
        //replace place in the array with uninitialized object
        let mut tmp = ManuallyDrop::new(unsafe{uninitialized()});
        swap(&mut self.buffer[index], &mut tmp);
        ManuallyDrop::into_inner(tmp)
    }

    fn push_at(&mut self, val: T, index: usize) {
        //the replaced value is unitialized, so it should not be dropped
        self.buffer[index] = ManuallyDrop::new(val);
    }
}

impl<T> Drop for CircularBuffer<T> {
    fn drop(&mut self) {
        self.clear();
    }
}

impl <T> Index<usize> for CircularBuffer<T> {
    type Output = T;

    fn index(&self, index: usize) -> &<Self as Index<usize>>::Output {
        &*self.buffer[self.internal_index(index)]
    }
}

impl <T> IndexMut<usize> for CircularBuffer<T> {
    fn index_mut(&mut self, index: usize) -> &mut <Self as Index<usize>>::Output {
        &mut *self.buffer[self.internal_index(index)]
    }
}

impl <T> fmt::Debug for CircularBuffer<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CircularBuffer{{ start: {}, end: {}, buf_len: {} }}", self.start, self.end, self.buffer.len())
    }
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_create(){
        let cb: CircularBuffer<i32> = CircularBuffer::new(5);
    }

    #[test]
    fn test_len() {
        let mut cb = CircularBuffer::new(3);
        assert_eq!(cb.len(), 0);
        cb.push_back(1);
        cb.push_back(2);
        assert_eq!(cb.len(), 2);
        cb.push_back(3);
        cb.push_back(4);
        assert_eq!(cb.len(), 3);
        cb.pop_back();
        assert_eq!(cb.len(), 2);
        cb.pop_back();
        cb.pop_back();
        cb.pop_back();
        assert_eq!(cb.len(), 0);
    }

    #[test]
    fn test_full_empty() {
        let mut cb = CircularBuffer::new(3);
        assert!(cb.is_empty());
        assert!(!cb.is_full());
        cb.push_back(1);
        cb.push_back(2);
        assert!(!cb.is_empty());
        assert!(!cb.is_full());
        cb.push_back(3);
        assert!(!cb.is_empty());
        assert!(cb.is_full());
        cb.push_back(4);
        assert!(!cb.is_empty());
        assert!(cb.is_full());
        cb.pop_back();
        assert!(!cb.is_empty());
        assert!(!cb.is_full());
        cb.pop_back();
        cb.pop_back();
        cb.pop_back();
        assert!(cb.is_empty());
        assert!(!cb.is_full());
    }

    #[test]
    fn test_back_push_pop(){
        let mut cb = CircularBuffer::new(3);
        cb.push_back(1);
        cb.push_back(2);
        cb.push_back(3);
        cb.push_back(4);
        assert_eq!(cb.pop_back(), Some(4));
        assert_eq!(cb.pop_back(), Some(3));
        assert_eq!(cb.pop_back(), Some(2));
        assert_eq!(cb.pop_back(), None);
    }

    #[test]
    fn test_front_push_pop(){
        let mut cb = CircularBuffer::new(3);
        cb.push_front(1);
        cb.push_front(2);
        cb.push_front(3);
        cb.push_front(4);
        assert_eq!(cb.pop_front(), Some(4));
        assert_eq!(cb.pop_front(), Some(3));
        assert_eq!(cb.pop_front(), Some(2));
        assert_eq!(cb.pop_front(), None);
    }

    use std::rc::Rc;
    use std::cell::RefCell;
    use std::ops::AddAssign;

    struct Droppable {
        pub counter: Rc<RefCell<usize>>
    }

    impl Drop for Droppable{
        fn drop(&mut self) {
            *self.counter.borrow_mut() += 1;
        }
    }

    #[test]
    fn test_droppable(){
        let counter = Rc::new(RefCell::new(0));
        let d = Droppable{counter:counter.clone()};
        assert_eq!(*counter.borrow(), 0);
        drop(d);
        assert_eq!(*counter.borrow(), 1);
    }

    #[test]
    fn test_drops() {
        let counter = Rc::new(RefCell::new(0));
        let d1 = Droppable{counter: counter.clone()};
        let d2 = Droppable{counter: counter.clone()};
        let d3 = Droppable{counter: counter.clone()};
        let d4 = Droppable{counter: counter.clone()};
        let d5 = Droppable{counter: counter.clone()};

        let mut cb = CircularBuffer::new(3);
        cb.push_back(d1);
        cb.push_back(d2);
        cb.pop_back();
        assert_eq!(*counter.borrow(), 1);
        cb.push_back(d3);
        cb.push_back(d4);
        cb.push_back(d5);
        assert_eq!(*counter.borrow(), 2);
        drop(cb);
        assert_eq!(*counter.borrow_mut(), 5);
    }
}