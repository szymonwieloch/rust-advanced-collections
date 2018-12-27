/*!
# Overview

This crate contains a set of high quality (tested, documented, with complete implementation
of standard traits) collections. It is supposed to be an extension of the standard
`std::collections` crate that contains the most common collections but lacks more advanced ones.

# Usage

Cargo.toml:

```toml
[dependencies]
advanced_collections = "0.1"
```

*/

pub mod counter;
pub mod disjoint_set;
pub mod circular_buffer;
pub mod interval;