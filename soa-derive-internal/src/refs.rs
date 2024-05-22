use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use crate::input::Input;
use crate::names;

pub fn derive(input: &Input) -> TokenStream {
    let name = &input.name;
    let visibility = &input.visibility;
    let attrs = &input.attrs.ref_;
    let mut_attrs = &input.attrs.ref_mut;
    let vec_name = names::vec_name(&input.name);
    let ref_name = names::ref_name(&input.name);
    let ref_mut_name = names::ref_mut_name(&input.name);

    let fields_types = &input.fields.iter()
        .map(|field| field.ty.clone())
        .collect::<Vec<_>>();

    let doc_url = format!("[`{0}`](struct.{0}.html)", name);
    let vec_doc_url = format!("[`{0}`](struct.{0}.html)", vec_name);
    let ref_doc_url = format!("[`{0}`](struct.{0}.html)", ref_name);
    let ref_mut_doc_url = format!("[`{0}`](struct.{0}.html)", ref_mut_name);

    let fields_names = &input.fields.iter()
        .map(|field| field.ident.clone().unwrap())
        .collect::<Vec<_>>();

    let fields_names_hygienic = input.fields.iter()
        .enumerate()
        .map(|(i, _)| Ident::new(&format!("___soa_derive_private_{}", i), Span::call_site()))
        .collect::<Vec<_>>();

    let ref_fields_types = input.map_fields_nested_or(
        |_, field_type| {
            let field_ptr_type = names::ref_name(field_type);
            quote! { #field_ptr_type<'a> }
        },
        |_, field_type| quote! { &'a #field_type },
    ).collect::<Vec<_>>();

    let ref_mut_fields_types = input.map_fields_nested_or(
        |_, field_type| {
            let field_ptr_type = names::ref_mut_name(field_type);
            quote! { #field_ptr_type<'a> }
        },
        |_, field_type| quote! { &'a mut #field_type },
    ).collect::<Vec<_>>();

    let as_ref = input.map_fields_nested_or(
        |ident, _| quote! { self.#ident.as_ref() },
        |ident, _| quote! { &self.#ident },
    ).collect::<Vec<_>>();

    let as_mut = input.map_fields_nested_or(
        |ident, _| quote! { self.#ident.as_mut() },
        |ident, _| quote! { &mut self.#ident },
    ).collect::<Vec<_>>();

    let to_owned = input.map_fields_nested_or(
        |ident, _| quote! { self.#ident.to_owned() },
        |ident, _| quote! { self.#ident.clone() },
    ).collect::<Vec<_>>();

    let ref_replace = input.map_fields_nested_or(
        |ident, _| quote! { self.#ident.replace(field) },
        |ident, _| quote! { ::std::mem::replace(&mut *self.#ident, field) },
    ).collect::<Vec<_>>();

    quote! {
        /// A reference to a
        #[doc = #doc_url]
        /// with struct of array layout.
        #(#[#attrs])*
        #[derive(Copy, Clone)]
        #visibility struct #ref_name<'a> {
            #(
                /// reference to the `
                #[doc = stringify!(#fields_names)]
                ///` field of a single
                #[doc = #doc_url]
                /// inside a
                #[doc = #vec_doc_url]
                pub #fields_names: #ref_fields_types,
            )*
        }

        /// A mutable reference to a
        #[doc = #doc_url]
        /// with struct of array layout.
        #(#[#mut_attrs])*
        #visibility struct #ref_mut_name<'a> {
            #(
                /// reference to the `
                #[doc = stringify!(#fields_names)]
                ///` field of a single
                #[doc = #doc_url]
                /// inside a
                #[doc = #vec_doc_url]
                pub #fields_names: #ref_mut_fields_types,
            )*
        }

        #[allow(dead_code)]
        impl #name {
            /// Create a
            #[doc = #ref_doc_url]
            /// from a borrowed
            #[doc = #doc_url]
            /// .
            #visibility fn as_ref(&self) -> #ref_name {
                #ref_name {
                    #( #fields_names: #as_ref, )*
                }
            }

            /// Create a
            #[doc = #ref_mut_doc_url]
            /// from a mutably borrowed
            #[doc = #doc_url]
            /// .
            #visibility fn as_mut(&mut self) -> #ref_mut_name {
                #ref_mut_name {
                    #( #fields_names: #as_mut, )*
                }
            }
        }

        impl<'a> #ref_name<'a> {
            /// Convert a reference to
            #[doc = #doc_url]
            /// into an owned value. This is only available if all fields
            /// implement `Clone`.
            pub fn to_owned(&self) -> #name
                // only expose to_owned if all fields are Clone
                // https://github.com/rust-lang/rust/issues/48214#issuecomment-1150463333
                where #( for<'b> #fields_types: Clone, )*
            {
                #name {
                    #( #fields_names: #to_owned, )*
                }
            }
        }

        impl<'a> #ref_mut_name<'a> {
            /// Convert a mutable reference to
            #[doc = #doc_url]
            /// into an owned value. This is only available if all fields
            /// implement `Clone`.
            pub fn to_owned(&self) -> #name
                // only expose to_owned if all fields are Clone
                // https://github.com/rust-lang/rust/issues/48214#issuecomment-1150463333
                where #( for<'b> #fields_types: Clone, )*
            {
                #name {
                    #( #fields_names: #to_owned, )*
                }
            }

            /// Similar to [`std::mem::replace()`](https://doc.rust-lang.org/std/mem/fn.replace.html).
            pub fn replace(&mut self, val: #name) -> #name {
                #(
                    let field = unsafe { ::std::ptr::read(&val.#fields_names) };
                    let #fields_names_hygienic = #ref_replace;
                )*
                // if val implements Drop, we don't want to run it here, only
                // when the vec itself will be dropped
                ::std::mem::forget(val);

                #name{#(#fields_names: #fields_names_hygienic),*}
            }
        }
    }
}
