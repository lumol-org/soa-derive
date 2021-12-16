use proc_macro2::{Span, TokenStream};
use syn::Ident;
use quote::TokenStreamExt;
use quote::quote;

use crate::input::Input;

pub fn derive(input: &Input) -> TokenStream {
    let visibility = &input.visibility;
    let slice_name = &input.slice_name();
    let attrs = &input.attrs.slice;
    let vec_name = &input.vec_name();
    let ref_name = &input.ref_name();
    let ptr_name = &input.ptr_name();

    let slice_name_str = format!("[{}]", input.name);
    let doc_url = format!("[`{0}`](struct.{0}.html)", input.name);
    let vec_doc_url = format!("[`{0}`](struct.{0}.html)", input.vec_name());

    let fields_names = &input.fields.iter()
                                   .map(|field| field.ident.as_ref().unwrap())
                                   .collect::<Vec<_>>();

    let first_field = &fields_names[0];

    let fields_names_hygienic_1 = input.fields.iter()
        .enumerate()
        .map(|(i, _)| Ident::new(&format!("___soa_derive_private_1_{}", i), Span::call_site()))
        .collect::<Vec<_>>();
    let fields_names_hygienic_2 = input.fields.iter()
        .enumerate()
        .map(|(i, _)| Ident::new(&format!("___soa_derive_private_2_{}", i), Span::call_site()))
        .collect::<Vec<_>>();

    let get_field_doc = |field_ident: &Ident| {
        format!("A slice of `{0}` from a [`{1}`](struct.{1}.html)", field_ident, vec_name)
    };

    let slice_fields = input.field_seq_by_nested_soa(
        |field_ident, field_type| {
            let doc = get_field_doc(field_ident);
            quote! {
                #[doc = #doc]
                pub #field_ident: &'a [#field_type],
            }
        },
        |field_ident, field_type| {
            let doc = get_field_doc(field_ident);
            quote! {
                #[doc = #doc]
                pub #field_ident: <#field_type as soa_derive::SoASlice<'a>>::Slice,
            }
        },
    );

    let slice_reborrow = input.field_seq_by_nested_soa(
        |field_ident, _| {
            quote! {
                #field_ident: &self.#field_ident,
            }
        },
        |field_ident, _| {
            quote! {
                #field_ident: self.#field_ident.reborrow(),
            }
        },
    );

    let slice_from_raw_parts = input.field_seq_by_nested_soa(
        |field_ident, _| {
            quote! {
                #field_ident: ::std::slice::from_raw_parts(data.#field_ident, len),
            }
        },
        |field_ident, field_type| {
            quote! {
                #field_ident: <#field_type as soa_derive::SoASlice>::Slice::from_raw_parts(data.#field_ident, len),
            }
        },
    );

    let mut generated = quote! {
        /// A slice of
        #[doc = #doc_url]
        /// inside a
        #[doc = #vec_doc_url]
        /// .
        #[allow(dead_code)]
        #[derive(Copy, Clone)]
        #(#[#attrs])*
        #visibility struct #slice_name<'a> {
            #slice_fields
        }

        #[allow(dead_code)]
        impl<'a> #slice_name<'a> {
            /// Similar to [`
            #[doc = #slice_name_str]
            /// ::len()`](https://doc.rust-lang.org/std/primitive.slice.html#method.len),
            /// the length of all fields should be the same.
            pub fn len(&self) -> usize {
                let len = self.#first_field.len();
                #(debug_assert_eq!(self.#fields_names.len(), len);)*
                len
            }

            /// Similar to [`
            #[doc = #slice_name_str]
            /// ::is_empty()`](https://doc.rust-lang.org/std/primitive.slice.html#method.is_empty),
            /// the length of all fields should be the same.
            pub fn is_empty(&self) -> bool {
                let empty = self.#first_field.is_empty();
                #(debug_assert_eq!(self.#fields_names.is_empty(), empty);)*
                empty
            }

            /// Similar to [`
            #[doc = #slice_name_str]
            /// ::first()`](https://doc.rust-lang.org/std/primitive.slice.html#method.first).
            pub fn first(&self) -> Option<#ref_name<'a>> {
                if self.is_empty() {
                    None
                } else {
                    #(
                        let #fields_names_hygienic_1 = self.#fields_names.first().unwrap();
                    )*
                    Some(#ref_name{#(#fields_names: #fields_names_hygienic_1),*})
                }
            }

            /// Similar to [`
            #[doc = #slice_name_str]
            /// ::split_first()`](https://doc.rust-lang.org/std/primitive.slice.html#method.split_first).
            pub fn split_first(&self) -> Option<(#ref_name<'a>, #slice_name<'a>)> {
                if self.is_empty() {
                    None
                } else {
                    #(
                        let (#fields_names_hygienic_1, #fields_names_hygienic_2) = self.#fields_names.split_first().unwrap();
                    )*
                    let ref_ = #ref_name{#(#fields_names: #fields_names_hygienic_1),*};
                    let slice = #slice_name{#(#fields_names: #fields_names_hygienic_2),*};
                    Some((ref_, slice))
                }
            }

            /// Similar to [`
            #[doc = #slice_name_str]
            /// ::last()`](https://doc.rust-lang.org/std/primitive.slice.html#method.last).
            pub fn last(&self) -> Option<#ref_name<'a>> {
                if self.is_empty() {
                    None
                } else {
                    #(
                        let #fields_names_hygienic_1 = self.#fields_names.last().unwrap();
                    )*
                    Some(#ref_name{#(#fields_names: #fields_names_hygienic_1),*})
                }
            }

            /// Similar to [`
            #[doc = #slice_name_str]
            /// ::split_last()`](https://doc.rust-lang.org/std/primitive.slice.html#method.split_last).
            pub fn split_last(&self) -> Option<(#ref_name<'a>, #slice_name<'a>)> {
                if self.is_empty() {
                    None
                } else {
                    #(
                        let (#fields_names_hygienic_1, #fields_names_hygienic_2) = self.#fields_names.split_last().unwrap();
                    )*
                    let ref_ = #ref_name{#(#fields_names: #fields_names_hygienic_1),*};
                    let slice = #slice_name{#(#fields_names: #fields_names_hygienic_2),*};
                    Some((ref_, slice))
                }
            }

            /// Similar to [`
            #[doc = #slice_name_str]
            /// ::split_at()`](https://doc.rust-lang.org/std/primitive.slice.html#method.split_at).
            pub fn split_at(&self, mid: usize) -> (#slice_name<'a>, #slice_name<'a>) {
                #(
                    let (#fields_names_hygienic_1, #fields_names_hygienic_2) = self.#fields_names.split_at(mid);
                )*
                let left = #slice_name{#(#fields_names: #fields_names_hygienic_1),*};
                let right = #slice_name{#(#fields_names: #fields_names_hygienic_2),*};
                (left, right)
            }

            /// Similar to [`
            #[doc = #slice_name_str]
            /// ::get()`](https://doc.rust-lang.org/std/primitive.slice.html#method.get).
            pub fn get<'b, I>(&'b self, index: I) -> Option<I::RefOutput>
            where
                I: ::soa_derive::SoAIndex<#slice_name<'b>>,
                'a: 'b
            {
                let slice: #slice_name<'b> = self.reborrow();
                index.get(slice)
            }

            /// Similar to [`
            #[doc = #slice_name_str]
            /// ::get_unchecked()`](https://doc.rust-lang.org/std/primitive.slice.html#method.get_unchecked).
            pub unsafe fn get_unchecked<'b, I>(&'b self, index: I) -> I::RefOutput
            where
                I: ::soa_derive::SoAIndex<#slice_name<'b>>,
                'a: 'b
            {
                let slice: #slice_name<'b> = self.reborrow();
                index.get_unchecked(slice)
            }

            /// Similar to [`std::ops::Index` trait](https://doc.rust-lang.org/std/ops/trait.Index.html) on
            #[doc = #slice_name_str]
            /// .
            /// This is required because we cannot implement `std::ops::Index` directly since it requires returning a reference.
            pub fn index<'b, I>(&'b self, index: I) -> I::RefOutput
            where
                I: ::soa_derive::SoAIndex<#slice_name<'b>>,
                'a: 'b
            {
                let slice: #slice_name<'b> = self.reborrow();
                index.index(slice)
            }

            /// Reborrows the slices in a narrower lifetime
            pub fn reborrow<'b>(&'b self) -> #slice_name<'b>
            where
                'a: 'b
            {
                #slice_name {
                    #slice_reborrow
                }
            }

            /// Similar to [`
            #[doc = #slice_name_str]
            /// ::as_ptr()`](https://doc.rust-lang.org/std/primitive.slice.html#method.as_ptr).
            pub fn as_ptr(&self) -> #ptr_name {
                #ptr_name {
                    #(#fields_names: self.#fields_names.as_ptr(),)*
                }
            }

            /// Similar to [`std::slice::from_raw_parts()`](https://doc.rust-lang.org/std/slice/fn.from_raw_parts.html).
            pub unsafe fn from_raw_parts<'b>(data: #ptr_name, len: usize) -> #slice_name<'b> {
                #slice_name {
                    #slice_from_raw_parts
                }
            }
        }
    };

    if input.attrs.derive_clone {
        generated.append_all(quote!{
            #[allow(dead_code)]
            impl<'a> #slice_name<'a> {
                /// Similar to [`
                #[doc = #slice_name_str]
                /// ::to_vec()`](https://doc.rust-lang.org/std/primitive.slice.html#method.to_vec).
                pub fn to_vec(&self) -> #vec_name {
                    #vec_name {
                        #(#fields_names: self.#fields_names.to_vec(),)*
                    }
                }
            }
        });
    }

    return generated;
}

pub fn derive_mut(input: &Input) -> TokenStream {
    let visibility = &input.visibility;
    let slice_name = &input.slice_name();
    let slice_mut_name = &input.slice_mut_name();
    let vec_name = &input.vec_name();
    let attrs = &input.attrs.slice_mut;
    let ref_mut_name = &input.ref_mut_name();
    let ptr_name = &input.ptr_name();
    let ptr_mut_name = &input.ptr_mut_name();

    let slice_name_str = format!("[{}]", input.name);
    let doc_url = format!("[`{0}`](struct.{0}.html)", input.name);
    let slice_doc_url = format!("[`{0}`](struct.{0}.html)", slice_name);
    let slice_mut_doc_url = format!("[`{0}`](struct.{0}.html)", slice_mut_name);
    let vec_doc_url = format!("[`{0}`](struct.{0}.html)", input.vec_name());

    let fields_names = input.fields.iter()
                                   .map(|field| field.ident.clone().unwrap())
                                   .collect::<Vec<_>>();

    let fields_names_1 = &fields_names;
    let fields_names_2 = &fields_names;
    let first_field = &fields_names[0];
    let slice_names_1 = &input.fields.iter()
        .enumerate()
        .map(|(i, _)| Ident::new(&format!("___soa_derive_private_slice_1_{}", i), Span::call_site()))
        .collect::<Vec<_>>();
    let slice_names_2 = &input.fields.iter()
        .enumerate()
        .map(|(i, _)| Ident::new(&format!("___soa_derive_private_slice_2_{}", i), Span::call_site()))
        .collect::<Vec<_>>();

    let get_field_doc = |field_ident: &Ident| {
        format!("A mutable slice of `{0}` from a [`{1}`](struct.{1}.html)", field_ident, vec_name)
    };

    let slice_fields = input.field_seq_by_nested_soa(
        |field_ident, field_type| {
            let doc = get_field_doc(field_ident);
            quote! {
                #[doc = #doc]
                pub #field_ident: &'a mut [#field_type],
            }
        },
        |field_ident, field_type| {
            let doc = get_field_doc(field_ident);
            quote! {
                #[doc = #doc]
                pub #field_ident: <#field_type as soa_derive::SoASlice<'a>>::SliceMut,
            }
        },
    );

    let slice_as_ref = input.field_seq_by_nested_soa(
        |field_ident, _| {
            quote! {
                #field_ident: self.#field_ident,
            }
        },
        |field_ident, _| {
            quote! {
                #field_ident: self.#field_ident.as_ref(),
            }
        },
    );

    let slice_as_slice = input.field_seq_by_nested_soa(
        |field_ident, _| {
            quote! {
                #field_ident: &self.#field_ident,
            }
        },
        |field_ident, _| {
            quote! {
                #field_ident: self.#field_ident.as_slice(),
            }
        },
    );

    let slice_reborrow = input.field_seq_by_nested_soa(
        |field_ident, _| {
            quote! {
                #field_ident: &mut self.#field_ident,
            }
        },
        |field_ident, _| {
            quote! {
                #field_ident: self.#field_ident.reborrow(),
            }
        },
    );

    let slice_from_raw_parts_mut = input.field_seq_by_nested_soa(
        |field_ident, _| {
            quote! {
                #field_ident: ::std::slice::from_raw_parts_mut(data.#field_ident, len),
            }
        },
        |field_ident, field_type| {
            quote! {
                #field_ident: <#field_type as soa_derive::SoASlice>::SliceMut::from_raw_parts_mut(data.#field_ident, len),
            }
        },
    );

    let mut generated = quote! {
        /// A mutable slice of
        #[doc = #doc_url]
        /// inside a
        #[doc = #vec_doc_url]
        /// .
        #[allow(dead_code)]
        #(#[#attrs])*
        #visibility struct #slice_mut_name<'a> {
            #slice_fields
        }

        #[allow(dead_code)]
        impl<'a> #slice_mut_name<'a> {
            /// Convert a
            #[doc = #slice_mut_doc_url]
            /// to a
            #[doc = #slice_doc_url]
            /// in order to be able to use the methods on the non mutable
            /// version of the slices.
            pub fn as_ref(&self) -> #slice_name {
                #slice_name {
                    #slice_as_ref
                }
            }

            /// Similar to [`
            #[doc = #slice_name_str]
            /// ::len()`](https://doc.rust-lang.org/std/primitive.slice.html#method.len),
            /// the length of all fields should be the same.
            pub fn len(&self) -> usize {
                let len = self.#first_field.len();
                #(debug_assert_eq!(self.#fields_names_1.len(), len);)*
                len
            }

            /// Similar to [`
            #[doc = #slice_name_str]
            /// ::is_empty()`](https://doc.rust-lang.org/std/primitive.slice.html#method.is_empty),
            /// the length of all fields should be the same.
            pub fn is_empty(&self) -> bool {
                let empty = self.#first_field.is_empty();
                #(debug_assert_eq!(self.#fields_names_1.is_empty(), empty);)*
                empty
            }

            /// Similar to [`
            #[doc = #slice_name_str]
            /// ::first_mut()`](https://doc.rust-lang.org/std/primitive.slice.html#method.first_mut).
            pub fn first_mut(&mut self) -> Option<#ref_mut_name> {
                if self.is_empty() {
                    None
                } else {
                    #(
                        let #fields_names_1 = self.#fields_names_2.first_mut().unwrap();
                    )*
                    Some(#ref_mut_name{#(#fields_names_1: #fields_names_2),*})
                }
            }

            /// Similar to [`
            #[doc = #slice_name_str]
            /// ::split_first_mut()`](https://doc.rust-lang.org/std/primitive.slice.html#method.split_first_mut).
            pub fn split_first_mut(&mut self) -> Option<(#ref_mut_name, #slice_mut_name)> {
                if self.is_empty() {
                    None
                } else {
                    #(
                        let (#fields_names_1, #slice_names_1) = self.#fields_names_2.split_first_mut().unwrap();
                    )*
                    let ref_ = #ref_mut_name{#(#fields_names_1: #fields_names_2),*};
                    let slice = #slice_mut_name{#(#fields_names_1: #slice_names_1),*};
                    Some((ref_, slice))
                }
            }

            /// Similar to [`
            #[doc = #slice_name_str]
            /// ::last_mut()`](https://doc.rust-lang.org/std/primitive.slice.html#method.last_mut).
            pub fn last_mut(&mut self) -> Option<#ref_mut_name> {
                if self.is_empty() {
                    None
                } else {
                    #(
                        let #fields_names_1 = self.#fields_names_2.last_mut().unwrap();
                    )*
                    Some(#ref_mut_name{#(#fields_names_1: #fields_names_2),*})
                }
            }

            /// Similar to [`
            #[doc = #slice_name_str]
            /// ::last_mut()`](https://doc.rust-lang.org/std/primitive.slice.html#method.last_mut).
            pub fn split_last_mut(&mut self) -> Option<(#ref_mut_name, #slice_mut_name)> {
                if self.is_empty() {
                    None
                } else {
                    #(
                        let (#fields_names_1, #slice_names_1) = self.#fields_names_2.split_last_mut().unwrap();
                    )*
                    let ref_ = #ref_mut_name{#(#fields_names_1: #fields_names_2),*};
                    let slice = #slice_mut_name{#(#fields_names_1: #slice_names_1),*};
                    Some((ref_, slice))
                }
            }

            /// Similar to [`
            #[doc = #slice_name_str]
            /// ::split_at_mut()`](https://doc.rust-lang.org/std/primitive.slice.html#method.split_at_mut).
            pub fn split_at_mut(&mut self, mid: usize) -> (#slice_mut_name, #slice_mut_name) {
                #(
                    let (#slice_names_1, #slice_names_2) = self.#fields_names_2.split_at_mut(mid);
                )*
                let left = #slice_mut_name{#(#fields_names_1: #slice_names_1),*};
                let right = #slice_mut_name{#(#fields_names_1: #slice_names_2),*};
                (left, right)
            }

            /// Similar to [`
            #[doc = #slice_name_str]
            /// ::swap()`](https://doc.rust-lang.org/std/primitive.slice.html#method.swap).
            pub fn swap(&mut self, a: usize, b: usize) {
                #(
                    self.#fields_names_1.swap(a, b);
                )*
            }

            /// Similar to [`
            #[doc = #slice_name_str]
            /// ::get()`](https://doc.rust-lang.org/std/primitive.slice.html#method.get).
            pub fn get<'b, I>(&'b self, index: I) -> Option<I::RefOutput>
            where
                I: ::soa_derive::SoAIndex<#slice_name<'b>>,
                'a: 'b
            {
                let slice: #slice_name<'b> = self.as_slice();
                index.get(slice)
            }

            /// Similar to [`
            #[doc = #slice_name_str]
            /// ::get_unchecked()`](https://doc.rust-lang.org/std/primitive.slice.html#method.get_unchecked).
            pub unsafe fn get_unchecked<'b, I>(&'b self, index: I) -> I::RefOutput
            where
                I: ::soa_derive::SoAIndex<#slice_name<'b>>,
                'a: 'b
            {
                let slice: #slice_name<'b> = self.as_slice();
                index.get_unchecked(slice)
            }


            /// Similar to [`std::ops::Index` trait](https://doc.rust-lang.org/std/ops/trait.Index.html) on
            #[doc = #slice_name_str]
            /// .
            /// This is required because we cannot implement that trait.
            pub fn index<'b, I>(&'b self, index: I) -> I::RefOutput
            where
                I: ::soa_derive::SoAIndex<#slice_name<'b>>,
                'a: 'b
            {
                let slice: #slice_name<'b> = self.as_slice();
                index.index(slice)
            }

            /// Similar to [`
            #[doc = #slice_name_str]
            /// ::get_mut()`](https://doc.rust-lang.org/std/primitive.slice.html#method.get_mut).
            pub fn get_mut<'b, I>(&'b mut self, index: I) -> Option<I::MutOutput>
            where
                I: ::soa_derive::SoAIndexMut<#slice_mut_name<'b>>,
                'a: 'b
            {
                let slice: #slice_mut_name<'b> = self.reborrow();
                index.get_mut(slice)
            }

            /// Similar to [`
            #[doc = #slice_name_str]
            /// ::get_unchecked_mut()`](https://doc.rust-lang.org/std/primitive.slice.html#method.get_unchecked_mut).
            pub unsafe fn get_unchecked_mut<'b, I>(&'b mut self, index: I) -> I::MutOutput
            where
                I: ::soa_derive::SoAIndexMut<#slice_mut_name<'b>>,
                'a: 'b
            {
                let slice: #slice_mut_name<'b> = self.reborrow();
                index.get_unchecked_mut(slice)
            }

            /// Similar to [`std::ops::IndexMut` trait](https://doc.rust-lang.org/std/ops/trait.IndexMut.html) on
            #[doc = #slice_name_str]
            /// .
            /// This is required because we cannot implement `std::ops::IndexMut` directly since it requires returning a mutable reference.
            pub fn index_mut<'b, I>(&'b mut self, index: I) -> I::MutOutput
            where
                I: ::soa_derive::SoAIndexMut<#slice_mut_name<'b>>,
                'a: 'b
            {
                let slice: #slice_mut_name<'b> = self.reborrow();
                index.index_mut(slice)
            }

            /// Returns a non-mutable slice from this mutable slice.
            pub fn as_slice<'b>(&'b self) -> #slice_name<'b>
            where
                'a: 'b
            {
                #slice_name {
                    #slice_as_slice
                }
            }

            /// Reborrows the slices in a narrower lifetime
            pub fn reborrow<'b>(&'b mut self) -> #slice_mut_name<'b>
            where
                'a: 'b
            {
                #slice_mut_name {
                    #slice_reborrow
                }
            }

            /// Similar to [`
            #[doc = #slice_name_str]
            /// ::as_ptr()`](https://doc.rust-lang.org/std/primitive.slice.html#method.as_ptr).
            pub fn as_ptr(&self) -> #ptr_name {
                #ptr_name {
                    #(#fields_names_1: self.#fields_names_2.as_ptr(),)*
                }
            }

            /// Similar to [`
            #[doc = #slice_name_str]
            /// ::as_mut_ptr()`](https://doc.rust-lang.org/std/primitive.slice.html#method.as_mut_ptr).
            pub fn as_mut_ptr(&mut self) -> #ptr_mut_name {
                #ptr_mut_name {
                    #(#fields_names_1: self.#fields_names_2.as_mut_ptr(),)*
                }
            }

            /// Similar to [`std::slice::from_raw_parts_mut()`](https://doc.rust-lang.org/std/slice/fn.from_raw_parts_mut.html).
            pub unsafe fn from_raw_parts_mut<'b>(data: #ptr_mut_name, len: usize) -> #slice_mut_name<'b> {
                #slice_mut_name {
                    #slice_from_raw_parts_mut
                }
            }
        }
    };

    if input.attrs.derive_clone {
        generated.append_all(quote!{
            #[allow(dead_code)]
            impl<'a> #slice_mut_name<'a> {
                /// Similar to [`
                #[doc = #slice_name_str]
                /// ::to_vec()`](https://doc.rust-lang.org/std/primitive.slice.html#method.to_vec).
                pub fn to_vec(&self) -> #vec_name {
                    #vec_name {
                        #(#fields_names_1: self.#fields_names_2.to_vec(),)*
                    }
                }
            }
        });
    }

    return generated;
}
