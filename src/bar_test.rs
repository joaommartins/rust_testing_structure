use pretty_assertions::assert_eq;

use super::*;

#[test]
fn equals() {
    let default_obj = Bar::new();
    let constructed_obj = Bar { value: "a" };
    assert_eq!(default_obj, constructed_obj);
}
