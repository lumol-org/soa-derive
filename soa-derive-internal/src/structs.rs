use syn::{Data, DeriveInput, Ident, Field, Visibility, Meta, Lit};
use quote::{Tokens, ToTokens};

/// Representing the struct we are deriving
pub struct Struct {
    /// The input struct name
    pub name: Ident,
    /// The list of traits to derive passed to `soa_derive` attribute
    pub derives: Vec<Ident>,
    /// The list of fields in the struct
    pub fields: Vec<Field>,
    /// The struct overall visibility
    pub visibility: Visibility
}

impl Struct {
    pub fn new(input: DeriveInput) -> Struct {
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
                                derives.push(value.trim().into())
                            }
                        }
                    } else {
                        panic!("expected #[soa_derive = \"Traits, To, Derive\"], got {}", meta.into_tokens())
                    }
                }
            }
        }

        Struct {
            name: input.ident,
            derives: derives,
            fields: fields,
            visibility: input.vis
        }
    }

    pub fn derive(&self) -> Tokens {
        if self.derives.is_empty() {
            Tokens::new()
        } else {
            let derives = &self.derives;
            quote!(
                #[derive(
                    #(#derives,)*
                )]
            )
        }
    }

    pub fn derive_with_exceptions(&self) -> Tokens {
        if self.derives.is_empty() {
            Tokens::new()
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
        format!("{}Vec", self.name).into()
    }

    pub fn slice_name(&self) -> Ident {
        format!("{}Slice", self.name).into()
    }

    pub fn slice_mut_name(&self) -> Ident {
        format!("{}SliceMut", self.name).into()
    }

    pub fn ref_name(&self) -> Ident {
        format!("{}Ref", self.name).into()
    }

    pub fn ref_mut_name(&self) -> Ident {
        format!("{}RefMut", self.name).into()
    }
}
