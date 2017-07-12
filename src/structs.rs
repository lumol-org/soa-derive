use syn::{Body, VariantData, MacroInput, Ident, Field, Visibility, MetaItem};
use syn::Lit;
use quote;

/// Representing the struct we are deriving
pub struct Struct {
    pub name: Ident,
    pub derive: Ident,
    pub derive_no_clone: Ident,
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

        let mut derive = String::from("#[derive(");
        derive += &derives.join(", ");
        derive += ")]";

        let mut derive_no_clone = String::from("#[derive(");
        derive_no_clone += &derives.iter()
                                   .cloned()
                                   .filter(|trai| trai != "Clone")
                                   .collect::<Vec<_>>()
                                   .join(", ");
        derive_no_clone += ")]";

        Struct {
            name: input.ident,
            derive: derive.into(),
            derive_no_clone: derive_no_clone.into(),
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
