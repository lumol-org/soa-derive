#![recursion_limit="512"]

//! This crate provides a custom derive (`#[derive(StructOfArray)]`) to
//! automatically generate code from a given struct `T` that allow to replace
//! `Vec<T>` with a struct of arrays. For example, the following code
//!
//! ```ignore
//! #[derive(StructOfArray)]
//! pub struct Particle {
//!     pub x: f64,
//!     pub y: f64,
//!     pub z: f64,
//! }
//! ```
//!
//! will generate a `ParticleVec` struct that looks like this:
//!
//! ```ignore
//! pub struct ParticleVec {
//!     pub x: Vec<f64>,
//!     pub y: Vec<f64>,
//!     pub z: Vec<f64>,
//! }
//! ```
//!
//! It will also generate the same functions that a `Vec<Particle>` would have,
//! and a few helper structs: `ParticleSlice`, `ParticleSliceMut`,
//! `ParticleRef` and `ParticleRefMut`.
//!
//! # How to use it
//!
//! Add `#[derive(StructOfArray)]` to each struct you want to derive a struct
//! of array version. If you need the helper structs to derive additional
//! traits (such as `Debug` or `PartialEq`), you can add an attribute
//! `#[soa_derive = "Debug, PartialEq"]` to the struct declaration.
//!
//! ```ignore
//! #[derive(Debug, PartialEq, StructOfArray)]
//! #[soa_derive = "Debug, PartialEq"]
//! pub struct Particle {
//!     pub x: f64,
//!     pub y: f64,
//!     pub z: f64,
//! }
//! ```
//!
//! # Caveats and limitations
//!
//! `Vec<T>` functionalities rely a lot on references and automatic *deref*
//! feature, for getting function from `[T]` and indexing. But the SoA vector
//! (let's call it `CheeseVec`, generated from the `Cheese` struct) generated
//! by this crate can not implement `Deref<Target=CheeseSlice>`, because
//! `Deref` is required to return a reference, and `CheeseSlice` is not a
//! reference. The same applies to `Index` and `IndexMut` trait, that can not
//! return `CheeseRef/CheeseRefMut`.
//!
//! This means that the we can not index into a `CheeseVec`, and that a few
//! functions are duplicated, or require a call to `as_ref()/as_mut()` to
//! change the type used.
//!
//! # Documentation
//!
//! Please see http://lumol.org/soa-derive/soa_derive_example/ for a small
//! example and the documentation of all the generated code.

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

mod structs;
mod vec;
mod refs;
mod slice;
mod iter;

#[proc_macro_derive(StructOfArray, attributes(soa_derive))]
pub fn soa_derive(input: TokenStream) -> TokenStream {
    let source = input.to_string();
    let ast = syn::parse_macro_input(&source).unwrap();
    let input = structs::Struct::new(ast);

    let mut generated = quote::Tokens::new();
    generated.append(vec::derive(&input).as_str());
    generated.append(refs::derive(&input).as_str());
    generated.append(slice::derive_slice(&input).as_str());
    generated.append(slice::derive_slice_mut(&input).as_str());
    generated.append(iter::derive(&input).as_str());
    generated.parse().unwrap()
}
