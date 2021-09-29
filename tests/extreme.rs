#![deny(warnings)]
#![allow(clippy::float_cmp)]

use soa_derive::StructOfArray;

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
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
    name: String,
}

// strange names used by variables inside the implementation.
// This checks for hygiene in code generation
#[derive(Debug, Clone, PartialEq, StructOfArray)]
pub struct BadNames {
    pub index: String,
    pub at: String,
    pub other: String,
    pub len: String,
    pub size: String,
    pub cap: String,
    pub capacity: String,
    pub buf: String,
}

// Raw identifiers
#[derive(Debug, Clone, PartialEq, StructOfArray)]
pub struct RawIdent {
    pub r#for: String,
    pub r#in: String,
}
