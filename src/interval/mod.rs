/*!
Mathematical intervals.

Mathematical intervals represent sets of numbers (or other values, for example IP addresses)
from between of bonds. Each bond can be closed or open meaning that the bond itself is
or isn't included in the set. An example of a closed interval is:

```txt
[1, 2]
```

That means - each number between 1 and 2 including 1 and 2. An example of an open bound is:

```txt
(3,7)
```

That means - each number between 3 and 7 but 3 and 7 are excluded from the set.
A special case of an interval is an empty interval, usually noted as:

```txt
∅
```

[More on intervals at Wikipedia](https://en.wikipedia.org/wiki/Interval_(mathematics))

This implementation is highly inspired by three C++ boost libraries:

- [interval](https://www.boost.org/doc/libs/1_69_0/libs/numeric/interval/doc/interval.htm)
- [icl](https://www.boost.org/doc/libs/1_69_0/libs/icl/doc/html/boost/icl/interval.html)
- [ptime](https://www.boost.org/doc/libs/1_69_0/doc/html/date_time/posix_time.html#date_time.posix_time.time_period)

*/


mod bounds;
mod interval;
mod interval_cmp;
mod interval_math;

pub use self::bounds::{LowerBound, UpperBound};
pub use self::interval::Interval;