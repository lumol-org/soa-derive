use std::convert::TryInto;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    Data, DeriveInput, Field, Ident, Lit, Meta, MetaList, MetaNameValue, NestedMeta, Visibility,
};

/// Representing the struct we are deriving
pub struct Input {
    /// The input struct name
    pub name: Ident,
    /// The list of traits to derive passed to `soa_derive` attribute
    pub derives: Vec<Ident>,
    /// The list of fields in the struct
    pub fields: Vec<Field>,
    /// The struct overall visibility
    pub visibility: Visibility,

    pub vec_attrs: Vec<Meta>,
    pub slice_attrs: Vec<Meta>,
    pub slice_mut_attrs: Vec<Meta>,
    pub ref_attrs: Vec<Meta>,
    pub ref_mut_attrs: Vec<Meta>,
    pub ptr_attrs: Vec<Meta>,
    pub ptr_mut_attrs: Vec<Meta>,
}

impl Input {
    pub fn new(input: DeriveInput) -> Input {
        let fields = match input.data {
            Data::Struct(s) => s.fields.iter().cloned().collect::<Vec<_>>(),
            _ => panic!("#[derive(StructOfArray)] only supports structs."),
        };

        let mut derives: Vec<Ident> = vec![];
        let mut vec_attrs = Vec::new();
        let mut slice_attrs = Vec::new();
        let mut slice_mut_attrs = Vec::new();
        let mut ref_attrs = Vec::new();
        let mut ref_mut_attrs = Vec::new();
        let mut ptr_attrs = Vec::new();
        let mut ptr_mut_attrs = Vec::new();
        for attr in input.attrs {
            if let Ok(meta) = attr.parse_meta() {
                if meta.path().is_ident("soa_derive") {
                    match meta {
                        Meta::NameValue(MetaNameValue {
                            lit: Lit::Str(string),
                            ..
                        }) => {
                            for value in string.value().split(',') {
                                derives.push(Ident::new(value.trim(), Span::call_site()));
                            }
                        }
                        _ => panic!(
                            "expected #[soa_derive = \"Traits, To, Derive\"], got #[{}]",
                            quote!(#meta)
                        ),
                    }
                } else if meta.path().is_ident("soa_attr") {
                    match meta.clone() {
                        Meta::List(MetaList { nested, .. }) => {
                            let [soa_type, attr]: [NestedMeta; 2] =
                                nested.into_iter().collect::<Vec<_>>().try_into().unwrap_or_else(|_| panic!("expected #[soa_attr(\"Types, To, Add, Attribute\", \"Attribute\")], got #[{}]", quote!(#meta)));
                            let attr = match attr {
                                NestedMeta::Meta(meta) => meta,
                                NestedMeta::Lit(_) => {
                                    panic!("expected a attribute, got {}", quote!(attr))
                                }
                            };
                            let attrs_mut = match soa_type {
                                NestedMeta::Meta(Meta::Path(path)) => match path.get_ident() {
                                    Some(ident) => match ident.to_string().as_str() {
                                        "Vec" => &mut vec_attrs,
                                        "Slice" => &mut slice_attrs,
                                        "SliceMut" => &mut slice_mut_attrs,
                                        "Ref" => &mut ref_attrs,
                                        "RefMut" => &mut ref_mut_attrs,
                                        "Ptr" => &mut ptr_attrs,
                                        "PtrMut" => &mut ptr_mut_attrs,
                                        _ => panic!("expected a soa type, got {}", ident),
                                    },
                                    None => {
                                        panic!("expected a soa type, got {}", quote!(#path));
                                    }
                                },
                                _ => {
                                    panic!("expected a soa type, got {}", quote!(#soa_type));
                                }
                            };
                            attrs_mut.push(attr);
                        }
                        _ => panic!("expected #[soa_attr(...)], got #[{}]", quote!(#meta)),
                    }
                }
            }
        }

        Input {
            name: input.ident,
            derives: derives,
            fields: fields,
            visibility: input.vis,
            vec_attrs,
            slice_attrs,
            slice_mut_attrs,
            ref_attrs,
            ref_mut_attrs,
            ptr_attrs,
            ptr_mut_attrs,
        }
    }

    pub fn derive(&self) -> TokenStream {
        if self.derives.is_empty() {
            TokenStream::new()
        } else {
            let derives = &self.derives;
            quote!(
                #[derive(
                    #(#derives,)*
                )]
            )
        }
    }

    pub fn derive_with_exceptions(&self) -> TokenStream {
        if self.derives.is_empty() {
            TokenStream::new()
        } else {
            let derives = &self.derives.iter()
                                       .cloned()
                                       .filter(|name| name != "Clone")
                                       .filter(|name| name != "Deserialize")
                                       .filter(|name| name != "Serialize")
                                       .collect::<Vec<_>>();
            quote!(
                #[derive(
                    #(#derives,)*
                )]
            )
        }
    }

    pub fn vec_name(&self) -> Ident {
        Ident::new(&format!("{}Vec", self.name), Span::call_site())
    }

    pub fn slice_name(&self) -> Ident {
        Ident::new(&format!("{}Slice", self.name), Span::call_site())
    }

    pub fn slice_mut_name(&self) -> Ident {
        Ident::new(&format!("{}SliceMut", self.name), Span::call_site())
    }

    pub fn ref_name(&self) -> Ident {
        Ident::new(&format!("{}Ref", self.name), Span::call_site())
    }

    pub fn ref_mut_name(&self) -> Ident {
        Ident::new(&format!("{}RefMut", self.name), Span::call_site())
    }

    pub fn ptr_name(&self) -> Ident {
        Ident::new(&format!("{}Ptr", self.name), Span::call_site())
    }

    pub fn ptr_mut_name(&self) -> Ident {
        Ident::new(&format!("{}PtrMut", self.name), Span::call_site())
    }
}
