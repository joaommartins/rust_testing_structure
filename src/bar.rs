#[derive(PartialEq, Eq, Debug)]
struct Bar {
    value: &'static str,
}

impl Bar {
    fn new() -> Self {
        Self { value: "a" }
    }
}

#[cfg(test)]
#[path = "./bar_test.rs"]
mod bar_test;
