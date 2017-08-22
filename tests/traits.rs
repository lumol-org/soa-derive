#[macro_use]
extern crate soa_derive;

// This test checks that the derive code works even when not all standard
// traits are implemented

#[derive(StructOfArray)]
struct Private {
    inner: f64,
}

#[test]
fn private() {
    let p = Private {inner: 42.0};
    assert_eq!(p.inner, 42.0);
}

pub struct Empty;
#[derive(StructOfArray)]
pub struct NoTraits {
    inner: Empty,
}
