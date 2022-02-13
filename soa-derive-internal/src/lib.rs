#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::needless_return, clippy::redundant_field_names)]
#![allow(clippy::use_self, clippy::too_many_lines, clippy::missing_panics_doc)]
// TODO: improve the code and make it simpler to read
#![allow(clippy::cognitive_complexity)]

extern crate proc_macro;

use proc_macro2::{TokenStream};
use quote::TokenStreamExt;

mod index;
#[macro_use]
mod input;
mod iter;
mod ptr;
mod refs;
mod slice;
mod vec;

#[proc_macro_derive(StructOfArray, attributes(soa_derive, soa_attr, nested_soa))]
pub fn soa_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();
    let input = input::Input::new(ast);

    let mut generated = TokenStream::new();
    generated.append_all(vec::derive(&input));
    generated.append_all(refs::derive(&input));
    generated.append_all(ptr::derive(&input));
    generated.append_all(slice::derive(&input));
    generated.append_all(slice::derive_mut(&input));
    generated.append_all(index::derive(&input));
    generated.append_all(iter::derive(&input));
    generated.append_all(derive_trait(&input));
    generated.into()
}

use crate::input::Input;
use quote::quote;
fn derive_trait(input: &Input) -> TokenStream {
    let name = &input.name;
    let vec_name = &input.vec_name();
    let ptr_name = &input.ptr_name();
    let ptr_mut_name = &input.ptr_mut_name();

    quote! {
        impl soa_derive::StructOfArray for #name {
            type Type = #vec_name;
        }
        impl soa_derive::SoAPtr for #name {
            type Ptr = #ptr_name;
            type PtrMut = #ptr_mut_name;
        }
    }
}