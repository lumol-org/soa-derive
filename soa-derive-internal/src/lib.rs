#![recursion_limit="512"]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

mod structs;
mod vec;
mod refs;
mod slice;
mod iter;

#[proc_macro_derive(StructOfArray, attributes(soa_derive))]
pub fn soa_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    let input = structs::Struct::new(ast);

    let mut generated = quote::Tokens::new();
    generated.append_all(vec::derive(&input));
    generated.append_all(refs::derive(&input));
    generated.append_all(slice::derive_slice(&input));
    generated.append_all(slice::derive_slice_mut(&input));
    generated.append_all(iter::derive(&input));
    generated.into()
}
