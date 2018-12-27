/*!
Counts recurring elements from a provided iterable.

Its main purpose is to count elements in a iterable collection and provide related statistics.
However it closely mimics the well known structure called hash bag or hash multiset.

**More:** <https://en.wikipedia.org/wiki/Multiset>

# Complexity

|Metric                                 |  Complexity |
|---------------------------------------|-------------|
|Initialization                         | O(n)        |
| Memory usage                          | O(k)        |
|Insertion                              | O(1)        |
|Calculating the most popular elements  | O(k)        |

where k - number of unique elements in the initializing series.

# Inspiration

This structure was highly inspired by the Python `Counter` class:

[https://docs.python.org/3/library/collections.html#collections.Counter](https://docs.python.org/3/library/collections.html#collections.Counter)
*/

mod counter;
mod fast_counter;

pub use self::counter::Counter;
pub use self::fast_counter::FastCounter;