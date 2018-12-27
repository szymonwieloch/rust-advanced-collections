/*!
Disjoint-set is a set of elements paritioned in a collection of non-overlapping subsets.

This data structure is also known as union-find or merge-find set.
This implementation supports path compression and union by rank features that make
typical operations much more efficient.

**More:** <https://en.wikipedia.org/wiki/Disjoint-set_data_structure>

# Complexity

|Metric               | Complexity |
\---------------------|------------|
| Create a new subset | O(1)       |
| Union               | ≈ O(1)     |
| Search              | ≈ O(1)     |
| Memory              | O(n)       |

*/

mod disjoint_set;
mod fast_disjoint_set;

pub use self::disjoint_set::DisjointSet;
pub use self::fast_disjoint_set::FastDisjointSet;