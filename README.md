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

# Benchmarks

Here are a few [simple benchmarks](benches/soa.rs) results, on my machine:

```
running 10 tests
test aos_big_do_work_1000    ... bench:         997 ns/iter (+/- 192)
test aos_big_do_work_10000   ... bench:      21,324 ns/iter (+/- 3,282)
test aos_big_push            ... bench:          93 ns/iter (+/- 17)
test aos_small_do_work_10000 ... bench:       8,822 ns/iter (+/- 1,459)
test aos_small_push          ... bench:          10 ns/iter (+/- 4)
test soa_big_do_work_1000    ... bench:         890 ns/iter (+/- 142)
test soa_big_do_work_10000   ... bench:      10,538 ns/iter (+/- 1,621)
test soa_big_push            ... bench:         171 ns/iter (+/- 44)
test soa_small_do_work_10000 ... bench:       8,978 ns/iter (+/- 1,538)
test soa_small_push          ... bench:          24 ns/iter (+/- 6)
```

Benchmarks tests exist for soa (struct of array) and aos (array of struct)
versions of the same code, using a samll and a big struct.

You can run the same benchmarks on your own system by cloning this repository
and running `cargo bench`.

# Licensing and contributions

This crate distributed under either the MIT or the Apache license, at your
choice. Contributions are welcome, please open an issue before to discuss your
changes !

The code is based on an initial idea by @maikklein:  https://maikklein.github.io/post/soa-rust/
