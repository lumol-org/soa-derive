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

    let unnested_fields_names = input.unnested_fields.iter()
                                   .map(|field| field.ident.clone().unwrap())
                                   .collect::<Vec<_>>();
    let nested_fields_names = input.nested_fields.iter()
                                   .map(|field| field.ident.clone().unwrap())
                                   .collect::<Vec<_>>();
    let unnested_fields_types = &input.unnested_fields.iter()
                                    .map(|field| &field.ty)
                                    .collect::<Vec<_>>();
    let nested_fields_types = &input.nested_fields.iter()
                                    .map(|field| &field.ty)
                                    .collect::<Vec<_>>();

    let unnested_fields_doc = unnested_fields_names.iter()
                                 .map(|field| format!("A reference to a `{0}` from a [`{1}`](struct.{1}.html)", field, vec_name))
                                 .collect::<Vec<_>>();

    let unnested_fields_mut_doc = unnested_fields_names.iter()
                                     .map(|field| format!("A mutable reference to a `{0}` from a [`{1}`](struct.{1}.html)", field, vec_name))
                                     .collect::<Vec<_>>();
    let nested_fields_doc = nested_fields_names.iter()
                                 .map(|field| format!("A reference to a `{0}` from a [`{1}`](struct.{1}.html)", field, vec_name))
                                 .collect::<Vec<_>>();

    let nested_fields_mut_doc = nested_fields_names.iter()
                                     .map(|field| format!("A mutable reference to a `{0}` from a [`{1}`](struct.{1}.html)", field, vec_name))
                                     .collect::<Vec<_>>();

    quote! {
        /// A reference to a
        #[doc = #doc_url]
        /// with struct of array layout.
        #(#[#attrs])*
        #[derive(Copy, Clone)]
        #visibility struct #ref_name<'a> {
            #(
                #[doc = #unnested_fields_doc]
                pub #unnested_fields_names: &'a #unnested_fields_types,
            )*
            #(
                #[doc = #nested_fields_doc]
                pub #nested_fields_names: <#nested_fields_types as soa_derive::SoARef<'a>>::Ref,
            )*
        }

        /// A mutable reference to a
        #[doc = #doc_url]
        /// with struct of array layout.
        #(#[#mut_attrs])*
        #visibility struct #ref_mut_name<'a> {
            #(
                #[doc = #unnested_fields_mut_doc]
                pub #unnested_fields_names: &'a mut #unnested_fields_types,
            )*
            #(
                #[doc = #nested_fields_mut_doc]
                pub #nested_fields_names: <#nested_fields_types as soa_derive::SoARef<'a>>::RefMut,
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
                    #(#unnested_fields_names: & self.#unnested_fields_names, )*
                    #(#nested_fields_names: self.#nested_fields_names.as_ref(), )*
                }
            }

            /// Create a
            #[doc = #ref_mut_doc_url]
            /// from a mutably borrowed
            #[doc = #doc_url]
            /// .
            #visibility fn as_mut(&mut self) -> #ref_mut_name {
                #ref_mut_name {
                    #(#unnested_fields_names: &mut self.#unnested_fields_names, )*
                    #(#nested_fields_names: self.#nested_fields_names.as_mut(), )*
                }
            }
        }
    }
}
