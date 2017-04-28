#![recursion_limit="512"]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

mod structs;
mod vec;
mod refs;

#[proc_macro_derive(StructOfArray, attributes(soa_derive))]
pub fn soa_derive(input: TokenStream) -> TokenStream {
    let source = input.to_string();
    let ast = syn::parse_macro_input(&source).unwrap();
    let input = structs::Struct::new(ast);

    let mut generated = quote::Tokens::new();
    generated.append(vec::derive(&input).as_str());
    generated.append(refs::derive(&input).as_str());
    generated.parse().unwrap()
}
