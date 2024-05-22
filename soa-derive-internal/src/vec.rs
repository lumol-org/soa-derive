use proc_macro2::{Ident, Span, TokenStream};
use quote::TokenStreamExt;
use quote::quote;

use crate::input::Input;
use crate::names;

pub fn derive(input: &Input) -> TokenStream {
    let name = &input.name;
    let vec_name_str = format!("Vec<{}>", name);
    let attrs = &input.attrs.vec;
    let visibility = &input.visibility;
    let vec_name = names::vec_name(&input.name);
    let slice_name = names::slice_name(name);
    let slice_mut_name = names::slice_mut_name(&input.name);
    let ref_name = names::ref_name(&input.name);
    let ref_mut_name = names::ref_mut_name(&input.name);
    let ptr_name = names::ptr_name(&input.name);
    let ptr_mut_name = names::ptr_mut_name(&input.name);

    let doc_url = format!("[`{0}`](struct.{0}.html)", input.name);

    let fields_names = &input.fields.iter()
        .map(|field| field.ident.as_ref().unwrap())
        .collect::<Vec<_>>();

    let fields_names_hygienic = input.fields.iter()
        .enumerate()
        .map(|(i, _)| Ident::new(&format!("___soa_derive_private_{}", i), Span::call_site()))
        .collect::<Vec<_>>();

    let first_field = &fields_names[0];

    let vec_fields_types = input.map_fields_nested_or(
        |_, field_type| {
            let vec_type = names::vec_name(field_type);
            quote! { #vec_type }
        },
        |_, field_type| quote! { Vec<#field_type> },
    ).collect::<Vec<_>>();

    let vec_with_capacity = input.map_fields_nested_or(
        |_, field_type| quote! { <#field_type as StructOfArray>::Type::with_capacity(capacity) },
        |_, _| quote! { Vec::with_capacity(capacity) },
    ).collect::<Vec<_>>();

    let vec_slice = input.map_fields_nested_or(
        |ident, _| quote! { self.#ident.slice(range.clone()) },
        |ident, _| quote! { &self.#ident[range.clone()] },
    ).collect::<Vec<_>>();

    let vec_slice_mut = input.map_fields_nested_or(
        |ident, _| quote! { self.#ident.slice_mut(range.clone()) },
        |ident, _| quote! { &mut self.#ident[range.clone()] },
    ).collect::<Vec<_>>();

    let vec_from_raw_parts = input.map_fields_nested_or(
        |ident, field_type| {
            let vec_type = names::vec_name(field_type);
            quote! { #vec_type::from_raw_parts(data.#ident, len, capacity) }
        },
        |ident, _| quote! { Vec::from_raw_parts(data.#ident, len, capacity) },
    ).collect::<Vec<_>>();

    let vec_replace = input.map_fields_nested_or(
        |ident, _| quote! { self.#ident.replace(index, field) },
        |ident, _| quote! { ::std::mem::replace(&mut self.#ident[index], field) },
    ).collect::<Vec<_>>();

    let mut generated = quote! {
        /// An analog to `
        #[doc = #vec_name_str]
        /// ` with Struct of Array (SoA) layout
        #[allow(dead_code)]
        #(#[#attrs])*
        #[derive(Default)]
        #visibility struct #vec_name {
            #(
                /// a vector of `
                #[doc = stringify!(#fields_names)]
                ///` from a
                #[doc = #doc_url]
                pub #fields_names: #vec_fields_types,
            )*
        }

        #[allow(dead_code)]
        #[allow(clippy::forget_non_drop)]
        impl #vec_name {
            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::new()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.new)
            pub fn new() -> #vec_name {
                Default::default()
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::with_capacity()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.with_capacity),
            /// initializing all fields with the given `capacity`.
            pub fn with_capacity(capacity: usize) -> #vec_name {
                #vec_name {
                    #( #fields_names: #vec_with_capacity, )*
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
            #[allow(clippy::forget_non_drop)]
            pub fn push(&mut self, value: #name) {
                // We need to use ptr read/write instead of moving out of the
                // fields in case the value struct implements Drop.
                unsafe {
                    #(self.#fields_names.push(::std::ptr::read(&value.#fields_names));)*
                }
                // if value implements Drop, we don't want to run it here, only
                // when the vec itself will be dropped.
                ::std::mem::forget(value);
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
            #[allow(clippy::forget_non_drop)]
            pub fn insert(&mut self, index: usize, element: #name) {
                if index > self.len() {
                    panic!("index out of bounds: the len is {} but the index is {}", self.len(), index);
                }

                // similar to push, we can not use move and have to rely on ptr
                // read/write
                unsafe {
                    #(self.#fields_names.insert(index, ::std::ptr::read(&element.#fields_names));)*
                }
                // if value implements Drop, we don't want to run it here, only
                // when the vec itself will be dropped.
                ::std::mem::forget(element);
            }

            /// Similar to [`std::mem::replace()`](https://doc.rust-lang.org/std/mem/fn.replace.html).
            #[allow(clippy::forget_non_drop)]
            pub fn replace(&mut self, index: usize, element: #name) -> #name {
                if index > self.len() {
                    panic!("index out of bounds: the len is {} but the index is {}", self.len(), index);
                }

                // similar to push, we can not use move and have to rely on ptr
                // read/write
                #(
                    let field = unsafe { ::std::ptr::read(&element.#fields_names) };
                    let #fields_names_hygienic = #vec_replace;
                )*
                // if value implements Drop, we don't want to run it here, only
                // when the vec itself will be dropped.
                ::std::mem::forget(element);

                #name{#(#fields_names: #fields_names_hygienic),*}
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
                    #(#fields_names: self.#fields_names.as_slice(), )*
                }
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::as_mut_slice()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.as_mut_slice).
            pub fn as_mut_slice(&mut self) -> #slice_mut_name {
                #slice_mut_name {
                    #(#fields_names: self.#fields_names.as_mut_slice(), )*
                }
            }

            /// Create a slice of this vector matching the given `range`. This
            /// is analogous to `Index<Range<usize>>`.
            pub fn slice(&self, range: ::std::ops::Range<usize>) -> #slice_name {
                #slice_name {
                    #( #fields_names: #vec_slice, )*
                }
            }

            /// Create a mutable slice of this vector matching the given
            /// `range`. This is analogous to `IndexMut<Range<usize>>`.
            pub fn slice_mut(&mut self, range: ::std::ops::Range<usize>) -> #slice_mut_name {
                #slice_mut_name {
                    #( #fields_names: #vec_slice_mut, )*
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
            /// ::retain_mut()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.retain_mut).
            pub fn retain_mut<F>(&mut self, mut f: F) where F: FnMut(#ref_mut_name) -> bool {
                let len = self.len();
                let mut del = 0;

                {
                    let mut slice = self.as_mut_slice();
                    for i in 0..len {
                        if !f(slice.get_mut(i).unwrap()) {
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
                    #( #fields_names: #vec_from_raw_parts, )*
                }
            }
        }

        #[allow(clippy::drop_non_drop)]
        impl Drop for #vec_name {
            fn drop(&mut self) {
                while let Some(value) = self.pop() {
                    ::std::mem::drop(value);
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
                pub fn resize(&mut self, new_len: usize, value: #name) {
                    #(
                        self.#fields_names.resize(new_len, value.#fields_names);
                    )*
                }
            }
        });
    }

    return generated;
}
