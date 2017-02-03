use quote::{Tokens, Ident};

use structs::Struct;

pub fn derive(input: &Struct) -> Tokens {
    let name = &input.name;
    let vec_name = Ident::from(format!("{}Vec", name));

    let vec_fields = input.fields.iter()
        .map(|field| {
            let field_ident = field.ident.clone().unwrap();
            let field_ty = &field.ty;
            quote!{
                pub #field_ident: Vec<#field_ty>
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
        struct #vec_name {
            #(#vec_fields,)*
        }

        impl #vec_name {
            pub fn new() -> Self {
                #vec_name {
                    #(#field_names_1 : Vec::new(),)*
                }
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
        }
    }
}
