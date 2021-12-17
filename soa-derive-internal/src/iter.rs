use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::{Ident, Visibility};
use quote::TokenStreamExt;
use quote::quote;

use crate::input::Input;

pub fn derive(input: &Input) -> TokenStream {
    let name = &input.name;
    let visibility = &input.visibility;
    let detail_mod = Ident::new(&format!("__detail_iter_{}", name.to_string().to_lowercase()), Span::call_site());
    let vec_name = &input.vec_name();
    let slice_name = &input.slice_name();
    let slice_mut_name = &input.slice_mut_name();
    let ref_name = &input.ref_name();
    let ref_mut_name = &input.ref_mut_name();

    let ref_doc_url = format!("[`{0}`](struct.{0}.html)", ref_name);
    let ref_mut_doc_url = format!("[`{0}`](struct.{0}.html)", ref_mut_name);

    let fields_names = &input.fields.iter()
                                    .map(|field| field.ident.clone().unwrap())
                                    .collect::<Vec<_>>();

    let iter_type = input.fold_fields(
        |_, field_type, is_nested| {
            if is_nested {
                quote! {
                    <#field_type as soa_derive::SoAIter<'a>>::Iter
                }
            }
            else {
                quote! {
                    slice::Iter<'a, #field_type>
                }
            }
        },
        |seq, next| {
            *seq = quote! {
                iter::Zip<#seq, #next>
            }
        },
    );

    let iter_mut_type = input.fold_fields(
        |_, field_type, is_nested| {
            if is_nested {
                quote! {
                    <#field_type as soa_derive::SoAIter<'a>>::IterMut
                }
            }
            else {
                quote! {
                    slice::IterMut<'a, #field_type>
                }
            }
        },
        |seq, next| {
            *seq = quote! {
                iter::Zip<#seq, #next>
            }
        },
    );

    let iter_pat = input.fold_fields(
        |field_ident, _, _| {
            quote! { #field_ident }
        },
        |seq, next| {
            *seq = quote! { (#seq, #next) }
        }
    );

    let create_iter = input.fold_fields(
        |field_ident, _, _| {
            quote! { self.#field_ident.iter() }
        },
        |seq, next| {
            *seq = quote! { #seq.zip(#next) }
        }
    );

    let create_iter_mut = input.fold_fields(
        |field_ident, _, _| {
            quote! { self.#field_ident.iter_mut() }
        },
        |seq, next| {
            *seq = quote! { #seq.zip(#next) }
        }
    );

    let create_into_iter = input.fold_fields(
        |field_ident, _, is_nested| {
            if is_nested {
                quote! { self.#field_ident.into_iter() }
            }
            else {
                quote! { self.#field_ident.iter() }
            }
        },
        |seq, next| {
            *seq = quote! { #seq.zip(#next) }
        }
    );

    let create_mut_into_iter = input.fold_fields(
        |field_ident, _, is_nested| {
            if is_nested {
                quote! { self.#field_ident.into_iter() }
            }
            else {
                quote! { self.#field_ident.iter_mut() }
            }
        },
        |seq, next| {
            *seq = quote! { #seq.zip(#next) }
        }
    );

    let iter_visibility = match visibility {
        Visibility::Inherited => quote!{pub(super)},
        other => other.to_token_stream(),
    };

    let mut generated = quote! {
        #[allow(non_snake_case, dead_code)]
        mod #detail_mod {
            use super::*;
            use std::slice;
            #[allow(unused_imports)]
            use std::iter;

            #[allow(missing_debug_implementations)]
            #iter_visibility struct Iter<'a>(pub(super) #iter_type);

            impl<'a> Iterator for Iter<'a> {
                type Item = #ref_name<'a>;

                #[inline]
                fn next(&mut self) -> Option<#ref_name<'a>> {
                    self.0.next().and_then(|#iter_pat|
                        Some(#ref_name{
                            #(#fields_names,)*
                        })
                    )
                }

                #[inline]
                fn size_hint(&self) -> (usize, Option<usize>) {
                    self.0.size_hint()
                }
            }

            impl<'a> DoubleEndedIterator for Iter<'a> {

                #[inline]
                fn next_back(&mut self) -> Option<#ref_name<'a>> {
                    self.0.next_back().and_then(|#iter_pat|
                        Some(#ref_name{
                            #(#fields_names,)*
                        })
                    )
                }
            }
            impl<'a> ExactSizeIterator for Iter<'a> {
                fn len(&self) -> usize {
                    self.0.len()
                }
            }

            impl #vec_name {
                /// Get an iterator over the
                #[doc = #ref_doc_url]
                /// in this vector
                #visibility fn iter(&self) -> Iter {
                    self.as_slice().into_iter()
                }
            }

            impl<'a> #slice_name<'a> {
                /// Get an iterator over the
                #[doc = #ref_doc_url]
                /// in this slice.
                #visibility fn iter(&self) -> Iter {
                    Iter(#create_iter)
                }
                /// Get an iterator over the
                #[doc = #ref_doc_url]
                /// in this slice.
                #visibility fn into_iter(self) -> Iter<'a> {
                    Iter(#create_into_iter)
                }
            }

            #[allow(missing_debug_implementations)]
            #iter_visibility struct IterMut<'a>(pub(super) #iter_mut_type);

            impl<'a> Iterator for IterMut<'a> {
                type Item = #ref_mut_name<'a>;

                #[inline]
                fn next(&mut self) -> Option<#ref_mut_name<'a>> {
                    self.0.next().and_then(|#iter_pat|
                        Some(#ref_mut_name{
                            #(#fields_names,)*
                        })
                    )
                }

                #[inline]
                fn size_hint(&self) -> (usize, Option<usize>) {
                    self.0.size_hint()
                }
            }

            impl<'a> DoubleEndedIterator for IterMut<'a> {

                #[inline]
                fn next_back(&mut self) -> Option<#ref_mut_name<'a>> {
                    self.0.next_back().and_then(|#iter_pat|
                        Some(#ref_mut_name{
                            #(#fields_names,)*
                        })
                    )
                }
            }
            impl<'a> ExactSizeIterator for IterMut<'a> {
                fn len(&self) -> usize {
                    self.0.len()
                }
            }

            impl #vec_name {
                /// Get a mutable iterator over the
                #[doc = #ref_mut_doc_url]
                /// in this vector
                #visibility fn iter_mut(&mut self) -> IterMut {
                    self.as_mut_slice().into_iter()
                }
            }

            impl<'a> #slice_mut_name<'a> {
                /// Get an iterator over the
                #[doc = #ref_doc_url]
                /// in this vector
                #visibility fn iter(&mut self) -> Iter {
                    self.as_ref().into_iter()
                }

                /// Get a mutable iterator over the
                #[doc = #ref_mut_doc_url]
                /// in this vector
                #visibility fn iter_mut(&mut self) -> IterMut {
                    IterMut(#create_iter_mut)
                }
                /// Get a mutable iterator over the
                #[doc = #ref_mut_doc_url]
                /// in this vector
                #visibility fn into_iter(self) -> IterMut<'a> {
                    IterMut(#create_mut_into_iter)
                }
            }

            impl<'a> soa_derive::SoAIter<'a> for #name {
                type Iter = Iter<'a>;
                type IterMut = IterMut<'a>;
            }


        }
    };

    if let Visibility::Public(_) = *visibility {
        generated.append_all(quote!{
            impl<'a> IntoIterator for #slice_name<'a> {
                type Item = #ref_name<'a>;
                type IntoIter = #detail_mod::Iter<'a>;

                fn into_iter(self) -> Self::IntoIter {
                    #detail_mod::Iter(#create_into_iter)
                }
            }


            impl std::iter::FromIterator<#name> for #vec_name {
                fn from_iter<T: IntoIterator<Item=#name>>(iter: T) -> Self {
                    let mut result = #vec_name::new();
                    for element in iter {
                        #(
                            (result.#fields_names).push(element.#fields_names);
                        )*
                    }
                    result
                }
            }

            impl<'a, 'b> IntoIterator for &'a #slice_name<'b> {
                type Item = #ref_name<'a>;
                type IntoIter = #detail_mod::Iter<'a>;

                fn into_iter(self) -> Self::IntoIter {
                    #detail_mod::Iter(#create_into_iter)
                }
            }

            impl<'a> IntoIterator for &'a #vec_name {
                type Item = #ref_name<'a>;
                type IntoIter = #detail_mod::Iter<'a>;

                fn into_iter(self) -> Self::IntoIter {
                    self.as_slice().into_iter()
                }
            }

            impl<'a> IntoIterator for #slice_mut_name<'a> {
                type Item = #ref_mut_name<'a>;
                type IntoIter = #detail_mod::IterMut<'a>;

                fn into_iter(self) -> Self::IntoIter {
                    #detail_mod::IterMut(#create_mut_into_iter)
                }
            }

            impl<'a> IntoIterator for &'a mut #vec_name {
                type Item = #ref_mut_name<'a>;
                type IntoIter = #detail_mod::IterMut<'a>;

                fn into_iter(self) -> Self::IntoIter {
                    self.as_mut_slice().into_iter()
                }
            }
        });
    }

    return generated;
}
