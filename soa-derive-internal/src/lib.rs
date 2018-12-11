#![recursion_limit="512"]

extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::TokenStreamExt;

mod input;
mod vec;
mod refs;
mod ptr;
mod slice;
mod iter;

#[proc_macro_derive(StructOfArray, attributes(soa_derive))]
pub fn soa_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();
    let input = input::Input::new(ast);

    let mut generated = TokenStream::new();
    generated.append_all(vec::derive(&input));
    generated.append_all(refs::derive(&input));
    generated.append_all(ptr::derive(&input));
    generated.append_all(slice::derive_slice(&input));
    generated.append_all(slice::derive_slice_mut(&input));
    generated.append_all(iter::derive(&input));
    generated.into()
}
