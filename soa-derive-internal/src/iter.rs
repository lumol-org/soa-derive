use proc_macro2::{Span, TokenStream};
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
    let first_field = &fields_names[0];

    let fields_types = &input.fields.iter()
                                    .map(|field| &field.ty)
                                    .collect::<Vec<_>>();
    let first_field_type = &fields_types[0];

    let mut iter_type = quote!{
        slice::Iter<'a, #first_field_type>
    };
    let mut iter_pat = quote!{
        #first_field
    };
    let mut create_iter = quote!{
        self.#first_field.iter()
    };

    let mut iter_mut_type = quote!{
        slice::IterMut<'a, #first_field_type>
    };
    let mut create_iter_mut = quote!{
        self.#first_field.iter_mut()
    };

    if fields_types.len() > 1 {
        for field in &input.fields[1..] {
            let field_name = &field.ident;
            let field_type = &field.ty;

            iter_pat = quote!{
                (#iter_pat, #field_name)
            };

            iter_type = quote!{
                iter::Zip<#iter_type, slice::Iter<'a, #field_type>>
            };

            create_iter = quote!{
                #create_iter.zip(self.#field_name.iter())
            };

            iter_mut_type = quote!{
                iter::Zip<#iter_mut_type, slice::IterMut<'a, #field_type>>
            };

            create_iter_mut = quote!{
                #create_iter_mut.zip(self.#field_name.iter_mut())
            };
        }
    }

    let mut generated = quote! {
        #[allow(non_snake_case, dead_code)]
        mod #detail_mod {
            use super::*;
            use std::slice;
            #[allow(unused_imports)]
            use std::iter;

            #[allow(missing_debug_implementations)]
            #visibility struct Iter<'a>(pub(super) #iter_type);

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

            impl #vec_name {
                /// Get an iterator over the
                #[doc = #ref_doc_url]
                /// in this vector
                #visibility fn iter(&self) -> Iter {
                    Iter(#create_iter)
                }
            }

            impl<'a> #slice_name<'a> {
                /// Get an iterator over the
                #[doc = #ref_doc_url]
                /// in this slice.
                #visibility fn iter(&self) -> Iter {
                    Iter(#create_iter)
                }
            }

            #[allow(missing_debug_implementations)]
            #visibility struct IterMut<'a>(pub(super) #iter_mut_type);

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

            impl #vec_name {
                /// Get a mutable iterator over the
                #[doc = #ref_mut_doc_url]
                /// in this vector
                #visibility fn iter_mut(&mut self) -> IterMut {
                    IterMut(#create_iter_mut)
                }
            }

            impl<'a> #slice_mut_name<'a> {
                /// Get an iterator over the
                #[doc = #ref_doc_url]
                /// in this vector
                #visibility fn iter(&mut self) -> Iter {
                    Iter(#create_iter)
                }

                /// Get a mutable iterator over the
                #[doc = #ref_mut_doc_url]
                /// in this vector
                #visibility fn iter_mut(&mut self) -> IterMut {
                    IterMut(#create_iter_mut)
                }
            }
        }
    };

    if let Visibility::Public(_) = *visibility {
        generated.append_all(quote!{
            impl<'a> IntoIterator for #slice_name<'a> {
                type Item = #ref_name<'a>;
                type IntoIter = #detail_mod::Iter<'a>;

                fn into_iter(self) -> Self::IntoIter {
                    #detail_mod::Iter(#create_iter)
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
                    #detail_mod::Iter(#create_iter)
                }
            }

            impl<'a> IntoIterator for &'a #vec_name {
                type Item = #ref_name<'a>;
                type IntoIter = #detail_mod::Iter<'a>;

                fn into_iter(self) -> Self::IntoIter {
                    #detail_mod::Iter(#create_iter)
                }
            }

            impl<'a> IntoIterator for #slice_mut_name<'a> {
                type Item = #ref_mut_name<'a>;
                type IntoIter = #detail_mod::IterMut<'a>;

                fn into_iter(self) -> Self::IntoIter {
                    #detail_mod::IterMut(#create_iter_mut)
                }
            }

            impl<'a> IntoIterator for &'a mut #vec_name {
                type Item = #ref_mut_name<'a>;
                type IntoIter = #detail_mod::IterMut<'a>;

                fn into_iter(self) -> Self::IntoIter {
                    #detail_mod::IterMut(#create_iter_mut)
                }
            }
        });
    }

    return generated;
}
