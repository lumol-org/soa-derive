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


    let fields_names = input.fields.iter()
                                   .map(|field| field.ident.clone().unwrap())
                                   .collect::<Vec<_>>();
    let fields_names_1 = &fields_names;
    let fields_names_2 = &fields_names;

    let generated = quote!{
        #[allow(non_snake_case, dead_code)]
        mod #detail_mod {
            use super::*;

            pub struct VecIter<'a> {
                pub(super) vec: &'a #vec_name,
                pub(super) n: usize,
            }

            impl<'a> VecIter<'a> {
                pub(self) fn new(vec: &'a #vec_name) -> VecIter<'a> {
                    VecIter {
                        vec,
                        n: 0,
                    }
                }
            }

            impl<'a> Iterator for VecIter<'a> {
                type Item = #ref_name<'a>;

                fn next(&mut self) -> Option<Self::Item> {
                    if self.n >= self.vec.len() {
                        return None;
                    }

                    let item = unsafe {
                        Some(#ref_name {
                            #(
                                #fields_names_1: self.vec.data.#fields_names_2.ptr().add(self.n).as_ref().unwrap(),
                            )*
                        })
                    };
                    self.n += 1;
                    item
                }

                fn size_hint(&self) -> (usize, Option<usize>) {
                    if self.n >= self.vec.len() {
                        return (0, Some(0))
                    }
                    let left = self.vec.len() - self.n;
                    (left, Some(left))
                }
            }

            pub struct VecIterMut<'a> {
                pub(super) vec: &'a mut #vec_name,
                pub(super) n: usize,
            }

            impl<'a> VecIterMut<'a> {
                pub(self) fn new(vec: &'a mut #vec_name) -> VecIterMut<'a> {
                    VecIterMut {
                        vec,
                        n: 0,
                    }
                }
            }


            impl<'a> Iterator for VecIterMut<'a> {
                type Item = #ref_mut_name<'a>;

                fn next(&mut self) -> Option<Self::Item> {
                    if self.n >= self.vec.len() {
                        return None;
                    }

                    let item = unsafe {
                        Some(#ref_mut_name {
                            #(
                                #fields_names_1: self.vec.data.#fields_names_2.ptr().add(self.n).as_mut().unwrap(),
                            )*
                        })
                    };
                    self.n += 1;
                    item
                }

                fn size_hint(&self) -> (usize, Option<usize>) {
                    if self.n >= self.vec.len() {
                        return (0, Some(0))
                    }
                    let left = self.vec.len() - self.n;
                    (left, Some(left))
                }
            }
        }

        impl #vec_name {
            pub fn iter<'a>(&'a self) -> #detail_mod::VecIter<'a> {
                #detail_mod::VecIter {
                    vec: self,
                    n: 0,
                }
            }

            pub fn iter_mut<'a>(&'a mut self) -> #detail_mod::VecIterMut<'a> {
                #detail_mod::VecIterMut {
                    vec: self,
                    n: 0,
                }
            }
        }

        impl<'a> IntoIterator for &'a #vec_name {
            type Item = #ref_name<'a>;
            type IntoIter = #detail_mod::VecIter<'a>;

            fn into_iter(self) -> Self::IntoIter {
                return self.iter()
            }
        }

        impl<'a> IntoIterator for &'a mut #vec_name {
            type Item = #ref_mut_name<'a>;
            type IntoIter = #detail_mod::VecIterMut<'a>;

            fn into_iter(self) -> Self::IntoIter {
                return self.iter_mut()
            }
        }
    };

    return generated;
}
