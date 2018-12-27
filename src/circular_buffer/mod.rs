/*!
A circular buffer uses a single, fixed-size buffer as if it were connected end-to-end.

 This structure is also known as circular queue, cyclic buffer or ring buffer.
It lends itself easily to buffering data streams.

It differs from the standard `VecDeque` collection by having a constant buffer size and by
removing last items when the buffer gets full. It is also faster thanks to not allocating any
memory while being used.

**More:** <https://en.wikipedia.org/wiki/Circular_buffer>

# Complexity

| Metric                                                | Complexity |
|-------------------------------------------------------|------------|
|Pushing data to the begging or end of the buffer       | O(1)       |
|Poping data from the beginning or end of the buffer    | O(1)       |
|Accessing the n-th element                             | O(1)       |

# Inspiration

This implementation was inspired by C++ boos library [circular_buffer](https://www.boost.org/doc/libs/1_69_0/doc/html/circular_buffer.html)
*/

mod circular;
mod iter;

pub use self::circular::CircularBuffer;
pub use self::iter::{IntoIter, Iter, IterMut, Drain};