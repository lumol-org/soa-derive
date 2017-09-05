#![deny(warnings)]

#[macro_use]
extern crate soa_derive;

// This test checks that the derive code works even in some extreme cases

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

#[derive(StructOfArray)]
pub struct VeryBig {
    #[soa_derive(zip)]
    x: f64,
    #[soa_derive(zip)]
    y: f64,
    #[soa_derive(zip)]
    z: f64,
    #[soa_derive(zip)]
    vx: f64,
    vy: f64,
    vz: f64,
    name: String,
}
