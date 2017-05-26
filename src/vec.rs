use quote::{Tokens, Ident};

use structs::Struct;

pub fn derive(input: &Struct) -> Tokens {
    let name = &input.name;
    let visibility = &input.visibility;
    let vec_name = Ident::from(format!("{}Vec", name));
    let slice_name = Ident::from(format!("{}Slice", name));

    let vec_fields = input.fields.iter()
        .map(|field| {
            let field_ident = field.ident.clone().unwrap();
            let field_ty = &field.ty;
            quote!{
                pub #field_ident: Vec<#field_ty>
            }
        })
        .collect::<Vec<_>>();

    let slice_fields = input.fields.iter()
        .map(|field| {
            let field_ident = field.ident.clone().unwrap();
            let field_ty = &field.ty;
            quote!{
                pub #field_ident: &'a [#field_ty]
            }
        })
        .collect::<Vec<_>>();

    let field_names = input.fields.iter()
                                  .map(|field| field.ident.clone().unwrap())
                                  .collect::<Vec<_>>();
    let field_names_1 = &field_names;
    let field_names_2 = &field_names;

    let first_field = &field_names[0];

    quote!{
        #[derive(Debug)]
        #visibility struct #vec_name {
            #(#vec_fields,)*
        }

        #[derive(Debug)]
        #visibility struct #slice_name<'a> {
            #(#slice_fields,)*
        }

        impl #vec_name {
            pub fn new() -> #vec_name {
                #vec_name {
                    #(#field_names_1 : Vec::new(),)*
                }
            }

            pub fn with_capacity(capacity: usize) -> #vec_name {
                #vec_name {
                    #(#field_names_1 : Vec::with_capacity(capacity),)*
                }
            }

            pub fn capacity(&self) -> usize {
                // We use the minimal capacity as the whole capacity
                let capacities = [
                    #(self.#field_names_1.capacity(),)*
                ];
                *capacities.iter().min().unwrap()
            }

            pub fn reserve(&mut self, additional: usize) {
                #(self.#field_names_1.reserve(additional);)*
            }

            pub fn reserve_exact(&mut self, additional: usize) {
                #(self.#field_names_1.reserve_exact(additional);)*
            }

            pub fn shrink_to_fit(&mut self) {
                #(self.#field_names_1.shrink_to_fit();)*
            }

            pub fn truncate(&mut self, len: usize) {
                #(self.#field_names_1.truncate(len);)*
            }

            pub fn push(&mut self, value: #name) {
                let #name{#(#field_names_1),*} = value;
                #(self.#field_names_1.push(#field_names_2);)*
            }

            pub fn len(&self) -> usize {
                let len = self.#first_field.len();
                #(debug_assert_eq!(self.#field_names_1.len(), len);)*
                len
            }

            pub fn as_slice(&self) -> #slice_name {
                #slice_name {
                    #(#field_names_1 : &self.#field_names_2,)*
                }
            }
        }
    }
}
