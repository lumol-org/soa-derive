use std::convert::TryInto;

use proc_macro2::Span;
use quote::quote;

use syn::{Attribute, Data, DeriveInput, Field, Ident, Path, Visibility, Type};
use syn::{Meta, MetaList, NestedMeta};

/// Representing the struct we are deriving
pub struct Input {
    /// The input struct name
    pub name: Ident,
    /// The list of fields in the struct
    pub fields: Vec<Field>,
    /// The fields that has `#[nested_soa]` attribute
    pub nested_fields: Vec<Field>,
    /// The fields that without `#[nested_soa]` attribute
    pub unnested_fields: Vec<Field>,
    /// Is field marked with `#[nested_soa]`
    pub field_is_nested: Vec<bool>,
    /// The struct overall visibility
    pub visibility: Visibility,
    /// Additional attributes requested with `#[soa_attr(...)]` or
    /// `#[soa_derive()]`
    pub attrs: ExtraAttributes,
}

pub struct ExtraAttributes {
    // did the user explicitly asked us to derive clone?
    pub derive_clone: bool,

    pub vec: Vec<Meta>,
    pub slice: Vec<Meta>,
    pub slice_mut: Vec<Meta>,
    pub ref_: Vec<Meta>,
    pub ref_mut: Vec<Meta>,
    pub ptr: Vec<Meta>,
    pub ptr_mut: Vec<Meta>,
}

impl ExtraAttributes {
    fn new() -> ExtraAttributes {
        ExtraAttributes {
            derive_clone: false,
            vec: Vec::new(),
            slice: Vec::new(),
            slice_mut: Vec::new(),
            ref_: Vec::new(),
            ref_mut: Vec::new(),
            ptr: Vec::new(),
            ptr_mut: Vec::new(),
        }
    }

    /// parse a single `#[soa_attr(...)]`
    fn parse(&mut self, meta: &Meta) {
        match meta {
            Meta::List(MetaList { nested, .. }) => {
                let [soa_type, attr]: [NestedMeta; 2] = nested.into_iter()
                    .cloned()
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap_or_else(|_| panic!(
                        "expected #[soa_attr(\"Types, To, Add, Attribute\", \
                        \"Attribute\")], got #[{}]", quote!(#meta)
                    ));

                let attr = match attr {
                    NestedMeta::Meta(meta) => meta,
                    NestedMeta::Lit(_) => {
                        panic!("expected an attribute, got {}", quote!(attr))
                    }
                };

                match soa_type {
                    NestedMeta::Meta(Meta::Path(path)) => match path.get_ident() {
                        Some(ident) => match ident.to_string().as_str() {
                            "Vec" => {
                                self.vec.push(attr);
                            },
                            "Slice" => {
                                self.slice.push(attr);
                            },
                            "SliceMut" => {
                                self.slice_mut.push(attr);
                            },
                            "Ref" => {
                                self.ref_.push(attr);
                            },
                            "RefMut" => {
                                self.ref_mut.push(attr);
                            },
                            "Ptr" => {
                                self.ptr.push(attr);
                            },
                            "PtrMut" => {
                                self.ptr_mut.push(attr);
                            },
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
            }
            _ => panic!("expected #[soa_attr(...)], got #[{}]", quote!(#meta)),
        }
    }

    /// Add a single trait from `#[soa_derive]`
    fn add_derive(&mut self, path: &Path) {
        let derive_only_vec = |path: &Path| {
            static EXCEPTIONS: &[&str] = &["Clone", "Deserialize", "Serialize"];
            for exception in EXCEPTIONS {
                if path.is_ident(exception) {
                    return true;
                }
            }
            return false;
        };

        let derive = create_derive_meta(path.clone());
        if !derive_only_vec(path) {
            self.slice.push(derive.clone());
            self.slice_mut.push(derive.clone());
            self.ref_.push(derive.clone());
            self.ref_mut.push(derive.clone());
            self.ptr.push(derive.clone());
            self.ptr_mut.push(derive.clone());
        }

        // always add this derive to the Vec struct
        self.vec.push(derive);

        if path.is_ident("Clone") {
            self.derive_clone = true;
        }
    }
}

fn create_derive_meta(path: Path) -> Meta {
    let mut nested = syn::punctuated::Punctuated::new();
    nested.push(NestedMeta::Meta(Meta::Path(path)));

    Meta::List(MetaList {
        path: Path::from(Ident::new("derive", Span::call_site())),
        paren_token: syn::token::Paren {span: Span::call_site()},
        nested: nested
    })
}

fn contains_nested_soa(attrs: &[Attribute]) -> bool {
    for attr in attrs {
        if let Ok(Meta::Path(path)) = attr.parse_meta() {
            if path.is_ident("nested_soa") {
                return true;
            }
        }
    }
    false
}

impl Input {
    pub fn new(input: DeriveInput) -> Input {
        let mut fields = Vec::new();
        let mut nested_fields = Vec::new();
        let mut unnested_fields = Vec::new();
        let mut field_is_nested = Vec::new();
        match input.data {
            Data::Struct(s) => {
                for field in s.fields.iter().cloned() {
                    fields.push(field.clone());
                    field_is_nested.push(contains_nested_soa(&field.attrs));
                    if contains_nested_soa(&field.attrs) {
                        nested_fields.push(field);
                    } else {
                        unnested_fields.push(field);
                    }
                }
            }
            _ => panic!("#[derive(StructOfArray)] only supports struct"),
        };

        if fields.is_empty() {
            panic!("#[derive(StructOfArray)] only supports struct with fields");
        }

        let mut extra_attrs = ExtraAttributes::new();

        for attr in input.attrs {
            if let Ok(meta) = attr.parse_meta() {
                if meta.path().is_ident("soa_derive") {
                    match meta {
                        Meta::List(ref list) => {
                            for element in &list.nested {
                                match element {
                                    NestedMeta::Meta(meta) => {
                                        let path = meta.path();
                                        if path.is_ident("Copy") {
                                            panic!("can not derive Copy for SoA vectors");
                                        }
                                        extra_attrs.add_derive(path);
                                    }
                                    NestedMeta::Lit(_) => {
                                        panic!(
                                            "expected #[soa_derive(Traits, To, Derive)], got #[{}]",
                                            quote!(#meta)
                                        );
                                    }
                                }
                            }
                        }
                        _ => panic!(
                            "expected #[soa_derive(Traits, To, Derive)], got #[{}]",
                            quote!(#meta)
                        ),
                    }
                } else if meta.path().is_ident("soa_attr") {
                    extra_attrs.parse(&meta);
                }
            }
        }

        Input {
            name: input.ident,
            fields: fields,
            visibility: input.vis,
            attrs: extra_attrs,
            nested_fields,
            unnested_fields,
            field_is_nested
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
    pub fn field_tokens_by_nested_soa(&self, 
        unnested_variant: impl Fn(&Ident, &Type) -> proc_macro2::TokenStream, 
        nested_variant: impl Fn(&Ident, &Type) -> proc_macro2::TokenStream) -> proc_macro2::TokenStream 
    {
        let mut seq = quote! {};
        self.fields.iter().zip(self.field_is_nested.iter())
            .for_each(|(field, is_nested)| {
                let field_ident = field.ident.as_ref().unwrap();
                let field_type = &field.ty;

                let variant = if *is_nested {
                    nested_variant(field_ident, field_type)
                } else {
                    unnested_variant(field_ident, field_type)
                };
                seq = quote!(#seq #variant);
            });
        seq
    }
}