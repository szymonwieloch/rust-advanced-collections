/*!
Counts recurring elements from a provided iterable.

This structure was highly inspired by the Python `Counter` class:

[https://docs.python.org/3/library/collections.html#collections.Counter](https://docs.python.org/3/library/collections.html#collections.Counter)

Its main purpose is to count elements in a iterable collection and provide related statistics.

# Complexity

|Metric |  Complexity |
|-------|-------------|
|Initialization | O(n) |
| Memory usage | O(k) |

where k - number of unique elements in the initializing series.

*/

mod counter;
mod fast_counter;

pub use self::counter::Counter;
pub use self::fast_counter::FastCounter;