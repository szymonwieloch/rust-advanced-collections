use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hash};
use std::iter::{Extend, FromIterator};
use std::default::Default;
use std::iter::Iterator;
use std::collections::hash_map::IntoIter;

#[derive(Debug, Clone, Copy)]
struct Data {
    pub parent: usize,
    pub rank: u32
}

impl Data {
    pub fn new(id: usize) -> Data {
        Data {
            parent: id,
            rank: 0
        }
    }
}
/*
struct FieldIter<'a, T> where T: 'a + Eq + Hash {
    field: &'a T
}

impl<'a, T> FieldIter<'a, T> where T: 'a + Eq + Hash {
    pub fn new(field: &'a T) -> Self {
        Self{field}
    }
}
*/
pub struct SetIter<'a, T> where T:'a + Eq + Hash{
    sets: IntoIter<usize, Vec<&'a T>>
}

impl<'a, T> Iterator for SetIter<'a, T> where T:'a + Eq + Hash {
    type Item = ::std::vec::IntoIter<&'a T>;

    fn next<'b>(&'b mut self) -> Option<<Self as Iterator>::Item> {
        match self.sets.next() {
            Option::None => None,
            Option::Some((_key, vect)) => Some(vect.into_iter())
        }

    }
}

impl<'a, T> SetIter<'a, T> where T:'a+Eq+Hash {
    pub fn new(sets: IntoIter<usize, Vec<&'a T>>) -> Self {
        Self{
            sets
        }
    }
}


/**


where α() - very slowly growing function. α(n) < 4 for any reasonable n.
Therefore O(α(n)) ≈ O(1).

# Example

```
extern crate advanced_collections;
use advanced_collections::disjoint_set::DisjointSet;
use std::iter::FromIterator;

fn main(){
   let arr = [1,2,3,4,5,6,7];

   //creates 7 disjoint sets
   let mut ds: DisjointSet<i32> = DisjointSet::from_iter(&arr);

    //you can join existing sets
    ds.union(1, 2);

    //or add elements to existing sets
    ds.union(1,8);

    //you can check if elements are in the same set
    assert!(ds.in_union(&2,&8));
    assert!(!ds.in_union(&3,&4));

    //or if the element has been previously added to the set
    assert!(ds.contains(&7));
    assert!(!ds.contains(&10));

    //finally, you can access sets and content of sets using iterator
    for set in &mut ds {
        println!("A new set:");
        for elem in set.into_iter() {
            print!("{}, ", elem);
        }
        println!("");
    }
}
```
*/
#[derive(Clone, Debug)]
pub struct DisjointSet<T, S=RandomState>  where T: Eq+Hash , S: BuildHasher{
    ids: HashMap<T, usize, S>,
    data_by_id: Vec<Data>
}

impl<T, S> DisjointSet<T, S> where T:Eq + Hash , S:BuildHasher{

    /// Creates a new, empty `DisjointSet`.
    pub fn new() -> Self where S: Default{
        Default::default()
    }

    /**
    Creates an empty DisjointSet with the specified capacity.

    The DisjointSet will be able to hold at least capacity elements without reallocating.
    If capacity is 0, the DisjointSet will not allocate.
    */
    pub fn with_capacity(capacity: usize) -> Self where S: Default{
        Self {
            ids: HashMap::with_capacity_and_hasher(capacity, Default::default()),
            data_by_id: Vec::with_capacity(capacity)
        }
    }

    /**
    Creates an empty DisjointSet which will use the given hash builder to hash keys.

    The created set has the default initial capacity.
    */
    pub fn with_hasher(hash_builder: S) -> Self {
        Self {
            ids: HashMap::with_hasher(hash_builder),
            data_by_id: Vec::new()
        }
    }

    /**
    Creates an empty Counter with the specified capacity, using hash_builder to hash the keys.

    The Counter will be able to hold at least capacity elements without reallocating.
    If capacity is 0, the Counter will not allocate.
    */
    pub fn with_capacity_and_hasher(capacity: usize, hash_builder: S) -> Self {
        Self {
            ids: HashMap::with_capacity_and_hasher(capacity, hash_builder),
            data_by_id: Vec::with_capacity(capacity)
        }
    }

    /**
    Crates a subset with the provided element.

    If the given element already exists, nothing happens.

    **Complexity:**: O(1)
    */
    pub fn make_set(&mut self, val: T) {
        self.make_or_get_set(val);
    }

    /**
    Joins two subsets using one element from both subsets.

    If the provided elements do not exist in the collection when this function is called,
    a new subset with one element gets created prior to joining.

    **Complexity:** O(α(n)) ≈ O(1)
    */
    pub fn union(&mut self, a :T, b: T) {
        let a = self.make_or_get_set(a);
        let b = self.make_or_get_set(b);
        let mut a_root = Self::find_with_path_compression(&mut self.data_by_id, a);
        let mut b_root = Self::find_with_path_compression(&mut self.data_by_id, b);
        if a_root == b_root {
            return;
        }

        if self.data_by_id[a_root].rank < self.data_by_id[b_root].rank {
            let tmp = a_root;
            a_root = b_root;
            b_root = tmp;
        }

        self.data_by_id[b_root].parent = a_root;

        if self.data_by_id[a_root].rank == self.data_by_id[b_root].rank {
            self.data_by_id[a_root].rank += 1;
        }
    }

    /**
    Check if the given element has been added to this collection.

    **Complexity:** O(α(n)) ≈ O(1)
    */
    pub fn contains(&self, val: &T) -> bool {
        self.ids.contains_key(val)
    }

    /**
    Checks if the given two elements are in the same subset.

    **Complexity:** O(α(n)) ≈ O(1)
    */
    pub fn in_union(&mut self, a :&T, b: &T) -> bool{
        let a = match self.ids.get(a) {
            Option::None => return false,
            Option::Some(id) => *id
        };

        let b = match self.ids.get(b) {
            Option::None => return false,
            Option::Some(id) => *id
        };

        Self::find_with_path_compression(&mut self.data_by_id, a) == Self::find_with_path_compression(&mut self.data_by_id, b)
    }

    pub fn is_empty(&self) -> bool {
        self.ids.is_empty()
    }

    pub fn len(&self) -> usize {
        self.ids.len()
    }

    pub fn clear(&mut self) {
        self.ids.clear();
        self.data_by_id.clear()
    }

    pub fn reserve(&mut self, additional: usize) {
        self.data_by_id.reserve(additional);
        self.ids.reserve(additional);
    }

    fn make_or_get_set(&mut self, val: T) -> usize{
        let next_id = self.ids.len();
        //insert but do not override existing one
        match self.ids.entry(val) {
            Entry::Vacant(entry) => {
                entry.insert(next_id);
                //make element its own parent
                self.data_by_id.push(Data::new(next_id));
                next_id
            },
            Entry::Occupied(entry) => *entry.get()
        }
    }

    fn find_with_path_compression(data_by_id: &mut Vec<Data>, id: usize) -> usize{
        let mut parent = data_by_id[id].parent;
        if parent != id{
            parent = Self::find_with_path_compression(data_by_id, parent);
            data_by_id[id].parent = parent;
        }
        parent
    }

    fn build_sets<'a>(&'a mut self) -> HashMap<usize, Vec<&'a T>> {
        let mut map : HashMap<usize, Vec<&'a T>> = HashMap::new();
        for (ref key, ref val) in self.ids.iter(){
            let root = Self::find_with_path_compression(&mut self.data_by_id, **val);
            map.entry(root).or_insert_with(|| Vec::new()).push(key);
        }
        map
    }
}

impl<T, S> Default for DisjointSet<T, S>  where T: Eq+Hash , S: BuildHasher + Default {
    fn default() -> Self {

        Self{
            ids: HashMap::default(),
            data_by_id: Vec::default()
        }
    }
}

impl<T, S> FromIterator<T> for DisjointSet<T, S>
    where
        T: Hash + Eq,
        S: BuildHasher + Default,
{
    /**
    Creates DisjointSet from provided iterator.

    Elements become a new subsets with just one element
    (equivalent to calling make_set() multiple times).
    */
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let iter = iter.into_iter();
        let mut ds = Self::with_capacity(iter.size_hint().0);
        for val in iter {
            ds.make_set(val);
        }
        ds
    }
}

impl<'a, T, S> FromIterator<&'a T> for DisjointSet<T, S>
    where
        T: Hash + Eq + Clone,
        S: BuildHasher + Default,
{
    /**
   Creates DisjointSet from provided iterator.

   Elements become a new subsets with just one element
   (equivalent to calling make_set() multiple times).
   */
    fn from_iter<I: IntoIterator<Item = &'a T>>(iter: I) -> Self {
        let iter = iter.into_iter();
        let mut ds = Self::with_capacity(iter.size_hint().0);
        for val in iter.into_iter().map(|ref val| (*val).clone()) {
            ds.make_set(val)
        }
        ds
    }
}


impl<T, S> Extend<T> for DisjointSet<T, S>
    where
        T: Hash + Eq,
        S: BuildHasher,
{
    /**
   Extends collection using the provided iterator.

   Elements become a new subsets with just one element
   (equivalent to calling make_set() multiple times).
   */
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        let iter = iter.into_iter();
        self.reserve(iter.size_hint().0);
        for val in iter {
            self.make_set(val)
        }
    }
}

impl<'a, T, S> Extend<&'a T> for DisjointSet<T, S>
    where
        T: Hash + Eq + Copy,
        S: BuildHasher,
{
    /**
   Extends collection using the provided iterator.

   Elements become a new subsets with just one element
   (equivalent to calling make_set() multiple times).
   */
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        let iter = iter.into_iter();
        self.reserve(iter.size_hint().0);
        for val in iter.map(|&val| val.clone()) {
            self.make_set(val);
        }
    }
}

impl<'a, T, S> IntoIterator for &'a mut  DisjointSet<T, S>  where T: Hash + Eq, S: BuildHasher{
    type Item = ::std::vec::IntoIter<&'a T>;
    type IntoIter = SetIter<'a, T>;

    fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
        SetIter::new(self.build_sets().into_iter())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let arr = [1,2,3];
        let ds: DisjointSet<i32> = DisjointSet::from_iter(&arr);
        assert_eq!(ds.len(), 3);
    }

    #[test]
    fn contains() {
        let arr = [1,2,3];
        let ds: DisjointSet<i32> = DisjointSet::from_iter(&arr);
        assert!(ds.contains(&1));
        assert!(ds.contains(&2));
        assert!(ds.contains(&3));
        assert!(!ds.contains(&4));
        assert!(!ds.contains(&0));

    }

    #[test]
    fn make_set(){
        let mut ds: DisjointSet<i32> = DisjointSet::new();
        ds.make_set(3);
        ds.make_set(4);
        assert!(ds.contains(&3));
        assert!(ds.contains(&4));
        assert!(!ds.contains(&0));
        ds.make_set(4);
        assert!(ds.contains(&4));

    }



    #[test]
    fn union(){
        let mut ds: DisjointSet<i32> = DisjointSet::new();
        ds.union(3,4);
        ds.union(5,6);
        //union() should create sets:
        assert!(ds.contains(&3));
        assert!(ds.contains(&4));
        assert!(ds.contains(&5));
        assert!(ds.contains(&6));
        //with valid relations:
        assert!(ds.in_union(&3,&4));
        assert!(ds.in_union(&5,&6));
        assert!(!ds.in_union(&4,&5));
        ds.union(4,5);
        assert!(ds.in_union(&4, &5));
        assert!(ds.in_union(&3, &6));

    }

}