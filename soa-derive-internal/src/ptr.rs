use proc_macro2::TokenStream;
use quote::quote;

use crate::input::Input;
use crate::names;

pub fn derive(input: &Input) -> TokenStream {
    let name = &input.name;
    let visibility = &input.visibility;
    let attrs = &input.attrs.ptr;
    let mut_attrs = &input.attrs.ptr_mut;
    let vec_name = names::vec_name(&input.name);
    let ptr_name = names::ptr_name(&input.name);
    let ptr_mut_name = names::ptr_mut_name(&input.name);
    let ref_name = names::ref_name(&input.name);
    let ref_mut_name = names::ref_mut_name(&input.name);

    let doc_url = format!("[`{0}`](struct.{0}.html)", name);
    let vec_doc_url = format!("[`{0}`](struct.{0}.html)", vec_name);
    let ptr_doc_url = format!("[`{0}`](struct.{0}.html)", ptr_name);
    let ptr_mut_doc_url = format!("[`{0}`](struct.{0}.html)", ptr_mut_name);
    let ref_doc_url = format!("[`{0}`](struct.{0}.html)", ref_name);
    let ref_mut_doc_url = format!("[`{0}`](struct.{0}.html)", ref_mut_name);

    let fields_names = &input.fields.iter()
        .map(|field| field.ident.clone().unwrap())
        .collect::<Vec<_>>();

    let ptr_fields_types = input.map_fields_nested_or(
        |_, field_type| {
            let field_ptr_type = names::ptr_name(field_type);
            quote! { #field_ptr_type }
        },
        |_, field_type| quote! { *const #field_type },
    ).collect::<Vec<_>>();

    let ptr_mut_fields_types = input.map_fields_nested_or(
        |_, field_type| {
            let field_ptr_type = names::ptr_mut_name(field_type);
            quote! { #field_ptr_type }
        },
        |_, field_type| quote! { *mut #field_type },
    ).collect::<Vec<_>>();

    let as_ptr = input.map_fields_nested_or(
        |ident, _| quote! { self.#ident.as_ptr() },
        |ident, _| quote! { self.#ident as *const _ },
    ).collect::<Vec<_>>();

    let as_mut_ptr = input.map_fields_nested_or(
        |ident, _| quote! { self.#ident.as_mut_ptr() },
        |ident, _| quote! { self.#ident as *mut _ },
    ).collect::<Vec<_>>();

    quote! {
        /// An analog of a pointer to
        #[doc = #doc_url]
        /// with struct of array layout.
        #(#[#attrs])*
        #[derive(Copy, Clone)]
        #visibility struct #ptr_name {
            #(
                /// pointer to the `
                #[doc = stringify!(#fields_names)]
                ///` field of a single
                #[doc = #doc_url]
                /// inside a
                #[doc = #vec_doc_url]
                pub #fields_names: #ptr_fields_types,
            )*
        }

        /// An analog of a mutable pointer to
        #[doc = #doc_url]
        /// with struct of array layout.
        #(#[#mut_attrs])*
        #[derive(Copy, Clone)]
        #visibility struct #ptr_mut_name {
            #(
                /// pointer to the `
                #[doc = stringify!(#fields_names)]
                ///` field of a single
                #[doc = #doc_url]
                /// inside a
                #[doc = #vec_doc_url]
                pub #fields_names: #ptr_mut_fields_types,
            )*
        }

        #[allow(dead_code)]
        impl #ptr_name {
            /// Convert a
            #[doc = #ptr_doc_url]
            /// to a
            #[doc = #ptr_mut_doc_url]
            /// ; *i.e.* do a `*const T as *mut T` transformation.
            #visibility fn as_mut_ptr(&self) -> #ptr_mut_name {
                #ptr_mut_name {
                    #( #fields_names: #as_mut_ptr, )*
                }
            }

            /// Similar to [`*const T::is_null()`](https://doc.rust-lang.org/std/primitive.pointer.html#method.is_null).
            pub fn is_null(self) -> bool {
                false #( || self.#fields_names.is_null())*
            }

            /// Similar to [`*const T::as_ref()`](https://doc.rust-lang.org/std/primitive.pointer.html#method.as_ref),
            /// with the same safety caveats.
            pub unsafe fn as_ref<'a>(self) -> Option<#ref_name<'a>> {
                if self.is_null() {
                    None
                } else {
                    Some(#ref_name {
                        #(#fields_names: self.#fields_names.as_ref().expect("should not be null"), )*
                    })
                }
            }

            /// Similar to [`*const T::offset()`](https://doc.rust-lang.org/std/primitive.pointer.html#method.offset),
            /// with the same safety caveats.
            pub unsafe fn offset(self, count: isize) -> #ptr_name {
                #ptr_name {
                    #(#fields_names: self.#fields_names.offset(count), )*
                }
            }

            /// Similar to [`*const T::offset()`](https://doc.rust-lang.org/std/primitive.pointer.html#method.offset).
            pub fn wrapping_offset(self, count: isize) -> #ptr_name {
                #ptr_name {
                    #(#fields_names: self.#fields_names.wrapping_offset(count), )*
                }
            }

            /// Similar to [`*const T::add()`](https://doc.rust-lang.org/std/primitive.pointer.html#method.add),
            /// with the same safety caveats.
            pub unsafe fn add(self, count: usize) -> #ptr_name {
                #ptr_name {
                    #(#fields_names: self.#fields_names.add(count), )*
                }
            }

            /// Similar to [`*const T::sub()`](https://doc.rust-lang.org/std/primitive.pointer.html#method.sub),
            /// with the same safety caveats.
            pub unsafe fn sub(self, count: usize) -> #ptr_name {
                #ptr_name {
                    #(#fields_names: self.#fields_names.sub(count), )*
                }
            }

            /// Similar to [`*const T::wrapping_add()`](https://doc.rust-lang.org/std/primitive.pointer.html#method.wrapping_add).
            pub fn wrapping_add(self, count: usize) -> #ptr_name {
                #ptr_name {
                    #(#fields_names: self.#fields_names.wrapping_add(count), )*
                }
            }

            /// Similar to [`*const T::wrapping_sub()`](https://doc.rust-lang.org/std/primitive.pointer.html#method.wrapping_sub).
            pub fn wrapping_sub(self, count: usize) -> #ptr_name {
                #ptr_name {
                    #(#fields_names: self.#fields_names.wrapping_sub(count), )*
                }
            }

            /// Similar to [`*const T::read()`](https://doc.rust-lang.org/std/primitive.pointer.html#method.read),
            /// with the same safety caveats.
            pub unsafe fn read(self) -> #name {
                #name {
                    #(#fields_names: self.#fields_names.read(), )*
                }
            }

            /// Similar to [`*const T::read_volatile()`](https://doc.rust-lang.org/std/primitive.pointer.html#method.read_volatile),
            /// with the same safety caveats.
            pub unsafe fn read_volatile(self) -> #name {
                #name {
                    #(#fields_names: self.#fields_names.read_volatile(), )*
                }
            }

            /// Similar to [`*const T::read_unaligned()`](https://doc.rust-lang.org/std/primitive.pointer.html#method.read_unaligned),
            /// with the same safety caveats.
            pub unsafe fn read_unaligned(self) -> #name {
                #name {
                    #(#fields_names: self.#fields_names.read_unaligned(), )*
                }
            }
        }

        impl ::soa_derive::SoAPointers for #name {
            type Ptr = #ptr_name;
            type MutPtr = #ptr_mut_name;
        }

        #[allow(dead_code)]
        #[allow(clippy::forget_non_drop)]
        impl #ptr_mut_name {
            /// Convert a
            #[doc = #ptr_mut_doc_url]
            /// to a
            #[doc = #ptr_doc_url]
            /// ; *i.e.* do a `*mut T as *const T` transformation
            #visibility fn as_ptr(&self) -> #ptr_name {
                #ptr_name {
                    #( #fields_names: #as_ptr, )*
                }
            }

            /// Similar to [`*mut T::is_null()`](https://doc.rust-lang.org/std/primitive.pointer.html#method.is_null).
            pub fn is_null(self) -> bool {
                false #( || self.#fields_names.is_null())*
            }

            /// Similar to [`*mut T::as_ref()`](https://doc.rust-lang.org/std/primitive.pointer.html#method.as_ref),
            /// with the same safety caveats.
            pub unsafe fn as_ref<'a>(self) -> Option<#ref_name<'a>> {
                if self.is_null() {
                    None
                } else {
                    Some(#ref_name {
                        #(#fields_names: self.#fields_names.as_ref().expect("should not be null"), )*
                    })
                }
            }

            /// Similar to [`*mut T::as_mut()`](https://doc.rust-lang.org/std/primitive.pointer.html#method.as_mut),
            /// with the same safety caveats.
            pub unsafe fn as_mut<'a>(self) -> Option<#ref_mut_name<'a>> {
                if self.is_null() {
                    None
                } else {
                    Some(#ref_mut_name {
                        #(#fields_names: self.#fields_names.as_mut().expect("should not be null"), )*
                    })
                }
            }

            /// Similar to [`*mut T::offset()`](https://doc.rust-lang.org/std/primitive.pointer.html#method.offset),
            /// with the same safety caveats.
            pub unsafe fn offset(self, count: isize) -> #ptr_mut_name {
                #ptr_mut_name {
                    #(#fields_names: self.#fields_names.offset(count), )*
                }
            }

            /// Similar to [`*mut T::wrapping_offset()`](https://doc.rust-lang.org/std/primitive.pointer.html#method.wrapping_offset)
            pub fn wrapping_offset(self, count: isize) -> #ptr_mut_name {
                #ptr_mut_name {
                    #(#fields_names: self.#fields_names.wrapping_offset(count), )*
                }
            }

            /// Similar to [`*mut T::add()`](https://doc.rust-lang.org/std/primitive.pointer.html#method.add),
            /// with the same safety caveats.
            pub unsafe fn add(self, count: usize) -> #ptr_mut_name {
                #ptr_mut_name {
                    #(#fields_names: self.#fields_names.add(count), )*
                }
            }

            /// Similar to [`*mut T::sub()`](https://doc.rust-lang.org/std/primitive.pointer.html#method.sub),
            /// with the same safety caveats.
            pub unsafe fn sub(self, count: usize) -> #ptr_mut_name {
                #ptr_mut_name {
                    #(#fields_names: self.#fields_names.sub(count), )*
                }
            }

            /// Similar to [`*mut T::wrapping_add()`](https://doc.rust-lang.org/std/primitive.pointer.html#method.wrapping_add),
            /// with the same safety caveats.
            pub fn wrapping_add(self, count: usize) -> #ptr_mut_name {
                #ptr_mut_name {
                    #(#fields_names: self.#fields_names.wrapping_add(count), )*
                }
            }

            /// Similar to [`*mut T::wrapping_sub()`](https://doc.rust-lang.org/std/primitive.pointer.html#method.wrapping_sub),
            /// with the same safety caveats.
            pub fn wrapping_sub(self, count: usize) -> #ptr_mut_name {
                #ptr_mut_name {
                    #(#fields_names: self.#fields_names.wrapping_sub(count), )*
                }
            }

            /// Similar to [`*mut T::read()`](https://doc.rust-lang.org/std/primitive.pointer.html#method.read),
            /// with the same safety caveats.
            pub unsafe fn read(self) -> #name {
                #name {
                    #(#fields_names: self.#fields_names.read(), )*
                }
            }

            /// Similar to [`*mut T::read_volatile()`](https://doc.rust-lang.org/std/primitive.pointer.html#method.read_volatile),
            /// with the same safety caveats.
            pub unsafe fn read_volatile(self) -> #name {
                #name {
                    #(#fields_names: self.#fields_names.read_volatile(), )*
                }
            }

            /// Similar to [`*mut T::read_unaligned()`](https://doc.rust-lang.org/std/primitive.pointer.html#method.read_unaligned),
            /// with the same safety caveats.
            pub unsafe fn read_unaligned(self) -> #name {
                #name {
                    #(#fields_names: self.#fields_names.read_unaligned(), )*
                }
            }

            /// Similar to [`*mut T::write()`](https://doc.rust-lang.org/std/primitive.pointer.html#method.write),
            /// with the same safety caveats.
            #[allow(clippy::forget_non_drop)]
            pub unsafe fn write(self, val: #name) {
                unsafe {
                    #(self.#fields_names.write(::std::ptr::read(&val.#fields_names));)*
                }
                // if val implements Drop, we don't want to run it here, only
                // when the vec itself will be dropped
                ::std::mem::forget(val);
            }

            /// Similar to [`*mut T::write_volatile()`](https://doc.rust-lang.org/std/primitive.pointer.html#method.write_volatile),
            /// with the same safety caveats.
            #[allow(clippy::forget_non_drop)]
            pub unsafe fn write_volatile(self, val: #name) {
                unsafe {
                    #(self.#fields_names.write_volatile(::std::ptr::read(&val.#fields_names));)*
                }
                // if val implements Drop, we don't want to run it here, only
                // when the vec itself will be dropped
                ::std::mem::forget(val);
            }

            /// Similar to [`*mut T::write_unaligned()`](https://doc.rust-lang.org/std/primitive.pointer.html#method.write_unaligned),
            /// with the same safety caveats.
            #[allow(clippy::forget_non_drop)]
            pub unsafe fn write_unaligned(self, val: #name) {
                unsafe {
                    #(self.#fields_names.write_unaligned(::std::ptr::read(&val.#fields_names));)*
                }
                // if val implements Drop, we don't want to run it here, only
                // when the vec itself will be dropped
                ::std::mem::forget(val);
            }
        }

        #[allow(dead_code)]
        impl<'a> #ref_name<'a> {
            /// Convert a
            #[doc = #ref_doc_url]
            /// to a
            #[doc = #ptr_doc_url]
            /// ; *i.e.* do a `&T as *const T` transformation
            #visibility fn as_ptr(&self) -> #ptr_name {
                #ptr_name {
                    #( #fields_names: #as_ptr, )*
                }
            }
        }

        #[allow(dead_code)]
        impl<'a> #ref_mut_name<'a> {
            /// Convert a
            #[doc = #ref_mut_doc_url]
            /// to a
            #[doc = #ptr_doc_url]
            /// ; *i.e.* do a `&mut T as *const T` transformation
            #visibility fn as_ptr(&self) -> #ptr_name {
                #ptr_name {
                    #( #fields_names: #as_ptr, )*
                }
            }

            /// Convert a
            #[doc = #ref_mut_doc_url]
            /// to a
            #[doc = #ptr_mut_doc_url]
            /// ; *i.e.* do a `&mut T as *mut T` transformation
            #visibility fn as_mut_ptr(&mut self) -> #ptr_mut_name {
                #ptr_mut_name {
                    #( #fields_names: #as_mut_ptr, )*
                }
            }
        }
    }
}
