extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use syn::{Body, VariantData, MacroInput};
use proc_macro::TokenStream;

#[proc_macro_derive(StructOfArray)]
pub fn soa_derive(input: TokenStream) -> TokenStream {
    let source = input.to_string();
    let ast = syn::parse_macro_input(&source).unwrap();
    let generated = derive_vec(&ast);
    generated.parse().unwrap()
}

fn derive_vec(input: &MacroInput) -> quote::Tokens {
    let ident = &input.ident;
    let soa_ident = quote::Ident::from(format!("{}Vec", input.ident));
    let fields = match input.body {
        Body::Struct(ref data) => {
            match data {
                &VariantData::Struct(ref fields) => fields.clone(),
                _ => panic!("#[derive(SoA)] only supports structs."),
            }
        }
        _ => panic!("#[derive(SoA)] only supports structs."),
    };

    let vec_fields = fields.iter()
        .map(|f| {
            let field_ident = f.ident.clone().unwrap();
            let field_ty = &f.ty;
            quote!{
                pub #field_ident: Vec<#field_ty>
            }
        })
        .collect::<Vec<_>>();

    let field_idents = fields.iter().map(|f| f.ident.clone().unwrap()).collect::<Vec<_>>();
    let field_idents_1 = &field_idents;
    let field_idents_2 = &field_idents;

    let first_ident = &field_idents[0];

    quote!{
        #[derive(Debug)]
        struct #soa_ident {
            #(
                #vec_fields,
            )*
        }

        impl #soa_ident {
            pub fn new() -> Self {
                #soa_ident {
                    #(
                        #field_idents_1 : Vec::new(),
                    )*
                }
            }

            pub fn push(&mut self, value: #ident) {
                let #ident{#(#field_idents_1),*} = value;
                #(
                    self.#field_idents_1.push(#field_idents_2);
                )*
            }

            pub fn len(&self) -> usize {
                let len = self.#first_ident.len();
                #(
                    debug_assert_eq!(self.#field_idents_1.len(), len);
                )*
                len
            }
        }
    }
}
