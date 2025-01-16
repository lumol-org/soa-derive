use proc_macro2::{Span, TokenStream};
use quote::quote;

use syn::punctuated::Punctuated;
use syn::{Attribute, Data, DeriveInput, Field, Path, Visibility, Token};
use syn::{Meta, MetaList};

/// Representing the struct we are deriving
pub struct Input {
    /// The input struct name
    pub name: syn::Ident,
    /// The list of fields in the struct
    pub fields: Vec<Field>,
    /// Is field marked with `#[nested_soa]`
    pub field_is_nested: Vec<bool>,
    /// The struct overall visibility
    pub visibility: Visibility,
    /// Additional attributes requested with `#[soa_attr(...)]` or
    /// `#[soa_derive()]`
    pub attrs: ExtraAttributes,
    #[allow(unused)]
    /// Whether or not to generate extra trait implementations that make the SoA types usable
    /// in a generic context enabled by the `generic_traits` feature.
    pub generate_traits: bool,
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

    /// Add a single trait from `#[soa_derive]`
    fn add_derive(&mut self, ident: &proc_macro2::Ident) {
        let derive_only_vec = |ident| {
            static EXCEPTIONS: &[&str] = &["Clone", "Deserialize", "Serialize"];
            for exception in EXCEPTIONS {
                if ident == exception {
                    return true;
                }
            }
            return false;
        };

        let derive = Meta::List(MetaList {
            path: Path::from(syn::Ident::new("derive", Span::call_site())),
            delimiter: syn::MacroDelimiter::Paren(syn::token::Paren(Span::call_site())),
            tokens: quote!{ #ident },
        });

        if !derive_only_vec(ident) {
            self.slice.push(derive.clone());
            self.slice_mut.push(derive.clone());
            self.ref_.push(derive.clone());
            self.ref_mut.push(derive.clone());
            self.ptr.push(derive.clone());
            self.ptr_mut.push(derive.clone());
        }

        // always add this derive to the Vec struct
        self.vec.push(derive);

        if ident == "Clone" {
            self.derive_clone = true;
        }
    }
}

fn contains_nested_soa(attrs: &[Attribute]) -> bool {
    for attr in attrs {
        if attr.path().is_ident("nested_soa") {
            return true;
        }
    }
    return false;
}

impl Input {
    pub fn new(input: DeriveInput) -> Input {
        let mut fields = Vec::new();
        let mut field_is_nested = Vec::new();
        match input.data {
            Data::Struct(s) => {
                for field in s.fields.iter().cloned() {
                    fields.push(field.clone());
                    field_is_nested.push(contains_nested_soa(&field.attrs));
                }
            }
            _ => panic!("#[derive(StructOfArray)] only supports struct"),
        };

        assert!(!fields.is_empty(), "#[derive(StructOfArray)] only supports struct with fields");

        let mut extra_attrs = ExtraAttributes::new();
        let mut generate_traits: bool = false;

        for attr in input.attrs {
            if attr.path().is_ident("soa_derive") {
                attr.parse_nested_meta(|meta| {
                    match meta.path.get_ident() {
                        Some(ident) => {
                            assert!(ident != "Copy", "can not derive Copy for SoA vectors");
                            if ident != "Default" {
                                // ignore as Default is already derived for SoA vectors, slices and mut slices
                                extra_attrs.add_derive(ident);
                            }
                        }
                        None => {
                            panic!(
                                "expected #[soa_derive(Traits, To, Derive)], got #[{}]",
                                quote!(attr)
                            );
                        }
                    }
                    Ok(())
                }).expect("failed to parse soa_derive");
            }

            if attr.path().is_ident("soa_attr") {
                let nested = attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                    .expect("expected attribute like #[soa_attr(<Type>, <attr>)]");
                assert!(nested.len() == 2, "expected attribute like #[soa_attr(<Type>, <attr>)]");

                let soa_type = nested.first().expect("should have 2 elements");
                let attr = nested.last().expect("should have 2 elements").clone();

                match soa_type.path().get_ident() {
                    Some(ident) => {
                        if ident == "Vec" {
                            extra_attrs.vec.push(attr);
                        } else if ident == "Slice" {
                            extra_attrs.slice.push(attr);
                        } else if ident == "SliceMut" {
                            extra_attrs.slice_mut.push(attr);
                        } else if ident == "Ref" {
                            extra_attrs.ref_.push(attr);
                        } else if ident == "RefMut" {
                            extra_attrs.ref_mut.push(attr);
                        } else if ident == "Ptr" {
                            extra_attrs.ptr.push(attr);
                        } else if ident == "PtrMut" {
                            extra_attrs.ptr_mut.push(attr);
                        } else {
                            panic!("expected one of the SoA type, got {}", quote!(#soa_type));
                        }
                    }
                    None => panic!("expected one of the SoA type, got {}", quote!(#soa_type))
                }
            }

            if attr.path().is_ident("generate_traits") {
                generate_traits = true;
            }
        }

        Input {
            name: input.ident,
            fields: fields,
            visibility: input.vis,
            attrs: extra_attrs,
            field_is_nested,
            generate_traits: generate_traits,
        }
    }

    /// Map over all fields in the struct, calling the first function if the
    /// field is a nested struct of array, the second function otherwise
    pub(crate) fn map_fields_nested_or<'a, A, B>(&'a self, nested: A, not_nested: B) -> impl TokenStreamIterator + 'a
        where A: Fn(&syn::Ident, &syn::Type) -> TokenStream + 'a,
              B: Fn(&syn::Ident, &syn::Type) -> TokenStream + 'a,
    {
        self.fields.iter().zip(self.field_is_nested.iter()).map(move |(field, &is_nested)| {
            if is_nested {
                nested(field.ident.as_ref().expect("missing ident"), &field.ty)
            } else {
                not_nested(field.ident.as_ref().expect("missing ident"), &field.ty)
            }
        })
    }
}

pub(crate) trait TokenStreamIterator: Iterator<Item = proc_macro2::TokenStream> {
    fn concat_by(self, f: impl Fn(proc_macro2::TokenStream, proc_macro2::TokenStream) -> proc_macro2::TokenStream) -> proc_macro2::TokenStream;
}

impl<T: Iterator<Item = proc_macro2::TokenStream>> TokenStreamIterator for T {
    fn concat_by(mut self, f: impl Fn(proc_macro2::TokenStream, proc_macro2::TokenStream) -> proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        match self.next() {
            Some(first) => {
                self.fold(first, |current, next| {
                    f(current, next)
                })
            },
            None => quote!{},
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concat_by() {
        let token_streams = vec![quote!{a}, quote!{b}, quote!{c}];
        assert_eq!(token_streams.into_iter().concat_by(|current, next| {
            quote!{(#current, #next)}
        }).to_string(), "((a , b) , c)");
    }
}
