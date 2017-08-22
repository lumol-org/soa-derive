use syn::{Body, VariantData, MacroInput, Ident, Field, Visibility, MetaItem};
use syn::Lit;
use quote;

/// Representing the struct we are deriving
pub struct Struct {
    /// The input struct name
    pub name: Ident,
    /// The list of traits to derive passed to `soa_derive` attribute
    pub derives: Vec<String>,
    /// The list of fields in the struct
    pub fields: Vec<Field>,
    /// The struct overall visibility
    pub visibility: Visibility
}

impl Struct {
    pub fn new(input: MacroInput) -> Struct {
        let fields = match input.body {
            Body::Struct(data) => {
                match data {
                    VariantData::Struct(fields) => fields,
                    _ => panic!("#[derive(StructOfArray)] only supports structs."),
                }
            }
            _ => panic!("#[derive(StructOfArray)] only supports structs."),
        };

        let mut derives: Vec<String> = vec![];
        'attrs: for attr in input.attrs {
            if let MetaItem::NameValue(name, value) = attr.value {
                if name.as_ref() == "soa_derive" {
                    if let Lit::Str(string, _) = value {
                        for value in string.split(',') {
                            derives.push(value.trim().into())
                        }
                        break 'attrs;
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

    pub fn derive(&self) -> quote::Ident {
        if self.derives.is_empty() {
            return "".into();
        }
        let mut ident = String::from("#[derive(");
        ident += &self.derives.join(", ");
        ident += ")]";
        return ident.into();
    }

    pub fn derive_no_clone(&self) -> quote::Ident {
        if self.derives.is_empty() {
            return "".into();
        }
        let mut ident = String::from("#[derive(");
        ident += &self.derives.iter()
                              .cloned()
                              .filter(|trai| trai != "Clone")
                              .collect::<Vec<_>>()
                              .join(", ");
        ident += ")]";
        return ident.into();
    }

    pub fn vec_name(&self) -> quote::Ident {
        quote::Ident::from(format!("{}Vec", self.name))
    }

    pub fn slice_name(&self) -> quote::Ident {
        quote::Ident::from(format!("{}Slice", self.name))
    }

    pub fn slice_mut_name(&self) -> quote::Ident {
        quote::Ident::from(format!("{}SliceMut", self.name))
    }

    pub fn ref_name(&self) -> quote::Ident {
        quote::Ident::from(format!("{}Ref", self.name))
    }

    pub fn ref_mut_name(&self) -> quote::Ident {
        quote::Ident::from(format!("{}RefMut", self.name))
    }
}
