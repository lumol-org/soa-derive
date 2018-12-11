use proc_macro2::{Span, TokenStream};
use syn::{Data, DeriveInput, Ident, Field, Visibility, Meta, Lit};
use quote::quote;

/// Representing the struct we are deriving
pub struct Input {
    /// The input struct name
    pub name: Ident,
    /// The list of traits to derive passed to `soa_derive` attribute
    pub derives: Vec<Ident>,
    /// The list of fields in the struct
    pub fields: Vec<Field>,
    /// The struct overall visibility
    pub visibility: Visibility
}

impl Input {
    pub fn new(input: DeriveInput) -> Input {
        let fields = match input.data {
            Data::Struct(s) => {
                s.fields.iter().cloned().collect::<Vec<_>>()
            }
            _ => panic!("#[derive(StructOfArray)] only supports structs."),
        };

        let mut derives: Vec<Ident> = vec![];
        for attr in input.attrs {
            if let Some(meta) = attr.interpret_meta() {
                if meta.name() == "soa_derive" {
                    if let Meta::NameValue(meta) = meta {
                        if let Lit::Str(string) = meta.lit {
                            for value in string.value().split(',') {
                                derives.push(Ident::new(value.trim(), Span::call_site()));
                            }
                        }
                    } else {
                        panic!("expected #[soa_derive = \"Traits, To, Derive\"], got {}", "meta TODO")
                    }
                }
            }
        }

        Input {
            name: input.ident,
            derives: derives,
            fields: fields,
            visibility: input.vis
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
