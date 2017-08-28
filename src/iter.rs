use quote::{Tokens, Ident};
use syn::Visibility;
use structs::Struct;

pub fn derive(input: &Struct) -> Tokens {
    let name = &input.name;
    let visibility = &input.visibility;
    let detail_mod = Ident::from(format!("__detail_{}", name));
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
    let first_field = &fields_names[0];

    let fields_types = &input.fields.iter()
                                    .map(|field| &field.ty)
                                    .collect::<Vec<_>>();

    let mut generated = quote! {
        #[allow(non_snake_case, dead_code)]
        mod #detail_mod {
            use super::*;
            use std::slice;

            #visibility struct Iter<'a> {
                #(#fields_names_1: slice::Iter<'a, #fields_types>,)*
            }

            impl<'a> Iterator for Iter<'a> {
                type Item = #ref_name<'a>;
                fn next(&mut self) -> Option<#ref_name<'a>> {
                    #(let #fields_names_1 = self.#fields_names_2.next();)*
                    if #first_field.is_none() {
                        None
                    } else {
                        Some(#ref_name {
                            #(#fields_names_1: #fields_names_2.unwrap(),)*
                        })
                    }
                }
            }

            impl #vec_name {
                #visibility fn iter(&self) -> Iter {
                    Iter {
                        #(#fields_names_1: self.#fields_names_2.iter(),)*
                    }
                }
            }

            impl<'a> #slice_name<'a> {
                #visibility fn iter(&self) -> Iter {
                    Iter {
                        #(#fields_names_1: self.#fields_names_2.iter(),)*
                    }
                }
            }

            #visibility struct IterMut<'a> {
                #(#fields_names_1: slice::IterMut<'a, #fields_types>,)*
            }

            impl<'a> Iterator for IterMut<'a> {
                type Item = #ref_mut_name<'a>;
                fn next(&mut self) -> Option<#ref_mut_name<'a>> {
                    #(let #fields_names_1 = self.#fields_names_2.next();)*
                    if #first_field.is_none() {
                        None
                    } else {
                        Some(#ref_mut_name {
                            #(#fields_names_1: #fields_names_2.unwrap(),)*
                        })
                    }
                }
            }

            impl #vec_name {
                #visibility fn iter_mut(&mut self) -> IterMut {
                    IterMut {
                        #(#fields_names_1: self.#fields_names_2.iter_mut(),)*
                    }
                }
            }

            impl<'a> #slice_mut_name<'a> {
                #visibility fn iter(&mut self) -> Iter {
                    Iter {
                        #(#fields_names_1: self.#fields_names_2.iter(),)*
                    }
                }

                #visibility fn iter_mut(&mut self) -> IterMut {
                    IterMut {
                        #(#fields_names_1: self.#fields_names_2.iter_mut(),)*
                    }
                }
            }
        }
    };

    if let Visibility::Public = *visibility {
        generated.append(quote!{
            impl<'a> IntoIterator for &'a #slice_name<'a> {
                type Item = #ref_name<'a>;
                type IntoIter = #detail_mod::Iter<'a>;

                fn into_iter(self) -> Self::IntoIter {
                    self.iter()
                }
            }

            impl<'a> IntoIterator for &'a #vec_name {
                type Item = #ref_name<'a>;
                type IntoIter = #detail_mod::Iter<'a>;

                fn into_iter(self) -> Self::IntoIter {
                    self.iter()
                }
            }

            impl<'a> IntoIterator for &'a mut #slice_mut_name<'a> {
                type Item = #ref_mut_name<'a>;
                type IntoIter = #detail_mod::IterMut<'a>;

                fn into_iter(self) -> Self::IntoIter {
                    self.iter_mut()
                }
            }

            impl<'a> IntoIterator for &'a mut #vec_name {
                type Item = #ref_mut_name<'a>;
                type IntoIter = #detail_mod::IterMut<'a>;

                fn into_iter(self) -> Self::IntoIter {
                    self.iter_mut()
                }
            }
        });
    }

    return generated;
}
