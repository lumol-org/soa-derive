use proc_macro2::{Span, TokenStream};
use syn::{Ident, Visibility};

use quote::TokenStreamExt;
use quote::ToTokens;
use quote::quote;

use crate::input::{Input, TokenStreamIterator};
use crate::names;

pub fn derive(input: &Input) -> TokenStream {
    let name = &input.name;
    let visibility = &input.visibility;
    let detail_mod = Ident::new(&format!("__detail_iter_{}", name.to_string().to_lowercase()), Span::call_site());
    let vec_name = names::vec_name(&input.name);
    let slice_name = names::slice_name(name);
    let slice_mut_name = names::slice_mut_name(&input.name);
    let ref_name = names::ref_name(&input.name);
    let ref_mut_name = names::ref_mut_name(&input.name);

    let ref_doc_url = format!("[`{0}`](struct.{0}.html)", ref_name);
    let ref_mut_doc_url = format!("[`{0}`](struct.{0}.html)", ref_mut_name);

    let fields_names = &input.fields.iter()
        .map(|field| field.ident.clone().unwrap())
        .collect::<Vec<_>>();

    let fields_types = &input.fields.iter()
        .map(|field| field.ty.clone())
        .collect::<Vec<_>>();

    let iter_type = input.map_fields_nested_or(
        |_, field_type| quote! { <#field_type as soa_derive::SoAIter<'a>>::Iter },
        |_, field_type| quote! { slice::Iter<'a, #field_type> },
    ).concat_by(
        |seq, next| { quote! { iter::Zip<#seq, #next> } }
    );

    let iter_mut_type = input.map_fields_nested_or(
        |_, field_type| quote! { <#field_type as soa_derive::SoAIter<'a>>::IterMut },
        |_, field_type| quote! { slice::IterMut<'a, #field_type> },
    ).concat_by(
        |seq, next| { quote! { iter::Zip<#seq, #next> } }
    );

    let create_into_iter = input.map_fields_nested_or(
        |ident, _| quote! { self.#ident.into_iter() },
        |ident, _| quote! { self.#ident.iter() },
    ).concat_by(
        |seq, next| { quote! { #seq.zip(#next) } }
    );

    let create_mut_into_iter = input.map_fields_nested_or(
        |ident, _| quote! { self.#ident.into_iter() },
        |ident, _| quote! { self.#ident.iter_mut() },
    ).concat_by(
        |seq, next| { quote! { #seq.zip(#next) } }
    );

    let iter_visibility = match visibility {
        Visibility::Inherited => quote!{pub(super)},
        other => other.to_token_stream(),
    };

    let iter_pat = fields_names.iter().fold(None, |seq, ident| {
        if let Some(seq) = seq {
            Some(quote! { (#seq, #ident) })
        } else {
            Some(quote!{ #ident })
        }
    }).expect("should be Some");

    let create_iter = fields_names.iter().fold(None, |seq, ident| {
        if let Some(seq) = seq {
            Some(quote! { #seq.zip(self.#ident.iter()) })
        } else {
            Some(quote! { self.#ident.iter() })
        }
    }).expect("should be Some");

    let create_iter_mut = fields_names.iter().fold(None, |seq, ident| {
        if let Some(seq) = seq {
            Some(quote! { #seq.zip(self.#ident.iter_mut()) })
        } else {
            Some(quote! { self.#ident.iter_mut() })
        }
    }).expect("should be Some");

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

            impl Extend<#name> for #vec_name {
                fn extend<I: IntoIterator<Item = #name>>(&mut self, iter: I) {
                    for item in iter {
                        self.push(item)
                    }
                }
            }

            impl<'a> Extend<#ref_name<'a>> for #vec_name
                // only expose if all fields are Clone
                // https://github.com/rust-lang/rust/issues/48214#issuecomment-1150463333
                where #( for<'b> #fields_types: Clone, )*
            {
                fn extend<I: IntoIterator<Item = #ref_name<'a>>>(&mut self, iter: I) {
                    self.extend(iter.into_iter().map(|item| item.to_owned()))
                }
            }
        });
    }

    return generated;
}
