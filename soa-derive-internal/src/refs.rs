use proc_macro2::{TokenStream, Ident};
use quote::quote;

use crate::input::{Input, TokenStreamIterator};

pub fn derive(input: &Input) -> TokenStream {
    let name = &input.name;
    let visibility = &input.visibility;
    let attrs = &input.attrs.ref_;
    let mut_attrs = &input.attrs.ref_mut;
    let vec_name = &input.vec_name();
    let ref_name = &input.ref_name();
    let ref_mut_name = &input.ref_mut_name();

    let doc_url = format!("[`{0}`](struct.{0}.html)", name);
    let ref_doc_url = format!("[`{0}`](struct.{0}.html)", ref_name);
    let ref_mut_doc_url = format!("[`{0}`](struct.{0}.html)", ref_mut_name);

    let get_ref_field_doc = |field_ident: &Ident| {
        format!("A reference to a `{0}` from a [`{1}`](struct.{1}.html)", field_ident, vec_name)
    };

    let get_ref_mut_field_doc = |field_ident: &Ident| {
        format!("A mutable reference to a `{0}` from a [`{1}`](struct.{1}.html)", field_ident, vec_name)
    };

    let ref_fields = input.iter_fields().map(
        |(field_ident, field_type, is_nested)| {
            let doc = get_ref_field_doc(field_ident);
            if is_nested {
                quote! {
                    #[doc = #doc]
                    pub #field_ident: <#field_type as soa_derive::SoARef<'a>>::Ref,
                }
            }
            else {
                quote! {
                    #[doc = #doc]
                    pub #field_ident: &'a #field_type,
                }
            }
        },
    ).concat();

    let ref_mut_fields = input.iter_fields().map(
        |(field_ident, field_type, is_nested)| {
            let doc = get_ref_mut_field_doc(field_ident);
            if is_nested {
                quote! {
                    #[doc = #doc]
                    pub #field_ident: <#field_type as soa_derive::SoARef<'a>>::RefMut,
                }
            }
            else {
                quote! {
                    #[doc = #doc]
                    pub #field_ident: &'a mut #field_type,
                }
            }
        },
    ).concat();

    let as_ref = input.iter_fields().map(
        |(field_ident, _, is_nested)| {
            if is_nested {
                quote! {
                    #field_ident: self.#field_ident.as_ref(),
                }
            }
            else {
                quote! {
                    #field_ident: & self.#field_ident,
                }
            }
        },
    ).concat();

    let as_mut = input.iter_fields().map(
        |(field_ident, _, is_nested)| {
            if is_nested {
                quote! {
                    #field_ident: self.#field_ident.as_mut(),
                }
            }
            else {
                quote! {
                    #field_ident: &mut self.#field_ident,
                }
            }
        },
    ).concat();

    quote! {
        /// A reference to a
        #[doc = #doc_url]
        /// with struct of array layout.
        #(#[#attrs])*
        #[derive(Copy, Clone)]
        #visibility struct #ref_name<'a> {
            #ref_fields
        }

        /// A mutable reference to a
        #[doc = #doc_url]
        /// with struct of array layout.
        #(#[#mut_attrs])*
        #visibility struct #ref_mut_name<'a> {
            #ref_mut_fields
        }

        #[allow(dead_code)]
        impl #name {
            /// Create a
            #[doc = #ref_doc_url]
            /// from a borrowed
            #[doc = #doc_url]
            /// .
            #visibility fn as_ref(&self) -> #ref_name {
                #ref_name {
                    #as_ref
                }
            }

            /// Create a
            #[doc = #ref_mut_doc_url]
            /// from a mutably borrowed
            #[doc = #doc_url]
            /// .
            #visibility fn as_mut(&mut self) -> #ref_mut_name {
                #ref_mut_name {
                    #as_mut
                }
            }
        }
    }
}
