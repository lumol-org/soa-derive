use proc_macro2::TokenStream;
use quote::quote;

use crate::input::Input;

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

    let fields_names = input.fields.iter()
                                   .map(|field| field.ident.clone().unwrap())
                                   .collect::<Vec<_>>();
    let fields_names_1 = &fields_names;
    let fields_names_2 = &fields_names;

    let fields_types = &input.fields.iter()
                                    .map(|field| &field.ty)
                                    .collect::<Vec<_>>();

    let fields_doc = fields_names.iter()
                                 .map(|field| format!("A reference to a `{0}` from a [`{1}`](struct.{1}.html)", field, vec_name))
                                 .collect::<Vec<_>>();

    let fields_mut_doc = fields_names.iter()
                                     .map(|field| format!("A mutable reference to a `{0}` from a [`{1}`](struct.{1}.html)", field, vec_name))
                                     .collect::<Vec<_>>();

    let field_attrs = input.field_attrs.iter()
        .map(|attr| &attr.ref_).collect::<Vec<_>>();

    let mut_field_attrs = input.field_attrs.iter()
        .map(|attr| &attr.ref_mut).collect::<Vec<_>>();

    quote! {
        /// A reference to a
        #[doc = #doc_url]
        /// with struct of array layout.
        #(#[#attrs])*
        #[derive(Copy, Clone)]
        #visibility struct #ref_name<'a> {
            #(
                #[doc = #fields_doc]
                #(#[#field_attrs])*
                pub #fields_names_1: &'a #fields_types,
            )*
        }

        /// A mutable reference to a
        #[doc = #doc_url]
        /// with struct of array layout.
        #(#[#mut_attrs])*
        #visibility struct #ref_mut_name<'a> {
            #(
                #[doc = #fields_mut_doc]
                #(#[#mut_field_attrs])*
                pub #fields_names_1: &'a mut #fields_types,
            )*
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
                    #(#fields_names_1: & self.#fields_names_2, )*
                }
            }

            /// Create a
            #[doc = #ref_mut_doc_url]
            /// from a mutably borrowed
            #[doc = #doc_url]
            /// .
            #visibility fn as_mut(&mut self) -> #ref_mut_name {
                #ref_mut_name {
                    #(#fields_names_1: &mut self.#fields_names_2, )*
                }
            }
        }
    }
}
