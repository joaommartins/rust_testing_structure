# Description
Simple repository exemplifying a rust library structure that allows unit tests separation from business logic.

This allows unit tests to be performed without having to needlessly expose private members of a module by using the child-to-parent rules of discoverability in Rust.


```
.
├── Cargo.lock
├── Cargo.toml
└── src
    ├── foo
    │   └── tests.rs
    ├── foo.rs
    └── lib.rs

2 directories, 5 files
```
