use quote::{Tokens, Ident};
use structs::Struct;

pub fn derive(input: &Struct) -> Tokens {
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
        #[derive(#(#derives,)*)]
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
        }
    }
}
