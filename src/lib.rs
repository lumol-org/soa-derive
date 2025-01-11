//! This crate provides a custom derive (`#[derive(StructOfArray)]`) to
//! automatically generate code from a given struct `T` that allow to replace
//! `Vec<T>` with a struct of arrays. For example, the following code
//!
//! ```
//! # #[macro_use] extern crate soa_derive;
//! # mod cheese {
//! #[derive(StructOfArray)]
//! pub struct Cheese {
//!     pub smell: f64,
//!     pub color: (f64, f64, f64),
//!     pub with_mushrooms: bool,
//!     pub name: String,
//! }
//! # }
//! ```
//!
//! will generate a `CheeseVec` struct that looks like this:
//!
//! ```
//! pub struct CheeseVec {
//!     pub smell: Vec<f64>,
//!     pub color: Vec<(f64, f64, f64)>,
//!     pub with_mushrooms: Vec<bool>,
//!     pub name: Vec<String>,
//! }
//! ```
//!
//! It will also generate the same functions that a `Vec<Chees>` would have, and
//! a few helper structs: `CheeseSlice`, `CheeseSliceMut`, `CheeseRef` and
//! `CheeseRefMut` corresponding respectivly to `&[Cheese]`, `&mut [Cheese]`,
//! `&Cheese` and `&mut Cheese`.
//!
//! # How to use it
//!
//! Add `#[derive(StructOfArray)]` to each struct you want to derive a struct of
//! array version. If you need the helper structs to derive additional traits
//! (such as `Debug` or `PartialEq`), you can add an attribute `#[soa_derive =
//! "Debug, PartialEq"]` to the struct declaration.
//!
//! ```
//! # #[macro_use] extern crate soa_derive;
//! # mod cheese {
//! #[derive(Debug, PartialEq, StructOfArray)]
//! #[soa_derive(Debug, PartialEq)]
//! pub struct Cheese {
//!     pub smell: f64,
//!     pub color: (f64, f64, f64),
//!     pub with_mushrooms: bool,
//!     pub name: String,
//! }
//! # }
//! ```
//!
//! If you want to add attribute to a specific generated struct(such as
//! `#[cfg_attr(test, derive(PartialEq))]` on `CheeseVec`), you can add an
//! attribute `#[soa_attr(Vec, cfg_attr(test, derive(PartialEq)))]` to the
//! struct declaration.
//!
//! ```
//! # #[macro_use] extern crate soa_derive;
//! # mod cheese {
//! #[derive(Debug, PartialEq, StructOfArray)]
//! #[soa_attr(Vec, cfg_attr(test, derive(PartialEq)))]
//! pub struct Cheese {
//!     pub smell: f64,
//!     pub color: (f64, f64, f64),
//!     pub with_mushrooms: bool,
//!     pub name: String,
//! }
//! # }
//! ```
//!
//! Mappings for first argument of ``soa_attr`` to the generated struct for ``Cheese``:
//! * `Vec` => `CheeseVec`
//! * `Slice` => `CheeseSlice`
//! * `SliceMut` => `CheeseSliceMut`
//! * `Ref` => `CheeseRef`
//! * `RefMut` => `CheeseRefMut`
//! * `Ptr` => `CheesePtr`
//! * `PtrMut` => `CheesePtrMut`
//!
//! # Usage and API
//!
//! All the generated code have some generated documentation with it, so you
//! should be able to use `cargo doc` on your crate and see the documentation
//! for all the generated structs and functions.
//!
//! Most of the time, you should be able to replace `Vec<Cheese>` by
//! `CheeseVec`, with exception of code using direct indexing in the vector and
//! a few other caveats listed below.
//!
//! ## Caveats and limitations
//!
//! `Vec<T>` functionalities rely a lot on references and automatic *deref*
//! feature, for getting function from `[T]` and indexing. But the SoA vector
//! (let's call it `CheeseVec`, generated from the `Cheese` struct) generated by
//! this crate can not implement `Deref<Target=CheeseSlice>`, because `Deref` is
//! required to return a reference, and `CheeseSlice` is not a reference. The
//! same applies to `Index` and `IndexMut` trait, that can not return
//! `CheeseRef/CheeseRefMut`.
//!
//! This means that the we can not index into a `CheeseVec`, and that a few
//! functions are duplicated, or require a call to `as_ref()/as_mut()` to change
//! the type used.
//!
//! # Iteration
//!
//! It is possible to iterate over the values in a `CheeseVec`
//!
//! ```no_run
//! # #[macro_use] extern crate soa_derive;
//! # mod cheese {
//! # #[derive(Debug, PartialEq, StructOfArray)]
//! # pub struct Cheese {
//! #     pub smell: f64,
//! #     pub color: (f64, f64, f64),
//! #     pub with_mushrooms: bool,
//! #     pub name: String,
//! # }
//! # impl Cheese { fn new(name: &str) -> Cheese { unimplemented!() } }
//! # fn main() {
//! let mut vec = CheeseVec::new();
//! vec.push(Cheese::new("stilton"));
//! vec.push(Cheese::new("brie"));
//!
//! for cheese in vec.iter() {
//!     // when iterating over a CheeseVec, we load all members from memory
//!     // in a CheeseRef
//!     let typeof_cheese: CheeseRef = cheese;
//!     println!("this is {}, with a smell power of {}", cheese.name, cheese.smell);
//! }
//! # }
//! # }
//! ```
//!
//! One of the main advantage of the SoA layout is to be able to only load some
//! fields from memory when iterating over the vector. In order to do so, one
//! can manually pick the needed fields:
//!
//! ```no_run
//! # #[macro_use] extern crate soa_derive;
//! # mod cheese {
//! # #[derive(Debug, PartialEq, StructOfArray)]
//! # pub struct Cheese {
//! #     pub smell: f64,
//! #     pub color: (f64, f64, f64),
//! #     pub with_mushrooms: bool,
//! #     pub name: String,
//! # }
//! # impl Cheese { fn new(name: &str) -> Cheese { unimplemented!() } }
//! # fn main() {
//! # let mut vec = CheeseVec::new();
//! # vec.push(Cheese::new("stilton"));
//! # vec.push(Cheese::new("brie"));
//! for name in &vec.name {
//!     // We get referenes to the names
//!     let typeof_name: &String = name;
//!     println!("got cheese {}", name);
//! }
//! # }
//! # }
//! ```
//!
//! In order to iterate over multiple fields at the same time, one can use the
//! [soa_zip!](macro.soa_zip.html) macro.
//!
//! ```no_run
//! # #[macro_use] extern crate soa_derive;
//! # mod cheese {
//! # #[derive(Debug, PartialEq, StructOfArray)]
//! # pub struct Cheese {
//! #     pub smell: f64,
//! #     pub color: (f64, f64, f64),
//! #     pub with_mushrooms: bool,
//! #     pub name: String,
//! # }
//! # impl Cheese { fn new(name: &str) -> Cheese { unimplemented!() } }
//! # fn main() {
//! # let mut vec = CheeseVec::new();
//! # vec.push(Cheese::new("stilton"));
//! # vec.push(Cheese::new("brie"));
//! for (name, smell, color) in soa_zip!(&mut vec, [name, mut smell, color]) {
//!     println!("this is {}, with color {:#?}", name, color);
//!     // smell is a mutable reference
//!     *smell += 1.0;
//! }
//! # }
//! # }
//! ```
//!
//! ## Nested Struct of Arrays
//!
//! In order to nest a struct of arrays inside another struct of arrays, one can use the `#[nested_soa]` attribute.
//!
//! For example, the following code
//!
//! ```
//! # mod cheese {
//! # use soa_derive::StructOfArray;
//! #[derive(StructOfArray)]
//! pub struct Point {
//!     x: f32,
//!     y: f32,
//! }
//! #[derive(StructOfArray)]
//! pub struct Particle {
//!     #[nested_soa]
//!     point: Point,
//!     mass: f32,
//! }
//! # }
//! ```
//!
//! will generate structs that looks like this:
//!
//! ```
//! pub struct PointVec {
//!     x: Vec<f32>,
//!     y: Vec<f32>,
//! }
//! pub struct ParticleVec {
//!     point: PointVec, // rather than Vec<Point>
//!     mass: Vec<f32>
//! }
//! ```
//!
//! All helper structs will be also nested, for example `PointSlice` will be nested in `ParticleSlice`.

// The proc macro is implemented in soa_derive_internal, and re-exported by this
// crate. This is because a single crate can not define both a proc macro and a
// macro_rules macro.
pub use soa_derive_internal::StructOfArray;

// External dependency necessary for implementing the sorting methods.
// It is basically used by the macro-generated code.
#[doc(hidden)]
pub use permutation::permutation::*;

/// Any struct derived by StructOfArray will auto impl this trait You can use
/// `<Cheese as StructOfArray>::Type` instead of explicit named type
/// `CheeseVec`; This will helpful in generics programing that generate struct
/// can be expressed as `<T as StructOfArray>::Type`
pub trait StructOfArray {
    type Type;
}

/// Any struct derived by StructOfArray will auto impl this trait.
///
/// Useful for generic programming and implementation of attribute `nested_soa`.
///
/// `CheeseVec::iter(&'a self)` returns an iterator which has a type `<Cheese as SoAIter<'a>>::Iter`
///
/// `CheeseVec::iter_mut(&mut 'a self)` returns an iterator which has a type `<Cheese as SoAIter<'a>>::IterMut`
pub trait SoAIter<'a> {
    type Ref;
    type RefMut;
    type Iter: 'a + Iterator<Item=Self::Ref> + IntoIterator;
    type IterMut: 'a + Iterator<Item=Self::RefMut> + IntoIterator;
}

mod private_soa_indexes {
    // From [`std::slice::SliceIndex`](https://doc.rust-lang.org/std/slice/trait.SliceIndex.html) code.
    // Limits the types that may implement the SoA index traits.
    // It's also helpful to have the exaustive list of all accepted types.

    use ::std::ops;

    pub trait Sealed {}

    impl Sealed for usize {}                        // [a]
    impl Sealed for ops::Range<usize> {}            // [a..b]
    impl Sealed for ops::RangeTo<usize> {}          // [..b]
    impl Sealed for ops::RangeFrom<usize> {}        // [a..]
    impl Sealed for ops::RangeFull {}               // [..]
    impl Sealed for ops::RangeInclusive<usize> {}   // [a..=b]
    impl Sealed for ops::RangeToInclusive<usize> {} // [..=b]
}

/// Helper trait used for indexing operations.
/// Inspired by [`std::slice::SliceIndex`](https://doc.rust-lang.org/std/slice/trait.SliceIndex.html).
pub trait SoAIndex<T>: private_soa_indexes::Sealed {
    /// The output for the non-mutable functions
    type RefOutput;

    /// Returns the reference output in this location if in bounds, `None`
    /// otherwise.
    fn get(self, soa: T) -> Option<Self::RefOutput>;
    /// Returns the reference output in this location without performing any
    /// bounds check.
    ///
    /// # Safety
    /// The index must be in bounds.
    unsafe fn get_unchecked(self, soa: T) -> Self::RefOutput;
    /// Returns the reference output in this location. Panics if it is not in
    /// bounds.
    fn index(self, soa: T) -> Self::RefOutput;
}

/// Helper trait used for indexing operations returning mutable references.
/// Inspired by [`std::slice::SliceIndex`](https://doc.rust-lang.org/std/slice/trait.SliceIndex.html).
pub trait SoAIndexMut<T>: private_soa_indexes::Sealed {
    /// The output for the mutable functions
    type MutOutput;

    /// Returns the mutable reference output in this location if in bounds,
    /// `None` otherwise.
    fn get_mut(self, soa: T) -> Option<Self::MutOutput>;
    /// Returns the mutable reference output in this location without performing
    /// any bounds check.
    ///
    /// # Safety
    /// The index must be in bounds.
    unsafe fn get_unchecked_mut(self, soa: T) -> Self::MutOutput;
    /// Returns the mutable reference output in this location. Panics if it is
    /// not in bounds.
    fn index_mut(self, soa: T) -> Self::MutOutput;
}

/// Create an iterator over multiple fields in a Struct of array style vector.
///
/// This macro takes two main arguments: the array/slice container, and a list
/// of fields to use, inside square brackets. The iterator will give references
/// to the fields, which can be mutable references if the field name is prefixed
/// with `mut`.
///
/// ```
/// # #[macro_use] extern crate soa_derive;
/// # mod cheese {
/// #[derive(StructOfArray)]
/// struct Cheese {
///     size: f64,
///     mass: f64,
///     smell: f64,
///     name: String,
/// }
///
/// # fn main() {
/// let mut vec = CheeseVec::new();
/// // fill the vector
///
/// // Iterate over immutable references
/// for (mass, size, name) in soa_zip!(&vec, [mass, size, name]) {
///     println!("got {} kg and {} cm of {}", mass, size, name);
/// }
///
/// // Iterate over mutable references
/// for (mass, name) in soa_zip!(&mut vec, [mut mass, name]) {
///     println!("got {} kg of {}, eating 1 kg", mass, name);
///     *mass -= 1.0;
/// }
/// # }
/// # }
/// ```
///
/// The iterator can also work with external iterators. In this case, the
/// iterator will yields elements until any of the fields or one external
/// iterator returns None.
///
/// ```
/// # #[macro_use] extern crate soa_derive;
/// # mod cheese {
/// # #[derive(StructOfArray)]
/// # struct Cheese {
/// #     size: f64,
/// #     mass: f64,
/// #     smell: f64,
/// #     name: String,
/// # }
/// # #[derive(Debug)] struct Cellar;
/// # fn main() {
/// let mut vec = CheeseVec::new();
/// let mut cellars = Vec::<Cellar>::new();
///
/// for (name, mass, cellar) in soa_zip!(&vec, [name, mass], &cellars) {
///     println!("we have {} kg of {} in {:#?}", mass, name, cellar);
/// }
/// # }
/// # }
/// ```
#[macro_export]
macro_rules! soa_zip {
    ($self: expr, [$($fields: tt)*] $(, $external: expr)* $(,)*) => {{
        let this = $self;
        $crate::soa_zip_impl!(@munch this, {$($fields)*} -> [] $($external ,)*)
    }};
}

pub trait SoAPointers {
    type Ptr;
    type MutPtr;
}

pub trait SoAProps<'a> : StructOfArray + SoAIter<'a> + SoAPointers {}

impl<'a, T> SoAProps<'a> for T where T: StructOfArray + SoAIter<'a> + SoAPointers {}


/**
 * The interface for the `Slice` immutable slice struct-of-arrays type.
 */
pub trait SoASlice<'a, T: SoAProps<'a>> {
    type Ref<'t> where 'a: 't, Self: 't;
    type Reborrow<'t> where 'a: 't, Self: 't;

    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn as_slice<'c>(&'c self) -> Self::Reborrow<'c> where 'a: 'c;

    /// Create a slice of this vector matching the given `range`. This
    /// is analogous to `Index<Range<usize>>`.
    fn slice<'c>(&'c self, index: impl core::ops::RangeBounds<usize>) -> Self::Reborrow<'c> where 'a: 'c;

    /// Analogous to [`slice::get()`](https://doc.rust-lang.org/std/primitive.slice.html#method.get)
    fn get<'c>(&'c self, index: usize) -> Option<Self::Ref<'c>> where 'a: 'c;

    /// Analogous to [`std::ops::Index::index()`] for `usize`
    fn index<'c>(&'c self, index: usize) -> Self::Ref<'c> where 'a: 'c;

    /// Create an immutable iterator
    fn iter(&'a self) -> T::Iter;

    /// Analogous to [`slice::first()`](https://doc.rust-lang.org/std/primitive.slice.html#method.first)
    fn first<'c>(&'c self) -> Option<Self::Ref<'c>> where 'a: 'c {
        self.get(0)
    }

    /// Analogous to [`slice::last()`](https://doc.rust-lang.org/std/primitive.slice.html#method.last)
    fn last<'c>(&'c self) -> Option<Self::Ref<'c>> where 'a: 'c {
        self.get(self.len().saturating_sub(1))
    }

    /// Obtain a `const` pointer type for this data
    fn as_ptr(&self) -> T::Ptr;
}

/**
 * The interface for the `SliceMut` mutable slice struct-of-arrays type. A generalization of [`SoASlice`]
 * whose methods can modify elements of the arrays
 */
pub trait SoAMutSlice<'a, T: SoAProps<'a>>: SoASlice<'a, T> {
    type RefMut<'t> where 'a: 't, Self: 't;
    type ReborrowMut<'t> where 'a: 't, Self: 't;

    /// Analogous to [`Vec::as_mut_slice()`]
    fn as_mut_slice<'c>(&'c mut self) -> Self::ReborrowMut<'c> where 'a: 'c;

    /// Create a mutable slice of this vector matching the given
    /// `range`. This is analogous to `IndexMut<Range<usize>>`.
    fn slice_mut<'c>(&'c mut self, index: impl core::ops::RangeBounds<usize>) -> Self::ReborrowMut<'c> where 'a: 'c;

    /// Analogous to [`slice::get_mut()`](https://doc.rust-lang.org/std/primitive.slice.html#method.get_mut)
    fn get_mut<'c>(&'c mut self, index: usize) -> Option<Self::RefMut<'c>> where 'a: 'c;

    /// Analogous to [`std::ops::IndexMut::index_mut()`] for `usize`
    fn index_mut<'c>(&'c mut self, index: usize) -> Self::RefMut<'c> where 'a: 'c;

    /// Creates a mutable iterator
    fn iter_mut(&'a mut self) -> T::IterMut;

    /** Re-order the arrays using the provided indices. This is provided so that generic sorting methods
     can be implemented because closure-passing trait methods encounter difficulties with lifetimes.

    # Example

    ```
    use soa_derive::{StructOfArray, prelude::*};

    #[derive(Debug, Clone, PartialOrd, PartialEq, StructOfArray)]
    #[soa_derive(Debug, Clone, PartialOrd, PartialEq)]
    pub struct Particle {
        pub name: String,
        pub mass: f64,
    }
    # fn may_sort(vec: &mut <Particle as SoATypes>::Vec) {
    // vec: &mut <Particle as SoATypes>::Vec
    let mut indices: Vec<_> = (0..vec.len()).collect();

    indices.sort_by(|j, k| {
        let a = vec.index(*j);
        let b = vec.index(*k);
        a.mass.total_cmp(b.mass).reverse()
    });

    vec.apply_index(&indices);
    # }
    ```
     */
    fn apply_index(&mut self, indices: &[usize]);

    /// Analogous to [`slice::first_mut()`](https://doc.rust-lang.org/std/primitive.slice.html#method.first_mut)
    fn first_mut<'c>(&'c mut self) -> Option<Self::RefMut<'c>> where 'a: 'c {
        self.get_mut(0)
    }

    /// Analogous to [`slice::last_mut()`](https://doc.rust-lang.org/std/primitive.slice.html#method.last_mut)
    fn last_mut<'c>(&'c mut self) -> Option<Self::RefMut<'c>> where 'a: 'c {
        self.get_mut(self.len().saturating_sub(1))
    }

    /// Obtain a `mut` pointer type for this data
    fn as_mut_ptr(&mut self) -> T::MutPtr;
}

/**
 * The interface for the `Vec`-like struct-of-arrays type. A generalization of [`SoAMutSlice`] whose methods can
 * also re-size the underlying arrays.
 *
 * **NOTE**: This interface is incomplete and additional methods may be added as needed.
 */
pub trait SoAVec<'a, T: SoAProps<'a>>: SoASlice<'a, T> + SoAMutSlice<'a, T> {
    /// Create a new, empty struct of arrays
    fn new() -> Self;

    /// Create a new, empty struct of arrays with the specified capacity
    fn with_capacity(capacity: usize) -> Self;

    /// Analogous to [`Vec::capacity`]
    fn capacity(&self) -> usize;

    /// Analogous to [`Vec::reserve`]
    fn reserve(&mut self, additional: usize);

    /// Analogous to [`Vec::reserve_exact`]
    fn reserve_exact(&mut self, additional: usize);

    /// Analogous to [`Vec::shrink_to_fit`]
    fn shrink_to_fit(&mut self);

    /// Analogous to [`Vec::truncate`]
    fn truncate(&mut self, len: usize);

    /// Add a singular value of `T` to the arrays. Analogous to [`Vec::push`]
    fn push(&mut self, value: T);

    /// Analogous to [`Vec::swap_remove`]
    fn swap_remove(&mut self, index: usize) -> T;

    /// Analogous to [`Vec::insert`]
    fn insert(&mut self, index: usize, element: T);

    /// Analogous to [`Vec::replace`]
    fn replace(&mut self, index: usize, element: T) -> T;

    /// Analogous to [`Vec::remove`]
    fn remove(&mut self, index: usize) -> T;

    /// Analogous to [`Vec::pop`]
    fn pop(&mut self) -> Option<T>;

    /// Analogous to [`Vec::append`]
    fn append(&mut self, other: &mut Self);

    /// Analogous to [`Vec::clear`]
    fn clear(&mut self);

    /// Analogous to [`Vec::split_off`]
    fn split_off(&mut self, at: usize) -> Self;
}

/** A collection of types that represent the different facets of a [`StructOfArray`] type.

 It is a convenience type that can be used as a trait bound, but because it introduces
 a lifetime, it is not folded directly into [`StructOfArray`] itself. It also ensures
 that the associated types interlock between all facets.

 # Example

 Suppose one has a generic type that needs to be generic *over* [`StructOfArray`]
 types. This trait is a convenient means of claiming all the appropriate behaviors
 are available in one place:
 ```
use soa_derive::prelude::*;
#[derive(Debug, Clone)]
struct Swarm<'a, T: SoATypes<'a>> {
    entries: T::Vec,
}

impl<'a, T: SoATypes<'a>> Swarm<'a, T> {
    fn new() -> Self {
        Self {
            entries: T::Vec::new()
        }
    }

    fn push(&mut self, value: T) {
        self.entries.push(value);
    }

    fn iter(&'a self) -> T::Iter {
        self.entries.iter()
    }
}
 ```

 Without this, the generic type wouldn't be able to access any methods of `self.entries` because
 the associate type provided by [`StructOfArray`] has *no* bounds, which means it proves no methods
 are available.
*/
pub trait SoATypes<'a>: SoAProps<'a> + Sized {
    /// The [`Vec`]-like type
    type Vec: SoAVec<'a, Self,
        Ref<'a> = <Self as SoATypes<'a>>::Ref,
        Reborrow<'a> = Self::Slice,
        RefMut<'a> = <Self as SoATypes<'a>>::RefMut,
        ReborrowMut<'a> = Self::SliceMut,
    > + 'a;
    /// The immutable `&[Self]`-like type
    type Slice: SoASlice<'a, Self, Ref<'a> =<Self as SoATypes<'a>>::Ref, Reborrow<'a> = Self::Slice> + 'a;
    /// The mutable `&[Self]`-like type
    type SliceMut: SoAMutSlice<
        'a, Self,
        Ref<'a> =<Self as SoATypes<'a>>::Ref,
        Reborrow<'a> = Self::Slice,
        RefMut<'a> = <Self as SoATypes<'a>>::RefMut,
        ReborrowMut<'a> = Self::SliceMut,
    > + 'a;

    type Ref: 'a;
    type RefMut: 'a;
}

/// A collection of supporting traits for [`StructOfArray`] bundled in one place for ease-of-access
pub mod prelude {
    pub use super::{SoAVec, SoAIter, SoAProps, SoASlice, SoAMutSlice, SoAPointers, SoATypes, StructOfArray};
}


#[macro_export]
#[doc(hidden)]
macro_rules! soa_zip_impl {
    // @flatten creates a tuple-flattening closure for .map() call
    // Finish recursion
    (@flatten $p:pat => $tup:expr ) => {
        |$p| $tup
    };
    // Eat an element ($_iter) and add it to the current closure. Then recurse
    (@flatten $p:pat => ( $($tup:tt)* ) , $_iter:expr $( , $tail:expr )* ) => {
        $crate::soa_zip_impl!(@flatten ($p, a) => ( $($tup)*, a ) $( , $tail )*)
    };

    // The main code is emmited here: we create an iterator, zip it and then
    // map the zipped iterator to flatten it
    (@last , $first: expr, $($tail: expr,)*) => {
        ::std::iter::IntoIterator::into_iter($first)
            $(
                .zip($tail)
            )*
            .map(
                $crate::soa_zip_impl!(@flatten a => (a) $( , $tail )*)
            )
    };

    // Eat the last `mut $field` and then emit code
    (@munch $self: expr, {mut $field: ident} -> [$($output: tt)*] $($ext: expr ,)*) => {
        $crate::soa_zip_impl!(@last $($output)*, $self.$field.iter_mut(), $($ext, )*)
    };
    // Eat the last `$field` and then emit code
    (@munch $self: expr, {$field: ident} -> [$($output: tt)*] $($ext: expr ,)*) => {
        $crate::soa_zip_impl!(@last $($output)*, $self.$field.iter(), $($ext, )*)
    };

    // Eat the next `mut $field` and then recurse
    (@munch $self: expr, {mut $field: ident, $($tail: tt)*} -> [$($output: tt)*] $($ext: expr ,)*) => {
        $crate::soa_zip_impl!(@munch $self, {$($tail)*} -> [$($output)*, $self.$field.iter_mut()] $($ext, )*)
    };
    // Eat the next `$field` and then recurse
    (@munch $self: expr, {$field: ident, $($tail: tt)*} -> [$($output: tt)*] $($ext: expr ,)*) => {
        $crate::soa_zip_impl!(@munch $self, {$($tail)*} -> [$($output)*, $self.$field.iter()] $($ext, )*)
    };
}
