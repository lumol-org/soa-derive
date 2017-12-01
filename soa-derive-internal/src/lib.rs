#![recursion_limit="512"]

//! This crate provides a custom derive (`#[derive(StructOfArray)]`) to
//! automatically generate code from a given struct `T` that allow to replace
//! `Vec<T>` with a struct of arrays. For example, the following code
//!
//! ```
//! # #[macro_use] extern crate soa_derive;
//! # fn main() {
//! #[derive(StructOfArray)]
//! pub struct Particle {
//!     pub x: f64,
//!     pub y: f64,
//!     pub z: f64,
//! }
//! # }
//! ```
//!
//! will generate a `ParticleVec` struct that looks like this:
//!
//! ```
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
//! ```
//! # #[macro_use] extern crate soa_derive;
//! # fn main() {
//! #[derive(Debug, PartialEq, StructOfArray)]
//! #[soa_derive = "Debug, PartialEq"]
//! pub struct Particle {
//!     pub x: f64,
//!     pub y: f64,
//!     pub z: f64,
//! }
//! # }
//! ```
//!
//! # `zip` functions
//!
//! You can activate generation of two additional functions `zip` and
//! `zip_mut` by decorating some fields of the struct with
//! `#[soa_derive(zip)]`. These functions allow to iterate over multiple
//! fields of the derivated `Vec`.
//!
//! ```
//! # #[macro_use] extern crate soa_derive;
//! # fn main() {
//! #[derive(StructOfArray)]
//! pub struct Particle {
//!     #[soa_derive(zip)]
//!     pub x: f64,
//!     #[soa_derive(zip)]
//!     pub y: f64,
//!     #[soa_derive(zip)]
//!     pub z: f64,
//!     // Do not derive zip for this field
//!     pub name: String,
//! }
//! # }
//! ```
//!
//! This generate a `zip_particle` module containing marker types used to
//! request some specific data, like this:
//!
//! ```
//! # #[macro_use] extern crate soa_derive;
//! # #[derive(StructOfArray)]
//! # pub struct Particle {
//! #    #[soa_derive(zip)]
//! #    pub x: f64,
//! #    #[soa_derive(zip)]
//! #    pub y: f64,
//! #    #[soa_derive(zip)]
//! #    pub z: f64,
//! #    // Do not derive zip for this field
//! #    pub name: String,
//! # }
//! # fn main() {
//! use zip_particle::{X, Y, Z};
//!
//! let mut vector = ParticleVec::new();
//!
//! for x in vector.zip(&X) {
//!     // x is a &f64
//! }
//!
//! let mut total = 0.0;
//! for (&x, &z) in vector.zip((&X, &Z)) {
//!     total += x * z;
//! }
//!
//! // Mutable iteration is also available with zip_mut
//! for y in vector.zip_mut(&mut Y) {
//!     *y += 6.4;
//! }
//!
//! for (&x, z) in vector.zip_mut((&X, &mut Z)) {
//!     *z *= x;
//! }
//! # }
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
//! Because of the way the code is implemented, the `zip` functions can not be
//! derived on more than 4 fields, as the compilation time grows exponentialy.
//! Deriving it for 5 fields makes the compilation time goes from 6s to more
//! than 2 minutes. These functions will only be derived if the struct is
//! declared as public.
//!
//! # Documentation
//!
//! Please see http://lumol.org/soa-derive/soa_derive_example/ for a small
//! example and the documentation of all the generated code.

extern crate proc_macro;
extern crate case;
extern crate permutohedron;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

mod structs;
mod vec;
mod refs;
mod slice;
mod iter;
mod zip;

#[proc_macro_derive(StructOfArray, attributes(soa_derive))]
pub fn soa_derive(input: TokenStream) -> TokenStream {
    let source = input.to_string();
    let ast = syn::parse_macro_input(&source).unwrap();
    let input = structs::Struct::new(ast);

    let mut generated = quote::Tokens::new();
    generated.append(zip::derive(&input).as_str());
    generated.append(vec::derive(&input).as_str());
    generated.append(refs::derive(&input).as_str());
    generated.append(slice::derive_slice(&input).as_str());
    generated.append(slice::derive_slice_mut(&input).as_str());
    generated.append(iter::derive(&input).as_str());
    generated.parse().unwrap()
}
