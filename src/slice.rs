use quote::{Tokens, Ident};
use structs::Struct;

pub fn derive_slice(input: &Struct) -> Tokens {
    let derives = &input.derive;
    let visibility = &input.visibility;
    let slice_name = &input.slice_name();
    let ref_name = &input.ref_name();
    let fields_names = input.fields.iter()
                                   .map(|field| field.ident.clone().unwrap())
                                   .collect::<Vec<_>>();
    let fields_names_1 = &fields_names;
    let fields_names_2 = &fields_names;
    let first_field = &fields_names[0];
    let slice_names_1 = &input.fields.iter().map(|field|
        Ident::from(format!("{}_slice_1", field.ident.as_ref().unwrap())))
        .collect::<Vec<_>>();
    let slice_names_2 = &input.fields.iter().map(|field|
        Ident::from(format!("{}_slice_2", field.ident.as_ref().unwrap())))
        .collect::<Vec<_>>();

    let fields_types = &input.fields.iter()
                                    .map(|field| &field.ty)
                                    .collect::<Vec<_>>();

    quote!{
        #derives
        #visibility struct #slice_name<'a> {
            #(pub #fields_names_1: &'a [#fields_types],)*
        }

        #[allow(dead_code)]
        impl<'a> #slice_name<'a> {
            pub fn len(&self) -> usize {
                let len = self.#first_field.len();
                #(debug_assert_eq!(self.#fields_names_1.len(), len);)*
                len
            }

            pub fn is_empty(&self) -> bool {
                let empty = self.#first_field.is_empty();
                #(debug_assert_eq!(self.#fields_names_1.is_empty(), empty);)*
                empty
            }

            pub fn first(&self) -> Option<#ref_name> {
                if self.is_empty() {
                    None
                } else {
                    #(
                        let #fields_names_1 = self.#fields_names_2.first().unwrap();
                    )*
                    Some(#ref_name{#(#fields_names_1: #fields_names_2),*})
                }
            }

            pub fn split_first(&self) -> Option<(#ref_name, #slice_name)> {
                if self.is_empty() {
                    None
                } else {
                    #(
                        let (#fields_names_1, #slice_names_1) = self.#fields_names_2.split_first().unwrap();
                    )*
                    let ref_ = #ref_name{#(#fields_names_1: #fields_names_2),*};
                    let slice = #slice_name{#(#fields_names_1: #slice_names_1),*};
                    Some((ref_, slice))
                }
            }

            pub fn last(&self) -> Option<#ref_name> {
                if self.is_empty() {
                    None
                } else {
                    #(
                        let #fields_names_1 = self.#fields_names_2.last().unwrap();
                    )*
                    Some(#ref_name{#(#fields_names_1: #fields_names_2),*})
                }
            }

            pub fn split_last(&self) -> Option<(#ref_name, #slice_name)> {
                if self.is_empty() {
                    None
                } else {
                    #(
                        let (#fields_names_1, #slice_names_1) = self.#fields_names_2.split_last().unwrap();
                    )*
                    let ref_ = #ref_name{#(#fields_names_1: #fields_names_2),*};
                    let slice = #slice_name{#(#fields_names_1: #slice_names_1),*};
                    Some((ref_, slice))
                }
            }

            pub fn split_at(&self, mid: usize) -> (#slice_name, #slice_name) {
                #(
                    let (#slice_names_1, #slice_names_2) = self.#fields_names_2.split_at(mid);
                )*
                let left = #slice_name{#(#fields_names_1: #slice_names_1),*};
                let right = #slice_name{#(#fields_names_1: #slice_names_2),*};
                (left, right)
            }

            pub fn get(&self, i: usize) -> Option<#ref_name> {
                if self.is_empty() || i >= self.len() {
                    None
                } else {
                    Some(#ref_name {
                        #(#fields_names_1: self.#fields_names_2.get(i).unwrap(),)*
                    })
                }
            }

            pub unsafe fn get_unchecked(&self, i: usize) -> #ref_name {
                #ref_name {
                    #(#fields_names_1: self.#fields_names_2.get_unchecked(i),)*
                }
            }
        }
    }
}


pub fn derive_slice_mut(input: &Struct) -> Tokens {
    let derives = &input.derive;
    let visibility = &input.visibility;
    let slice_mut_name = &input.slice_mut_name();
    let ref_name = &input.ref_name();
    let ref_mut_name = &input.ref_mut_name();
    let fields_names = input.fields.iter()
                                   .map(|field| field.ident.clone().unwrap())
                                   .collect::<Vec<_>>();
    let fields_names_1 = &fields_names;
    let fields_names_2 = &fields_names;
    let first_field = &fields_names[0];
    let slice_names_1 = &input.fields.iter().map(|field|
        Ident::from(format!("{}_slice_1", field.ident.as_ref().unwrap())))
        .collect::<Vec<_>>();
    let slice_names_2 = &input.fields.iter().map(|field|
        Ident::from(format!("{}_slice_2", field.ident.as_ref().unwrap())))
        .collect::<Vec<_>>();

    let fields_types = &input.fields.iter()
                                    .map(|field| &field.ty)
                                    .collect::<Vec<_>>();

    quote!{
        #derives
        #visibility struct #slice_mut_name<'a> {
            #(pub #fields_names_1: &'a mut [#fields_types],)*
        }

        #[allow(dead_code)]
        impl<'a> #slice_mut_name<'a> {
            pub fn len(&self) -> usize {
                let len = self.#first_field.len();
                #(debug_assert_eq!(self.#fields_names_1.len(), len);)*
                len
            }

            pub fn is_empty(&self) -> bool {
                let empty = self.#first_field.is_empty();
                #(debug_assert_eq!(self.#fields_names_1.is_empty(), empty);)*
                empty
            }

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

            pub fn split_at_mut(&mut self, mid: usize) -> (#slice_mut_name, #slice_mut_name) {
                #(
                    let (#slice_names_1, #slice_names_2) = self.#fields_names_2.split_at_mut(mid);
                )*
                let left = #slice_mut_name{#(#fields_names_1: #slice_names_1),*};
                let right = #slice_mut_name{#(#fields_names_1: #slice_names_2),*};
                (left, right)
            }

            pub fn swap(&mut self, a: usize, b: usize) {
                #(
                    self.#fields_names_1.swap(a, b);
                )*
            }

            pub fn get(&self, i: usize) -> Option<#ref_name> {
                if self.is_empty() || i >= self.len() {
                    None
                } else {
                    Some(#ref_name {
                        #(#fields_names_1: self.#fields_names_2.get(i).unwrap(),)*
                    })
                }
            }

            pub unsafe fn get_unchecked(&self, i: usize) -> #ref_name {
                #ref_name {
                    #(#fields_names_1: self.#fields_names_2.get_unchecked(i),)*
                }
            }

            pub fn get_mut(&mut self, i: usize) -> Option<#ref_mut_name> {
                if self.is_empty() || i >= self.len() {
                    None
                } else {
                    Some(#ref_mut_name {
                        #(#fields_names_1: self.#fields_names_2.get_mut(i).unwrap(),)*
                    })
                }
            }

            pub unsafe fn get_unchecked_mut(&mut self, i: usize) -> #ref_mut_name {
                #ref_mut_name {
                    #(#fields_names_1: self.#fields_names_2.get_unchecked_mut(i),)*
                }
            }
        }
    }
}
