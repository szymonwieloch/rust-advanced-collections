/*!
A circular buffer (also known as circular queue, cyclic buffer or ring buffer)
is a data structure that uses a single, fixed-size buffer as if it were connected end-to-end.

This structure lends itself easily to buffering data streams.

It differs from the standard `VecDeque` collection by having a constant buffer size and by
removing last items when the buffer gets full. It is also faster thanks to not allocating any
memory while being used.

[More at Wikipedia](https://en.wikipedia.org/wiki/Circular_buffer)

# Common operations

| Operation                                             | Complexity |
|-------------------------------------------------------|------------|
|Pushing data to the begging or end of the buffer       | O(1)       |
|Poping data from the begging or end of the buffer      | O(1)       |
|Accessing the n-th element                             | O(1)       |

*/

mod circular;
mod iter;

pub use self::circular::CircularBuffer;
pub use self::iter::{IntoIter, Iter, IterMut, Drain};