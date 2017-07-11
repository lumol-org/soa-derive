# Automatic Struct of Array generation for Rust

[![Build Status](https://travis-ci.org/lumol-org/soa-derive.svg?branch=master)](https://travis-ci.org/lumol-org/soa-derive)

This crate provides a custom derive (`#[derive(StructOfArray)]`) to
automatically generate code from a given struct `T` that allow to replace
`Vec<T>` with a struct of arrays. For example, the following code

```rust
#[derive(StructOfArray)]
pub struct Particle {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
```

will generate a `ParticleVec` struct that looks like this:

```rust
pub struct ParticleVec {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub z: Vec<f64>,
}
```

It will also generate the same functions that a `Vec<Particle>` would have, and
a few helper structs: `ParticleSlice`, `ParticleSliceMut`, `ParticleRef` and
`ParticleRefMut`.

# How to use it

Add this to your `Cargo.toml`

```
[dependencies]
soa_derive = "0.1"
```

This to your `src/lib.rs`

```
#[macro_use]
extern crate soa_derive;
```

And then add `#[derive(StructOfArray)]` to each struct you want to derive a
struct of array version. If you need the helper structs to derive additional
traits (such as `Debug` or `PartialEq`), you can add an attribute
`#[soa_derive = "Debug, PartialEq"]` to the struct declaration.

```rust
#[derive(Debug, PartialEq, StructOfArray)]
#[soa_derive = "Debug, PartialEq"]
pub struct Particle {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
```

# Documentation

Please see http://lumol.org/soa-derive/soa_derive_example/ for a small example
and the documentation of all the generated code.

# Licensing and contributions

This crate distributed under either the MIT or the Apache license, at your
choice. Contributions are welcome, please open an issue before to discuss your
changes !

The code is based on an initial idea by @maikklein:  https://maikklein.github.io/post/soa-rust/
