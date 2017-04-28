use syn::{Body, VariantData, MacroInput, Ident, Field, Visibility};
use quote;

/// Representing the struct we are deriving
pub struct Struct {
    pub name: Ident,
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

        Struct {
            name: input.ident,
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
