use std::mem::{ManuallyDrop, uninitialized, swap, drop, transmute};
use std::ops::{Index, IndexMut};
use std::iter::{Extend, FromIterator, IntoIterator};
use std::cmp::{Ord, PartialEq, Eq, PartialOrd, Ordering};
use std::fmt;

use super::iter::{Iter, IterMut, Drain, IntoIter};


/**
Circular buffer implementation.

# Example

```
use advanced_collections::circular_buffer::CircularBuffer;
use std::iter::FromIterator;

fn main(){
    // circular buffer is always created with the given capacity
    let mut cb = CircularBuffer::new(3);

    //its typical usage is pushing data do the back and poping from the front
    cb.push_back(1);
    cb.push_back(2);
    cb.push_back(3);
    assert_eq!(cb.pop_front(), Some(1));

    //when amount of elements exceeds its capacity, the "oldest" elements are removed
    cb.push_back(4);
    cb.push_back(5);
    assert_eq!(cb.pop_front(), Some(3));

    //you can also resize the buffer on the fly if needed
    cb.resize(4);
    assert_eq!(cb.capacity(), 4);

    //you can also operate on bulks of data
    cb.extend(&[6,7,8,9]);
    let v = Vec::from_iter(cb.drain().take(2));
    assert_eq!(v, vec![6,7]);

    //or linearize the buffer to obtain one continuous slice
    assert_eq!(cb.linearize(), &[8,9]);

}
```
*/
#[derive(Clone)]
pub struct CircularBuffer<T> {
    buffer: Box<[ManuallyDrop<T>]>,
    start: usize,
    end:usize
}

impl<T> CircularBuffer<T> {
    /**
    Creates a new instance of `CircularBuffer` with the given capacity.

    # Example

    ```
    use advanced_collections::circular_buffer::CircularBuffer;

    fn main(){
        let cb:CircularBuffer<i32> = CircularBuffer::new(5);
        assert_eq!(cb.capacity(), 5);
    }
    ```
    */
    pub fn new(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }

    /**
    Creates a new instance of `CircularBuffer` with the given capacity.

    # Example

    ```
    use advanced_collections::circular_buffer::CircularBuffer;

    fn main(){
        let cb:CircularBuffer<i32> = CircularBuffer::with_capacity(5);
        assert_eq!(cb.capacity(), 5);
    }
    ```
    */
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

    /**
    Returns current number of elements in the buffer.

    # Example

    ```
    use advanced_collections::circular_buffer::CircularBuffer;

    fn main(){
        let mut cb:CircularBuffer<i32> = CircularBuffer::new(5);
        assert_eq!(cb.len(), 0);
        cb.push_back(1);
        assert_eq!(cb.len(), 1);

    }
    ```
    */
    pub fn len(&self) -> usize {
        if self.start <= self.end {
            self.end - self.start
        } else {
            self.buffer.len() + self.end - self.start
        }
    }

    /**
    Returns maximal number of elements that can be stored in the buffer.

    # Example

    ```
    use advanced_collections::circular_buffer::CircularBuffer;

    fn main(){
        let mut cb:CircularBuffer<i32> = CircularBuffer::new(5);
        assert_eq!(cb.capacity(), 5);
        cb.resize(7);
        assert_eq!(cb.capacity(), 7);

    }
    ```
    */
    pub fn capacity(&self) -> usize {
        self.buffer.len() - 1
    }

    /**
    Changes internal size of the buffer.

    # Example

    ```
    use advanced_collections::circular_buffer::CircularBuffer;

    fn main(){
        let mut cb:CircularBuffer<i32> = CircularBuffer::new(5);
        assert_eq!(cb.capacity(), 5);
        cb.resize(7);
        assert_eq!(cb.capacity(), 7);

    }
    ```
    */
    pub fn resize (&mut self, capacity: usize) {
        let mut new_buf = Vec::with_capacity(capacity+1);
        let to_be_skipped = if self.len()>capacity{
            self.len() - capacity
        } else {
            0
        };
        new_buf.extend(self.drain().skip(to_be_skipped).map(|x| ManuallyDrop::new(x)));
        let elem_num = new_buf.len();
        for _ in 0..capacity -new_buf.len() + 1{
            new_buf.push(ManuallyDrop::new(unsafe{uninitialized()}));
        }
        new_buf.shrink_to_fit();
        self.buffer = new_buf.into_boxed_slice();
        self.start = 0;
        self.end = elem_num;
    }


    /**
    Checks if the buffer is empty.

    # Example

    ```
    use advanced_collections::circular_buffer::CircularBuffer;

    fn main(){
        let mut cb:CircularBuffer<i32> = CircularBuffer::new(5);
        assert!(cb.is_empty());
        cb.push_back(1);
        assert!(!cb.is_empty());

    }
    ```
    */
    pub fn is_empty(&self) -> bool {
        self.end == self.start
    }

    /**
    Checks if the buffer is full.

    # Example

    ```
    use advanced_collections::circular_buffer::CircularBuffer;

    fn main(){
        let mut cb:CircularBuffer<i32> = CircularBuffer::new(2);
        assert!(!cb.is_full());
        cb.push_back(1);
        cb.push_back(2);
        assert!(cb.is_full());

    }
    ```
    */
    pub fn is_full(&self) -> bool {
        self.len() == self.capacity()
    }


    /**
    Places elements at the end of the buffer.

    If the buffer is full, it replaces elements from the front of the buffer.

    # Example

    ```
    use advanced_collections::circular_buffer::CircularBuffer;

    fn main(){
        let mut cb:CircularBuffer<i32> = CircularBuffer::new(3);

        cb.push_back(1);
        cb.push_back(2);
        cb.push_back(3);
        assert_eq!(cb, [1,2,3].as_ref());
        cb.push_back(4);
        assert_eq!(cb, [2,3,4].as_ref());
    }
    ```
    */
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



    /**
    Places elements at the beginning of the buffer.

    If the buffer is full, it replaces elements from the back of the buffer.

    # Example

    ```
    use advanced_collections::circular_buffer::CircularBuffer;

    fn main(){
        let mut cb:CircularBuffer<i32> = CircularBuffer::new(3);

        cb.push_front(1);
        cb.push_front(2);
        cb.push_front(3);
        assert_eq!(cb, [3,2,1].as_ref());
        cb.push_front(4);
        assert_eq!(cb, [4,3,2].as_ref());
    }
    ```
    */
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

    /**
    Pops an element from the end of the buffer.

    # Example

    ```
    use advanced_collections::circular_buffer::CircularBuffer;

    fn main(){
        let mut cb = CircularBuffer::from(vec![1,2]);

        assert_eq!(cb.pop_back(), Some(2));
        assert_eq!(cb.pop_back(), Some(1));
        assert_eq!(cb.pop_back(), None);
    }
    ```
    */
    pub fn pop_back(&mut self) -> Option<T> {
        if self.is_empty(){
            None
        } else {
            self.decr_end();
            Some(self.pop_at(self.end))
        }
    }

    /**
    Pops an element from the beginning of the buffer.

    # Example

    ```
    use advanced_collections::circular_buffer::CircularBuffer;

    fn main(){
        let mut cb = CircularBuffer::from(vec![1,2]);

        assert_eq!(cb.pop_front(), Some(1));
        assert_eq!(cb.pop_front(), Some(2));
        assert_eq!(cb.pop_front(), None);
    }
    ```
    */
    pub fn pop_front(&mut self) -> Option<T> {
        if self.is_empty(){
            None
        } else {
            let tmp = self.pop_at(self.start);
            self.incr_start();
            Some(tmp)
        }
    }

    /**
    Clears content of the buffer.

    # Example

    ```
    use advanced_collections::circular_buffer::CircularBuffer;

    fn main(){
        let mut cb = CircularBuffer::from(vec![1,2]);
        cb.clear();
        assert!(cb.is_empty());
    }
    ```
    */
    pub fn clear(&mut self) {
        while let Some(val) = self.pop_back() {
            drop(val)
        }
    }

    /**
    Returns an iterator over the buffer from the front to back.

    # Example

    ```
    use advanced_collections::circular_buffer::CircularBuffer;
    use std::iter::FromIterator;

    fn main(){
        let mut cb = CircularBuffer::from(vec![1,2,3]);
        let v = Vec::from_iter(cb.iter());
        assert_eq!(v, vec![&1,&2,&3]);
    }
    ```
    */
    pub fn iter(&self) -> Iter<T> {

        let (a,b) = self.slices();
        a.iter().chain(b.iter())
    }

    /**
    Returns a mutable iterator over the buffer from the front to back.

    # Example

    ```
    use advanced_collections::circular_buffer::CircularBuffer;

    fn main(){
        let mut cb = CircularBuffer::from(vec![1,2,3]);
        for  a in cb.iter_mut(){
            *a+= 1;
        }
        assert_eq!(cb, [2,3,4].as_ref());
    }
    ```
    */
    pub fn iter_mut(&mut self) -> IterMut<T> {
        let (a,b) = self.slices_mut();
        a.iter_mut().chain(b.iter_mut())
    }

    /**
    Appends content of one CircularBuffer at the end of another.

    If the buffer is too small for the content, elements from the begging of the buffer
    get replaced by elements from the end of the buffer.

    # Example

    ```
    use advanced_collections::circular_buffer::CircularBuffer;

    fn main(){
       let mut c1 = CircularBuffer::from(vec![1,2,3]);
       let mut c2 = CircularBuffer::from(vec![4,5,6,7]);
       c1.append(&mut c2);
       assert_eq!(c1, [5,6,7].as_ref());
       assert!(c2.is_empty());
    }
    ```
    */
    pub fn append(&mut self, other: &mut Self) {
        self.extend(other.drain())
    }

    /**
    Returns a draining iterator over the buffer.

    # Example

    ```
    use advanced_collections::circular_buffer::CircularBuffer;
    use std::iter::FromIterator;

    fn main(){
       let mut cb = CircularBuffer::from(vec![1,2,3]);
       let v = Vec::from_iter(cb.drain());
       assert_eq!(v, vec![1,2,3]);
       assert!(cb.is_empty());
    }
    ```
    */
    pub fn drain(&mut self) -> Drain<T>{
        Drain::new(self)
    }

    /**
    Returns a reference to the first element of the buffer.

    Returns `None` if the buffer is empty.

    # Example

    ```
    use advanced_collections::circular_buffer::CircularBuffer;

    fn main(){
       let mut cb = CircularBuffer::from(vec![1,2,3]);
       assert_eq!(cb.first(), Some(&1));
    }
    ```
    */
    pub fn first(&self) -> Option<&T> {
        if self.is_empty(){
            None
        } else {
            Some(&* self.buffer[self.internal_index(0)])
        }
    }

    /**
    Returns a mutable reference to the first element of the buffer.

    Returns `None` if the buffer is empty.

    # Example

    ```
    use advanced_collections::circular_buffer::CircularBuffer;

    fn main(){
       let mut cb = CircularBuffer::from(vec![1,2,3]);
       *cb.first_mut().unwrap() +=1;
       assert_eq!(cb, [2,2,3].as_ref());
    }
    ```
    */
    pub fn first_mut(&mut self) -> Option<&mut T> {
        if self.is_empty(){
            None
        } else {
            Some(&mut *self.buffer[self.internal_index(0)])
        }
    }

    /**
    Returns a reference to the last element of the buffer.

    Returns `None` if the buffer is empty.

    # Example

    ```
    use advanced_collections::circular_buffer::CircularBuffer;

    fn main(){
      let mut cb = CircularBuffer::from(vec![1,2,3]);
      assert_eq!(cb.last(), Some(&3));
    }
    ```
    */
    pub fn last(&self) -> Option<&T> {
        if self.is_empty(){
            None
        } else {
            Some(&* self.buffer[self.internal_index(self.len()-1)])
        }
    }

    /**
    Returns a mutable reference to the last element of the buffer.

    Returns `None` if the buffer is empty.

    # Example

    ```
    use advanced_collections::circular_buffer::CircularBuffer;

    fn main(){
      let mut cb = CircularBuffer::from(vec![1,2,3]);
      *cb.last_mut().unwrap() +=1;
      assert_eq!(cb, [1,2,4].as_ref());
    }
    ```
    */
    pub fn last_mut(&mut self) -> Option<&mut T> {
        if self.is_empty(){
            None
        } else {
            Some(&mut * self.buffer[self.internal_index(self.len()-1)])
        }
    }

    /**
    Returns two slices to the internal buffer.

    Because the internal buffer is circular, normally it is not possible to represent it
    as a single slice of data, but it is possible to represent it as two slices -
    one after another.

    # Example

    ```
    use advanced_collections::circular_buffer::CircularBuffer;

    fn main(){
      let mut cb = CircularBuffer::from(vec![1,2,3]);
      cb.push_back(4);
      cb.push_back(5);
      assert_eq!(cb.slices(), ([3,4].as_ref(), [5].as_ref()));
    }
    ```
    */
    pub fn slices(&self) -> (&[T], &[T]){
        let (a,b) = if self.start <= self.end {
            (&self.buffer[self.start..self.end], &self.buffer[0..0])
        } else {
            (&self.buffer[self.start..], &self.buffer[..self.end])
        };

        //ManuallyDrop is a zero-cost wrapper, can be safely converted into slice of T
        unsafe{(transmute(a), transmute(b))}
    }

    /**
    Returns two mutable slices to the internal buffer.

    Because the internal buffer is circular, normally it is not possible to represent it
    as a single slice of data, but it is possible to represent it as two slices -
    one after another.

    # Example

    ```
    use advanced_collections::circular_buffer::CircularBuffer;

    fn main(){
      let mut cb = CircularBuffer::from(vec![1,2,3]);
      cb.push_back(4);
      cb.push_back(5);
      let (mut a, mut b) = cb.slices_mut();
      a[0] = 4;
      a[1] = 5;
      b[0] = 6;
      assert_eq!(cb, [4,5,6].as_ref());
    }
    ```
    */
    pub fn slices_mut(&mut self) -> (&mut[T], &mut [T]) {
        let (a,b) = if self.start <= self.end {
            let (x, y) = self.buffer.split_at_mut(self.end);
            (&mut x[self.start..self.end], &mut y[0..0])
        } else {
            let (x, y) = self.buffer.split_at_mut(self.start);
            (y,  &mut x[..self.end])
        };

        //ManuallyDrop is a zero-cost wrapper, can be safely converted into slice of T
        unsafe{(transmute(a), transmute(b))}
    }

    /**
    Rearranges content of the buffer to achieve a continuous region.

    # Example

    ```
    use advanced_collections::circular_buffer::CircularBuffer;

    fn main(){
      let mut cb = CircularBuffer::from(vec![1,2,3]);
      cb.push_back(4);
      cb.push_back(5);
      //slices() would now return [3,4], [5]
      assert_eq!(cb.linearize(), [3,4,5].as_ref());
    }
    ```
    */
    pub fn linearize(&mut self) -> &mut [T]{
        self.buffer.rotate_left(self.start);
        self.end = self.len();
        self.start = 0;
        //ManuallyDrop is a zero-cost wrapper, can be safely converted into slice of T
        unsafe{transmute(&mut self.buffer[..self.end])}
    }



    /**
    Swaps places of two elements in the buffer.

    # Example

    ```
    use advanced_collections::circular_buffer::CircularBuffer;

    fn main(){
      let mut cb = CircularBuffer::from(vec![1,2,3]);
      cb.swap(0,1);
      assert_eq!(cb, [2,1,3].as_ref());
    }
    ```
    */
    pub fn swap(&mut self, a: usize, b: usize) {
        self.buffer.swap(self.internal_index(a), self.internal_index(b));
    }

    /**
    Reverses order of elements in the buffer.

    # Example

    ```
    use advanced_collections::circular_buffer::CircularBuffer;

    fn main(){
      let mut cb = CircularBuffer::from(vec![1,2,3]);
      cb.reverse();
      assert_eq!(cb, [3,2,1].as_ref());
    }
    ```
    */
    pub fn reverse(&mut self) {
        for a in 0..self.len()/2 {
            let b = self.len() - a - 1;
            self.buffer.swap(self.internal_index(a), self.internal_index(b));
        }
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

impl <T> FromIterator<T> for CircularBuffer<T>{
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        let mut buf = Vec::from_iter(iter.into_iter().map(|x| ManuallyDrop::new(x)));
        buf.push(unsafe{uninitialized()});
        buf.shrink_to_fit();

        let end = buf.len() -1;
        Self {
            buffer: buf.into_boxed_slice(),
            start: 0,
            end
        }
    }
}

impl <'a, T> FromIterator<&'a T> for CircularBuffer<T> where T: Clone{
    fn from_iter<I: IntoIterator<Item=&'a T>>(iter: I) -> Self {
        let mut buf = Vec::from_iter(iter.into_iter().map(|x| ManuallyDrop::new(x.clone())));
        buf.push(unsafe{uninitialized()});
        buf.shrink_to_fit();
        let end = buf.len() -1;
        Self {
            buffer: buf.into_boxed_slice(),
            start: 0,
            end
        }
    }
}

impl<T> From<Vec<T>> for CircularBuffer<T>{
    fn from(mut v : Vec<T>) -> Self {
        let buf_len = v.len();
        v.push(unsafe{uninitialized()});
        v.shrink_to_fit();
        Self{
            buffer: unsafe{transmute(v.into_boxed_slice())},
            start: 0,
            end: buf_len
        }
    }
}

impl <T> Extend<T> for CircularBuffer<T> {
    fn extend<I: IntoIterator<Item=T>>(&mut self, iter: I) {

        for el in iter{
            self.push_back(el);
        }
    }
}

impl <'a, T> Extend<&'a T> for CircularBuffer<T> where T: 'a+Clone{
    fn extend<I: IntoIterator<Item=&'a T>>(&mut self, iter: I) {

        for el in iter{
            self.push_back(el.clone());
        }
    }
}

impl <T> IntoIterator for CircularBuffer<T>{
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
        IntoIter::new(self)
    }
}


impl <'a, T> IntoIterator for &'a CircularBuffer<T>{
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
        self.iter()
    }
}

impl <'a, T> IntoIterator for &'a mut CircularBuffer<T>{
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
        self.iter_mut()
    }
}

impl<T> PartialEq for CircularBuffer<T>

    where T: PartialEq
{
    fn eq(&self, other: &Self) -> bool {
       self.iter().eq(other.iter())
    }
}

impl<T> PartialEq<[T]> for CircularBuffer<T>

    where T: PartialEq
{
    fn eq(&self, other: &[T]) -> bool {
        self.iter().eq(other.iter())
    }
}

impl<'a, T> PartialEq<&'a [T]> for CircularBuffer<T>

    where T: PartialEq
{
    fn eq(&self, other: &&'a [T]) -> bool {
        self.iter().eq(other.iter())
    }
}

impl<T> PartialOrd for CircularBuffer<T>

    where T: PartialOrd
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.iter().partial_cmp(other.iter())
    }
}

impl<T> Eq for CircularBuffer<T>
    where T: Eq
{}


impl<T> Ord for CircularBuffer<T>
    where T: Ord
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.iter().cmp(other.iter())
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn  cb_eq<T>(cb:&CircularBuffer<T>, exp: &[T]) -> bool where T:Eq {
        cb.iter().eq(exp.iter())
    }

    #[test]
    fn test_create(){
        let _cb: CircularBuffer<i32> = CircularBuffer::new(5);
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

    #[test]
    fn test_capacity() {
        let mut cb = CircularBuffer::new(5);
        assert_eq!(cb.capacity(), 5);
        cb.push_back(1);
        cb.push_back(2);
        cb.push_back(3);
        assert!(cb_eq(&cb, &[1,2,3]));
        assert_eq!(cb.capacity(), 5);
        cb.resize(7);
        assert!(cb_eq(&cb, &[1,2,3]));
        cb.resize(2);
        assert!(cb_eq(&cb, &[2,3]));
    }

    #[test]
    fn test_drain(){
    let mut cb = CircularBuffer::new(4);
        cb.push_back(1);
        cb.push_back(2);
        cb.push_back(3);
        let v:Vec<i32> = cb.drain().collect();
        assert_eq!(v, vec![1,2,3]);
        assert!(cb.is_empty());
    }

    #[test]
    fn test_append(){
        let mut cb1 = CircularBuffer::new(7);
        cb1.push_back(1);
        cb1.push_back(2);
        cb1.push_back(3);

        let mut cb2 = CircularBuffer::new(3);
        cb2.push_back(4);
        cb2.push_back(5);
        cb2.push_back(6);

        cb1.append(&mut cb2);
        assert!(cb_eq(&cb1, &[1,2,3,4,5,6]));
    }

    #[test]
    fn test_clear(){
        let mut cb = CircularBuffer::new(7);
        cb.push_back(1);
        cb.push_back(2);
        cb.push_back(3);
        cb.clear();
        assert!(cb.is_empty());
    }

    #[test]
    fn test_access_elems(){
        let mut cb = CircularBuffer::new(3);
        cb.push_back(1);
        cb.push_back(2);
        cb.push_back(3);
        cb.push_back(4);

        assert_eq!(cb.first(), Some(&2));
        assert_eq!(cb[1], 3);
        assert_eq!(cb.last(), Some(&4));

        *cb.first_mut().unwrap() = 5;
        cb[1] = 6;
        *cb.last_mut().unwrap() = 7;

        assert!(cb_eq(&cb, &[5,6,7]))
    }

    #[test]
    fn test_slices(){
        let mut cb =  CircularBuffer::new(3);
        cb.push_back(1);
        cb.push_back(2);
        cb.push_back(3);
        let (a,b) = cb.slices();
        assert_eq!(a, &[1,2,3]);
        assert_eq!(b, &[]);
        let (a,b) = cb.slices_mut();
        assert_eq!(a, &[1,2,3]);
        assert_eq!(b, &[]);
        cb.push_back(4);
        cb.push_back(5);
        let (a,b) = cb.slices();
        assert_eq!(a, &[3,4]);
        assert_eq!(b, &[5]);
        let (a,b) = cb.slices_mut();
        assert_eq!(a, &[3,4]);
        assert_eq!(b, &[5]);

    }

    #[test]
    fn test_swap(){
        let mut cb =  CircularBuffer::new(3);
        cb.push_back(1);
        cb.push_back(2);
        cb.push_back(3);
        cb.push_back(4);
        assert!(cb_eq(&cb, &[2,3,4]));
        cb.swap(1,2);
        assert!(cb_eq(&cb, &[2,4,3]));
        cb.swap(0,2);
        assert!(cb_eq(&cb, &[3,4,2]));
    }

    #[test]
    fn test_linearize(){
        let mut cb =  CircularBuffer::new(3);
        cb.push_back(1);
        cb.push_back(2);
        cb.push_back(3);
        cb.push_back(4);
        cb.push_back(5);
        assert_eq!(cb.linearize(), &[3,4,5]);
    }

    #[test]
    fn test_reverse(){
        let mut cb =  CircularBuffer::new(3);
        cb.push_back(1);
        cb.push_back(2);
        cb.push_back(3);
        cb.push_back(4);
        cb.push_back(5);
        cb.reverse();
        assert!(cb_eq(&cb, &[5,4,3]))
    }

    #[test]
    fn test_iters_val(){
        let mut cb = CircularBuffer::from_iter(vec![1,2,3]);
        assert!(cb_eq(&cb, &[1,2,3]));
        assert_eq!(cb.capacity(), 3);
        cb.extend(vec![4,5]);
        assert!(cb_eq(&cb, &[3,4,5]));
        let v = Vec::from_iter(cb.into_iter());
        assert_eq!(v, vec![3,4,5]);
    }

    #[test]
    fn test_iters_ref(){
        let mut cb = CircularBuffer::from_iter(&[1,2,3]);
        assert!(cb_eq(&cb, &[1,2,3]));
        assert_eq!(cb.capacity(), 3);
        cb.extend(&[4,5]);
        assert!(cb_eq(&cb, &[3,4,5]));
        let v = Vec::from_iter(&cb);
        assert_eq!(v, vec![&3,&4,&5]);
    }

    #[test]
    fn test_cmp() {
        let mut c1 = CircularBuffer::from(vec![1, 2, 3]);
        let c2 = CircularBuffer::from(vec![2, 3, 4]);
        assert!(c1 < c2);
        c1.push_back(4);
        assert!(c1 == c2);
        c1.push_back(5);
        assert!(c1 > c2);

        let c3 = CircularBuffer::from(vec![2, 3, 4, 5]);
        assert!(c3 > c2)
    }
}