# rust-advanced-collections

[![Travis CI][tcii]][tci] [![Appveyor CI][acii]][aci] [![Crates CI][ccii]][cci]  [![Codedov CI][vcii]][vci]  

[tcii]: https://travis-ci.org/szymonwieloch/rust-advanced-collections.svg?branch=master
[tci]: https://travis-ci.org/szymonwieloch/rust-advanced-collections
[acii]: https://ci.appveyor.com/api/projects/status/github/szymonwieloch/rust-advanced-collections?svg=true
[aci]: https://ci.appveyor.com/project/szymonwieloch/rust-advanced-collections
[ccii]: https://img.shields.io/crates/v/advanced_collections.svg
[cci]: https://crates.io/crates/advanced_collections
[vcii]: https://codecov.io/api/gh/szymonwieloch/rust-advanced-collections/branch/master/graph/badge.svg
[vci]: https://codecov.io/gh/szymonwieloch/rust-advanced-collections

# Overview

This crate contains a set of high quality (tested, documented, with complete implementation
of standard traits) collections. It is supposed to be an extension of the standard
`std::collections` crate that contains the most common collections but lacks more advanced ones.

At the moment this crate includes:

- Counter - a counting and statistical collection similar to hash bag or multiset.
- Interval - structure for working with mathematical intervals.
- Disjoint set - also known as union-find or merge-find, a set of values split into a number of not overlapping subsets.
- Circular buffer -also known as cyclic buffer - a structure commonly used in multimedia streaming for storing 
    limited amount of data in a buffer.

# Usage

Cargo.toml:

```toml
[dependencies]
advanced_collections = "0.1"
```

# Documentation

[Cargo documentation](https://docs.rs/advanced_collections)

# License

This code is licensed under the free [MIT](./LICENSE) license.

# Contributing

This crate is open to anybody who would like to participate in the project and help me to create more collections.
Simply fork this repository, add your changes/fixes and create a pull request for me. 
Just please make sure that your code meets standards of this crate. The code needs to:

- Have high quality. This includes conforming to Rust formatting standards, comments, following naming conventions of
    other collections etc.
- Be tested with high code coverage.
- Be documented and have examples.
- Be well design. This includes implementation of common traits.
- Be high-performing.

