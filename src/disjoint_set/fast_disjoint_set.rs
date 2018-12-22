use super::DisjointSet;
use fnv::FnvBuildHasher;

/**
A faster but less safe version of DisjointSet.

By default, HashMap uses a hashing algorithm selected to provide resistance against HashDoS attacks.
DisjointSet does it too.
This algorithm is unfortunately slow.
For the most algorithmic challenges faster and less safe algorithms are often preferred.
FastDisjointSet uses the popular ```fnv::FnvBuildHasher```.
*/
pub type FastDisjointSet<T> = DisjointSet<T, FnvBuildHasher>;