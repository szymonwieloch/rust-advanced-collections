use std::hash::{BuildHasher, Hash};
use std::iter::{Extend, FromIterator};
use std::default::Default;
use std::ops::{Add, AddAssign, Deref, DerefMut, Sub, SubAssign};
use std::collections::HashMap;
use std::collections::hash_map::RandomState;
use std::collections::hash_map::Entry;

type IntoIter<T> = ::std::collections::hash_map::IntoIter<T, usize>;
type Iter<'a, T> = ::std::collections::hash_map::Iter<'a, T, usize>;
type IterMut<'a, T> = ::std::collections::hash_map::IterMut<'a, T, usize>;

/**
Counts recurring elements from a provided iterable.

This implementation was inspired by the Python Counter class:
<https://docs.python.org/3.5/library/collections.html#collections.Counter>

# Complexity

- Initialization complexity: O(n)
- Memory complexity: O(k)

where k - number of unique elements in the initializing series.

# Example

```
extern crate advanced_collections;
use advanced_collections::Counter;
use std::iter::FromIterator;

fn main(){
    //The basic functionality is counting elements
    let mut counter: Counter<char> = Counter::from_iter("Lorem ipsum dolor sit amet enim.".chars());

    //Counter supports HashMap-like API:
    assert_eq!(counter.get(&'o'), Some(&(3 as usize)));

    //It is also possible to find the most frequent elements:
    assert_eq!(counter.most_common().get(0), Some(&(' ', 5 as usize)));

    //Finally, Counter supports adding
     let other: Counter<char> = Counter::from_iter("Etiam ullamcorper.".chars());
     counter += other;
     assert_eq!(counter.get(&'o'), Some(&(4 as usize)));
}
```
*/
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Counter<T, S = RandomState>
where
    T: Hash + Eq,
    S: BuildHasher,
{
    //pub type Map = HashMap<T, usize, S>
    counter: HashMap<T, usize, S>,
}

impl<T, S> Counter<T, S>
where
    T: Hash + Eq,
    S: BuildHasher,
{

    /// Creates a new, empty `Counter`.
    pub fn new() -> Counter<T, S> where S: Default{
        Default::default()
    }

    /**
    Creates an empty Counter with the specified capacity.

    The Counter will be able to hold at least capacity elements without reallocating.
    If capacity is 0, the Counter will not allocate.
    */
    pub fn with_capacity(capacity: usize) -> Counter<T, S> where S: Default{
        Counter {
            counter: HashMap::with_capacity_and_hasher(capacity, Default::default())
        }
    }


    /**
    Creates an empty Counter which will use the given hash builder to hash keys.

    The created map has the default initial capacity.
    */
    pub fn with_hasher(hash_builder: S) -> Counter<T, S> {
        Counter {
            counter: HashMap::with_hasher(hash_builder),
        }
    }

    /**
    Creates an empty Counter with the specified capacity, using hash_builder to hash the keys.

    The Counter will be able to hold at least capacity elements without reallocating.
    If capacity is 0, the Counter will not allocate.
    */
    pub fn with_capacity_and_hasher(capacity: usize, hash_builder: S) -> Counter<T, S> {
        Counter {
            counter: HashMap::with_capacity_and_hasher(capacity, hash_builder),
        }
    }

    /**
    Creates Counter from the provided HashMap with the same BuildHasher type.

    This function is needed because Rust does not support template specialization
    and the generic From::from() method uses a suboptimal algorithm.
    */
    pub fn from_hashmap(rhs: HashMap<T, usize, S>) -> Self {
        Self { counter: rhs }
    }

    /**
    Returns a Vec with sorted tuples - a element plus its count.

    The collection starts with the most common elements.
    Elements with equal counts are ordered arbitrarily
    */
    pub fn into_most_common(self) -> Vec<(T, usize)> {
        let mut res: Vec<(T, usize)> = Vec::from_iter(self.counter.into_iter());
        res.sort_unstable_by_key(|&(ref _key, val)| ::std::usize::MAX - val);
        res
    }

    /**
    Returns a Vec with sorted tuples - a element plus its count.

    The collection starts with the most common elements.
    Elements with equal counts are ordered arbitrarily
    */
    pub fn most_common(&self) -> Vec<(T, usize)>
    where
        T: Clone,
    {
        let mut res: Vec<(T, usize)> = self.counter
            .iter()
            .map(|(key, &val)| ((*key).clone(), val))
            .collect();
        res.sort_unstable_by_key(|&(ref _key, val)| ::std::usize::MAX - val);
        res
    }

    pub fn push(&mut self, val: T){
        *self.counter.entry(val).or_insert(0) += 1;
    }
}

impl<T, S> Default for Counter<T, S>
where
    T: Hash + Eq,
    S: BuildHasher + Default,
{
    /// Creates a new, empty `Counter`.
    fn default() -> Self {
        Counter {
            counter: HashMap::default(),
        }
    }
}

impl<T, S> FromIterator<T> for Counter<T, S>
where
    T: Hash + Eq,
    S: BuildHasher + Default,
{
    ///Creates Counter from provided iterator.
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let iter = iter.into_iter();
        let mut cnt = Self::with_capacity(iter.size_hint().0);
        for key in iter {
            cnt.push(key);
        }
        cnt
    }
}

impl<'a, T, S> FromIterator<&'a T> for Counter<T, S>
where
    T: Hash + Eq + Clone,
    S: BuildHasher + Default,
{
    ///Creates Counter from provided iterator.
    fn from_iter<I: IntoIterator<Item = &'a T>>(iter: I) -> Self {
        let iter = iter.into_iter();
        let mut cnt = Self::with_capacity(iter.size_hint().0);
        for key in iter.map(|ref key| (*key).clone()) {
            cnt.push(key);
        }
        cnt
    }
}

impl<T, S> IntoIterator for Counter<T, S>
where
    T: Hash + Eq,
    S: BuildHasher,
{
    type Item = (T, usize);
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
        self.counter.into_iter()
    }
}

impl<'a, T, S> IntoIterator for &'a Counter<T, S>
where
    T: Hash + Eq,
    S: BuildHasher,
{
    type Item = (&'a T, &'a usize);
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
        self.counter.iter()
    }
}

impl<'a, T, S> IntoIterator for &'a mut Counter<T, S>
where
    T: Hash + Eq,
    S: BuildHasher,
{
    type Item = (&'a T, &'a mut usize);
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
        self.counter.iter_mut()
    }
}

impl<T, S> Extend<T> for Counter<T, S>
where
    T: Hash + Eq,
    S: BuildHasher,
{
    ///Extends Counter with provided interator.
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        let iter = iter.into_iter();
        self.counter.reserve(iter.size_hint().0);
        for key in iter {
            self.push(key);
        }
    }
}

impl<'a, T, S> Extend<&'a T> for Counter<T, S>
where
    T: Hash + Eq + Copy,
    S: BuildHasher,
{
    ///Extends Counter with provided interator.
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        let iter = iter.into_iter();
        self.counter.reserve(iter.size_hint().0);
        for key in iter.map(|&key| key) {
            self.push(key);
        }
    }
}

impl<T, S> Deref for Counter<T, S>
where
    T: Hash + Eq,
    S: BuildHasher,
{
    type Target = HashMap<T, usize, S>;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.counter
    }
}

impl<T, S> DerefMut for Counter<T, S>
where
    T: Hash + Eq,
    S: BuildHasher,
{
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.counter
    }
}

impl<T, S1, S2> AddAssign<Counter<T, S1>> for Counter<T, S2>
where
    T: Hash + Eq,
    S1: BuildHasher,
    S2: BuildHasher,
{
    fn add_assign(&mut self, rhs: Counter<T, S1>) {
        for (key, val) in rhs.into_iter() {
            *self.counter.entry(key).or_insert(0) += val;
        }
    }
}

impl<'a, T, S1, S2> AddAssign<&'a Counter<T, S1>> for Counter<T, S2>
where
    T: Hash + Eq + Clone,
    S1: BuildHasher,
    S2: BuildHasher,
{
    fn add_assign(&mut self, rhs: &'a Counter<T, S1>) {
        for (ref key, &val) in rhs.iter() {
            *self.counter.entry((*key).clone()).or_insert(0) += val;
        }
    }
}

impl<T, S1, S2> Add<Counter<T, S1>> for Counter<T, S2>
where
    T: Hash + Eq,
    S1: BuildHasher,
    S2: BuildHasher,
{
    type Output = Counter<T, S2>;
    fn add(mut self, rhs: Counter<T, S1>) -> <Self as Add<Self>>::Output {
        self += rhs;
        self
    }
}

impl<'a, T, S1, S2> Add<&'a Counter<T, S1>> for Counter<T, S2>
where
    T: Hash + Eq + Clone,
    S1: BuildHasher,
    S2: BuildHasher,
{
    type Output = Counter<T, S2>;
    fn add(mut self, rhs: &'a Counter<T, S1>) -> <Self as Add<Self>>::Output {
        for (ref key, val) in rhs.iter() {
            *self.entry((*key).clone()).or_insert(0) += *val;
        }
        self
    }
}

impl<T, S1, S2> SubAssign<Counter<T, S1>> for Counter<T, S2>
where
    T: Hash + Eq,
    S1: BuildHasher,
    S2: BuildHasher,
{
    fn sub_assign(&mut self, rhs: Counter<T, S1>) {
        for (key, val) in rhs.into_iter() {
            match self.counter.entry(key) {
                Entry::Occupied(mut entry) => {
                    if entry.get() <= &val {
                        entry.remove();
                    } else {
                        let new_val = entry.get() - val;
                        entry.insert(new_val);
                    }
                }
                Entry::Vacant(..) => {
                    //do nothing - discard
                }
            }
        }
    }
}

impl<'a, T, S1, S2> SubAssign<&'a Counter<T, S1>> for Counter<T, S2>
where
    T: Hash + Eq + Clone,
    S1: BuildHasher,
    S2: BuildHasher,
{
    fn sub_assign(&mut self, rhs: &'a Counter<T, S1>) {
        for (key, val) in rhs.into_iter() {
            match self.counter.entry(key.clone()) {
                Entry::Occupied(mut entry) => {
                    if entry.get() <= &val {
                        entry.remove();
                    } else {
                        let new_val = entry.get() - val;
                        entry.insert(new_val);
                    }
                }
                Entry::Vacant(..) => {
                    //do nothing - discard
                }
            }
        }
    }
}

impl<T, S1, S2> Sub<Counter<T, S1>> for Counter<T, S2>
where
    T: Hash + Eq,
    S1: BuildHasher,
    S2: BuildHasher,
{
    type Output = Counter<T, S2>;
    fn sub(mut self, rhs: Counter<T, S1>) -> <Self as Sub<Self>>::Output {
        self -= rhs;
        self
    }
}

impl<'a, T, S1, S2> Sub<&'a Counter<T, S1>> for Counter<T, S2>
where
    T: Hash + Eq + Clone,
    S1: BuildHasher,
    S2: BuildHasher,
{
    type Output = Counter<T, S2>;
    fn sub(mut self, rhs: &'a Counter<T, S1>) -> <Self as Sub<Self>>::Output {
        for (ref key, val) in rhs.iter() {
            match self.counter.entry((*key).clone()) {
                Entry::Occupied(mut entry) => {
                    if entry.get() <= &val {
                        entry.remove();
                    } else {
                        let new_val = entry.get() - val;
                        entry.insert(new_val);
                    }
                }
                Entry::Vacant(..) => {
                    //do nothing - discard
                }
            }
        }
        self
    }
}

impl<T, S1, S2> From<HashMap<T, usize, S1>> for Counter<T, S2>
where
    T: Hash + Eq,
    S1: BuildHasher,
    S2: BuildHasher + Default,
{
    /**
    Creates Counter from provided HashMap.

    Please note that this is the function for generic conversion.
    The ```from_hashmap()``` function is more optimal if Counter and HashMap use the same
    BuildHasher.
    */
    fn from(rhs: HashMap<T, usize, S1>) -> Self {
        Counter {
            counter: HashMap::from_iter(rhs.into_iter()),
        }
    }
}

impl<'a, T, S1, S2> From<&'a HashMap<T, usize, S1>> for Counter<T, S2>
where
    T: Hash + Eq + Clone,
    S1: BuildHasher,
    S2: BuildHasher + Default,
{
    fn from(rhs: &'a HashMap<T, usize, S1>) -> Self {
        Counter {
            counter: HashMap::from_iter(rhs.iter().map(|(ref key, &val)| ((*key).clone(), val))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let _cnt: Counter<i32> = Counter::new();
    }

    #[test]
    fn chars() {
        let s = "Lorem ipsum";
        let cnt: Counter<char> = Counter::from_iter(s.chars());
        assert_eq!(cnt[&'m'], 2)
    }

    #[test]
    fn ints() {
        let arr = [1,2,3,1,2,3,1,2,3];
        let mut cnt: Counter<i32> = Counter::from_iter(&arr);
        assert_eq!(cnt.len(), 3);
        assert_eq!(cnt[&2], 3);
        cnt.extend(&[1,2,3,4,5]);
        assert_eq!(cnt.len(), 5);
        assert_eq!(cnt[&1], 4);


    }


}