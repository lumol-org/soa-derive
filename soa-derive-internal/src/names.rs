use proc_macro2::Span;
use quote::ToTokens;
use syn::Ident;

/// Get the ident for the `Vec` type associated with `name`
pub fn vec_name(name: impl ToTokens) -> Ident {
    Ident::new(&format!("{}Vec", name.to_token_stream()), Span::call_site())
}

/// Get the ident for the slice type associated with `name`
pub fn slice_name(name: impl ToTokens) -> Ident {
    Ident::new(&format!("{}Slice", name.to_token_stream()), Span::call_site())
}

/// Get the ident for the mutable slice type associated with `name`
pub fn slice_mut_name(name: impl ToTokens) -> Ident {
    Ident::new(&format!("{}SliceMut", name.to_token_stream()), Span::call_site())
}

/// Get the ident for the reference type associated with `name`
pub fn ref_name(name: impl ToTokens) -> Ident {
    Ident::new(&format!("{}Ref", name.to_token_stream()), Span::call_site())
}

/// Get the ident for the mutable reference type associated with `name`
pub fn ref_mut_name(name: impl ToTokens) -> Ident {
    Ident::new(&format!("{}RefMut", name.to_token_stream()), Span::call_site())
}

/// Get the ident for the pointer type associated with `name`
pub fn ptr_name(name: impl ToTokens) -> Ident {
    Ident::new(&format!("{}Ptr", name.to_token_stream()), Span::call_site())
}

/// Get the ident for the mutable pointer type associated with `name`
pub fn ptr_mut_name(name: impl ToTokens) -> Ident {
    Ident::new(&format!("{}PtrMut", name.to_token_stream()), Span::call_site())
}
