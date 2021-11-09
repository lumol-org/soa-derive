use std::convert::TryInto;

use proc_macro2::Span;
use quote::quote;

use syn::{Data, DeriveInput, Field, Ident, Lit, Path, Visibility};
use syn::{Meta, MetaList, MetaNameValue, NestedMeta};

/// Representing the struct we are deriving
pub struct Input {
    /// The input struct name
    pub name: Ident,
    /// The list of fields in the struct
    pub fields: Vec<Field>,
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
    fn add_derive(&mut self, trait_: &str) {
        static EXCEPTIONS: &[&str] = &["Clone", "Deserialize", "Serialize"];

        let derive = create_derive_meta(trait_);
        if !EXCEPTIONS.contains(&trait_) {
            self.slice.push(derive.clone());
            self.slice_mut.push(derive.clone());
            self.ref_.push(derive.clone());
            self.ref_mut.push(derive.clone());
            self.ptr.push(derive.clone());
            self.ptr_mut.push(derive.clone());
        }

        // always add this derive to the Vec struct
        self.vec.push(derive);

        if trait_ == "Clone" {
            self.derive_clone = true;
        }
    }
}

fn create_derive_meta(name: &str) -> Meta {
    let mut nested = syn::punctuated::Punctuated::new();
    nested.push(NestedMeta::Meta(Meta::Path(Path::from(
        Ident::new(name, Span::call_site())
    ))));

    Meta::List(MetaList {
        path: Path::from(Ident::new("derive", Span::call_site())),
        paren_token: syn::token::Paren {span: Span::call_site()},
        nested: nested
    })
}

impl Input {
    pub fn new(input: DeriveInput) -> Input {
        let fields = match input.data {
            Data::Struct(s) => s.fields.iter().cloned().collect::<Vec<_>>(),
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
                        Meta::NameValue(MetaNameValue {
                            lit: Lit::Str(string),
                            ..
                        }) => {
                            for value in string.value().split(',') {
                                let value = value.trim();
                                if value == "Copy" {
                                    panic!("can not derive Copy for SoA vectors");
                                }
                                extra_attrs.add_derive(value);
                            }
                        }
                        _ => panic!(
                            "expected #[soa_derive = \"Traits, To, Derive\"], got #[{}]",
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
