# Automatic Struct of Array generation for Rust

[![Build Status](https://travis-ci.org/lumol-org/soa-derive.svg?branch=master)](https://travis-ci.org/lumol-org/soa-derive)
[![Crates.io](https://img.shields.io/crates/v/soa_derive.svg)](https://crates.io/crates/soa_derive)

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

It will also generate the same functions that a `Vec<Particle>` would have,
and a few helper structs: `ParticleSlice`, `ParticleSliceMut`,
`ParticleRef` and `ParticleRefMut`.

## How to use it

Add `#[derive(StructOfArray)]` to each struct you want to derive a struct
of array version. If you need the helper structs to derive additional
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

## `zip` functions

You can activate generation of two additional functions `zip` and
`zip_mut` by decorating some fields of the struct with
`#[soa_derive(zip)]`. These functions allow to iterate over multiple
fields of the derivated `Vec`.

```rust
#[derive(StructOfArray)]
pub struct Particle {
    #[soa_derive(zip)]
    pub x: f64,
    #[soa_derive(zip)]
    pub y: f64,
    #[soa_derive(zip)]
    pub z: f64,
    // Do not derive zip for this field
    pub name: String,
}
```

This generate a `zip_particle` module containing marker types used to
request some specific data, like this:

```rust
use zip_particle::{X, Y, Z};

let mut vector = ParticleVec::new();

for x in vector.zip(&X) {
    // x is a &f64
}

let mut total = 0.0;
for (&x, &z) in vector.zip((&X, &Z)) {
    total += x * z;
}

// Mutable iteration is also available with zip_mut
for y in vector.zip_mut(&mut Y) {
    *y += 6.4;
}

for (&x, z) in vector.zip_mut((&X, &mut Z)) {
    *z *= x;
}
```

## Caveats and limitations

`Vec<T>` functionalities rely a lot on references and automatic *deref*
feature, for getting function from `[T]` and indexing. But the SoA vector
(let's call it `CheeseVec`, generated from the `Cheese` struct) generated
by this crate can not implement `Deref<Target=CheeseSlice>`, because
`Deref` is required to return a reference, and `CheeseSlice` is not a
reference. The same applies to `Index` and `IndexMut` trait, that can not
return `CheeseRef/CheeseRefMut`.

This means that the we can not index into a `CheeseVec`, and that a few
functions are duplicated, or require a call to `as_ref()/as_mut()` to
change the type used.

Because of the way the code is implemented, the `zip` functions can not be
derived on more than 4 fields, as the compilation time grows exponentialy.
Deriving it for 5 fields makes the compilation time goes from 6s to more
than 2 minutes. These functions will only be derived if the struct is
declared as public.

## Documentation

Please see http://lumol.org/soa-derive/soa_derive_example/ for a small
example and the documentation of all the generated code.

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
versions of the same code, using a small and a big struct.

You can run the same benchmarks on your own system by cloning this repository
and running `cargo bench`.

# Licensing and contributions

This crate distributed under either the MIT or the Apache license, at your
choice. Contributions are welcome, please open an issue before to discuss your
changes !

The code is based on an initial idea by @maikklein:  https://maikklein.github.io/post/soa-rust/
