# Rust Testing Structure
Simple repository exemplifying a rust library structure that allows unit tests separation from business logic.

This allows unit tests to be performed without having to needlessly expose private members of a module by using the child-to-parent rules of discoverability in Rust.

The separate sub-module solution used in the `foo` module isn't able to handle Cargo.toml `[dev-dependencies]`. An
alternative way of separating the test files is by using `#[path = "./bar_test.rs"]`, which works correctly with
Rust's development dependencies expectations.

----------


```
.
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── README.md
└── src
    ├── bar.rs
    ├── bar_test.rs
    ├── foo
    │   └── tests.rs
    ├── foo.rs
    └── lib.rs

2 directories, 9 files
```

## Credits

1. [How to move tests into a separate file for binaries in Rust's Cargo?](https://stackoverflow.com/a/39009227)
2. [Better location for unit tests in Rust](http://xion.io/post/code/rust-unit-test-placement.html)
