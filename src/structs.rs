use syn::{Body, VariantData, MacroInput, Ident, Field, Visibility, MetaItem};
use syn::Lit;
use quote;

/// Representing the struct we are deriving
pub struct Struct {
    pub name: Ident,
    pub derive: Vec<Ident>,
    pub fields: Vec<Field>,
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

        let mut derive = Vec::new();
        'attrs: for attr in input.attrs {
            if let MetaItem::NameValue(name, value) = attr.value {
                if name.as_ref() == "soa_derive" {
                    if let Lit::Str(string, _) = value {
                        for value in string.split(',') {
                            derive.push(value.trim().into())
                        }
                        break 'attrs;
                    }
                }
            }
        }

        Struct {
            name: input.ident,
            derive: derive,
            fields: fields,
            visibility: input.vis
        }
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
