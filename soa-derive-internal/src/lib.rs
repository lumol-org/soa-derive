#![warn(clippy::all, clippy::pedantic)]

#![allow(clippy::needless_return, clippy::redundant_field_names)]
#![allow(clippy::use_self, clippy::too_many_lines, clippy::missing_panics_doc)]
#![allow(clippy::uninlined_format_args)]

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

pub(crate) mod names;

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

#[allow(unused)]
fn derive_trait(input: &Input) -> TokenStream {
    let name = &input.name;
    let vec_name = names::vec_name(name);
    let slice_name = names::slice_name(name);
    let slice_mut_name = names::slice_mut_name(name);
    let ref_name = names::ref_name(name);
    let ref_mut_name = names::ref_mut_name(name);
    let ptr_name = names::ptr_name(name);
    let ptr_mut_name = names::ptr_mut_name(name);

    quote! {
        impl soa_derive::StructOfArray for #name {
            type Type = #vec_name;
        }
    }
}
