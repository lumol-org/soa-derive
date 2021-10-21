use proc_macro2::{Span, TokenStream};
use syn::Ident;
use quote::TokenStreamExt;
use quote::quote;

use crate::input::Input;

pub fn derive(input: &Input) -> TokenStream {
    let name = &input.name;
    let vec_name_str = format!("Vec<{}>", name);
    let attrs = &input.attrs.vec;
    let visibility = &input.visibility;
    let vec_name = &input.vec_name();
    let slice_name = &input.slice_name();
    let slice_mut_name = &input.slice_mut_name();
    let ref_name = &input.ref_name();
    let ptr_name = &input.ptr_name();
    let ptr_mut_name = &input.ptr_mut_name();

    let fields_names = &input.fields.iter()
                                   .map(|field| field.ident.as_ref().unwrap())
                                   .collect::<Vec<_>>();

    let fields_names_hygienic = input.fields.iter()
        .enumerate()
        .map(|(i, _)| Ident::new(&format!("___soa_derive_private_{}", i), Span::call_site()))
        .collect::<Vec<_>>();

    let first_field = &fields_names[0];

    let fields_doc = fields_names.iter()
                                 .map(|field| format!("A vector of `{0}` from a [`{1}`](struct.{1}.html)", field, name))
                                 .collect::<Vec<_>>();

    let fields_types = &input.fields.iter()
                                    .map(|field| &field.ty)
                                    .collect::<Vec<_>>();

    let mut generated = quote! {
        /// An analog to `
        #[doc = #vec_name_str]
        /// ` with Struct of Array (SoA) layout
        #[allow(dead_code)]
        #(#[#attrs])*
        #visibility struct #vec_name {
            #(
                #[doc = #fields_doc]
                pub #fields_names: Vec<#fields_types>,
            )*
        }

        #[allow(dead_code)]
        impl #vec_name {
            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::new()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.new)
            pub fn new() -> #vec_name {
                #vec_name {
                    #(#fields_names : Vec::new(),)*
                }
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::with_capacity()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.with_capacity),
            /// initializing all fields with the given `capacity`.
            pub fn with_capacity(capacity: usize) -> #vec_name {
                #vec_name {
                    #(#fields_names: Vec::with_capacity(capacity),)*
                }
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::capacity()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.capacity),
            /// the capacity of all fields should be the same.
            pub fn capacity(&self) -> usize {
                let capacity = self.#first_field.capacity();
                #(debug_assert_eq!(self.#fields_names.capacity(), capacity);)*
                capacity
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::reserve()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.reserve),
            /// reserving the same `additional` space for all fields.
            pub fn reserve(&mut self, additional: usize) {
                #(self.#fields_names.reserve(additional);)*
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::reserve_exact()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.reserve_exact)
            /// reserving the same `additional` space for all fields.
            pub fn reserve_exact(&mut self, additional: usize) {
                #(self.#fields_names.reserve_exact(additional);)*
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::shrink_to_fit()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.shrink_to_fit)
            /// shrinking all fields.
            pub fn shrink_to_fit(&mut self) {
                #(self.#fields_names.shrink_to_fit();)*
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::truncate()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.truncate)
            /// truncating all fields.
            pub fn truncate(&mut self, len: usize) {
                #(self.#fields_names.truncate(len);)*
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::push()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.push).
            pub fn push(&mut self, value: #name) {
                #(self.#fields_names.push(value.#fields_names);)*
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::len()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.len),
            /// all the fields should have the same length.
            pub fn len(&self) -> usize {
                let len = self.#first_field.len();
                #(debug_assert_eq!(self.#fields_names.len(), len);)*
                len
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::is_empty()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.is_empty),
            /// all the fields should have the same length.
            pub fn is_empty(&self) -> bool {
                let empty = self.#first_field.is_empty();
                #(debug_assert_eq!(self.#fields_names.is_empty(), empty);)*
                empty
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::swap_remove()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.swap_remove).
            pub fn swap_remove(&mut self, index: usize) -> #name {
                #(
                    let #fields_names_hygienic = self.#fields_names.swap_remove(index);
                )*
                #name{#(#fields_names: #fields_names_hygienic),*}
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::insert()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.insert).
            pub fn insert(&mut self, index: usize, element: #name) {
                #(self.#fields_names.insert(index, element.#fields_names);)*
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::remove()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.remove).
            pub fn remove(&mut self, index: usize) -> #name {
                #(
                    let #fields_names_hygienic = self.#fields_names.remove(index);
                )*
                #name{#(#fields_names: #fields_names_hygienic),*}
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::pop()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.pop).
            pub fn pop(&mut self) -> Option<#name> {
                if self.is_empty() {
                    None
                } else {
                    #(
                        let #fields_names_hygienic = self.#fields_names.pop().unwrap();
                    )*
                    Some(#name{#(#fields_names: #fields_names_hygienic),*})
                }
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::append()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.append).
            pub fn append(&mut self, other: &mut #vec_name) {
                #(
                    self.#fields_names.append(&mut other.#fields_names);
                )*
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::clear()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.clear).
            pub fn clear(&mut self) {
                #(self.#fields_names.clear();)*
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::split_off()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.split_off).
            pub fn split_off(&mut self, at: usize) -> #vec_name {
                #vec_name {
                    #(#fields_names: self.#fields_names.split_off(at), )*
                }
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::as_slice()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.as_slice).
            pub fn as_slice(&self) -> #slice_name {
                #slice_name {
                    #(#fields_names : &self.#fields_names, )*
                }
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::as_mut_slice()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.as_mut_slice).
            pub fn as_mut_slice(&mut self) -> #slice_mut_name {
                #slice_mut_name {
                    #(#fields_names : &mut self.#fields_names, )*
                }
            }

            /// Create a slice of this vector matching the given `range`. This
            /// is analogous to `Index<Range<usize>>`.
            pub fn slice(&self, range: ::std::ops::Range<usize>) -> #slice_name {
                #slice_name {
                    #(#fields_names : &self.#fields_names[range.clone()], )*
                }
            }

            /// Create a mutable slice of this vector matching the given
            /// `range`. This is analogous to `IndexMut<Range<usize>>`.
            pub fn slice_mut(&mut self, range: ::std::ops::Range<usize>) -> #slice_mut_name {
                #slice_mut_name {
                    #(#fields_names : &mut self.#fields_names[range.clone()], )*
                }
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::retain()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.retain).
            pub fn retain<F>(&mut self, mut f: F) where F: FnMut(#ref_name) -> bool {
                let len = self.len();
                let mut del = 0;

                {
                    let mut slice = self.as_mut_slice();
                    for i in 0..len {
                        if !f(slice.get(i).unwrap()) {
                            del += 1;
                        } else if del > 0 {
                            slice.swap(i - del, i);
                        }
                    }
                }
                if del > 0 {
                    self.truncate(len - del);
                }
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::get<I>()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.get).
            pub fn get<'a, I>(&'a self, index: I) -> Option<I::RefOutput>
            where
                I: ::soa_derive::SoAIndex<&'a #vec_name>
            {
                index.get(self)
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::get_unchecked<I>()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.get_unchecked).
            pub unsafe fn get_unchecked<'a, I>(&'a self, index: I) -> I::RefOutput
            where
                I: ::soa_derive::SoAIndex<&'a #vec_name>
            {
                index.get_unchecked(self)
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::index<I>()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.index).
            pub fn index<'a, I>(&'a self, index: I) -> I::RefOutput
            where
                I: ::soa_derive::SoAIndex<&'a #vec_name>
            {
                index.index(self)
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::get_mut<I>()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.get_mut).
            pub fn get_mut<'a, I>(&'a mut self, index: I) -> Option<I::MutOutput>
            where
                I: ::soa_derive::SoAIndexMut<&'a mut #vec_name>
            {
                index.get_mut(self)
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::get_unchecked_mut<I>()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.get_unchecked_mut).
            pub unsafe fn get_unchecked_mut<'a, I>(&'a mut self, index: I) -> I::MutOutput
            where
                I: ::soa_derive::SoAIndexMut<&'a mut #vec_name>
            {
                index.get_unchecked_mut(self)
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::index_mut<I>()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.index_mut).
            pub fn index_mut<'a, I>(&'a mut self, index: I) -> I::MutOutput
            where
                I: ::soa_derive::SoAIndexMut<&'a mut #vec_name>
            {
                index.index_mut(self)
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::as_ptr()`](https://doc.rust-lang.org/std/struct.Vec.html#method.as_ptr).
            pub fn as_ptr(&self) -> #ptr_name {
                #ptr_name {
                    #(#fields_names: self.#fields_names.as_ptr(),)*
                }
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::as_mut_ptr()`](https://doc.rust-lang.org/std/struct.Vec.html#method.as_mut_ptr).
            pub fn as_mut_ptr(&mut self) -> #ptr_mut_name {
                #ptr_mut_name {
                    #(#fields_names: self.#fields_names.as_mut_ptr(),)*
                }
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::from_raw_parts()`](https://doc.rust-lang.org/std/struct.Vec.html#method.from_raw_parts).
            pub unsafe fn from_raw_parts(data: #ptr_mut_name, len: usize, capacity: usize) -> #vec_name {
                #vec_name {
                    #(#fields_names: Vec::from_raw_parts(data.#fields_names, len, capacity),)*
                }
            }
        }
    };

    if input.attrs.derive_clone {
        generated.append_all(quote!{
            #[allow(dead_code)]
            impl #vec_name {
                /// Similar to [`
                #[doc = #vec_name_str]
                /// ::resize()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.resize).
                pub fn resize<T>(&mut self, new_len: usize, value: #name) {
                    #(
                        self.#fields_names.resize(new_len, value.#fields_names);
                    )*
                }
            }
        });
    }

    return generated;
}
