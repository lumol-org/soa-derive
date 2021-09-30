use proc_macro2::TokenStream;
use quote::quote;

use crate::input::Input;

pub fn derive(input: &Input) -> TokenStream {
    let name = &input.name;
    let visibility = &input.visibility;
    let other_derive = &input.derive_with_exceptions();
    let attrs = &input.ptr_attrs;
    let vec_name = &input.vec_name();
    let ptr_name = &input.ptr_name();
    let ptr_mut_name = &input.ptr_mut_name();
    let ref_name = &input.ref_name();
    let ref_mut_name = &input.ref_mut_name();

    let doc_url = format!("[`{0}`](struct.{0}.html)", name);
    let ptr_doc_url = format!("[`{0}`](struct.{0}.html)", ptr_name);
    let ptr_mut_doc_url = format!("[`{0}`](struct.{0}.html)", ptr_mut_name);
    let ref_doc_url = format!("[`{0}`](struct.{0}.html)", ref_name);
    let ref_mut_doc_url = format!("[`{0}`](struct.{0}.html)", ref_mut_name);

    let fields_names = input.fields.iter()
                                   .map(|field| field.ident.clone().unwrap())
                                   .collect::<Vec<_>>();
    let fields_names_1 = &fields_names;
    let fields_names_2 = &fields_names;

    let fields_types = &input.fields.iter()
                                    .map(|field| &field.ty)
                                    .collect::<Vec<_>>();

    let fields_doc = fields_names.iter()
        .map(|field| format!("A pointer to a `{0}` from a [`{1}`](struct.{1}.html)", field, vec_name))
        .collect::<Vec<_>>();

    let fields_mut_doc = fields_names.iter()
        .map(|field| format!("A mutable pointer to a `{0}` from a [`{1}`](struct.{1}.html)", field, vec_name))
        .collect::<Vec<_>>();

    quote! {
        /// An analog of a pointer to
        #[doc = #doc_url]
        /// with struct of array layout.
        #other_derive
        #(#[#attrs])*
        #[derive(Copy, Clone)]
        #visibility struct #ptr_name {
            #(
                #[doc = #fields_doc]
                pub #fields_names_1: *const #fields_types,
            )*
        }

        /// An analog of a mutable pointer to
        #[doc = #doc_url]
        /// with struct of array layout.
        #other_derive
        #[derive(Copy, Clone)]
        #visibility struct #ptr_mut_name {
            #(
                #[doc = #fields_mut_doc]
                pub #fields_names_1: *mut #fields_types,
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
                    #(#fields_names_1: self.#fields_names_2 as *mut _, )*
                }
            }

            /// Similar to [`*const T::is_null()`](https://doc.rust-lang.org/std/primitive.pointer.html#method.is_null).
            pub fn is_null(self) -> bool {
                false #( || self.#fields_names_1.is_null())*
            }

            /// Similar to [`*const T::as_ref()`](https://doc.rust-lang.org/std/primitive.pointer.html#method.as_ref),
            /// with the same safety caveats.
            pub unsafe fn as_ref<'a>(self) -> Option<#ref_name<'a>> {
                if self.is_null() {
                    None
                } else {
                    Some(#ref_name {
                        #(#fields_names_1: self.#fields_names_2.as_ref().expect("should not be null"), )*
                    })
                }
            }

            /// Similar to [`*const T::offset()`](https://doc.rust-lang.org/std/primitive.pointer.html#method.offset),
            /// with the same safety caveats.
            pub unsafe fn offset(self, count: isize) -> #ptr_name {
                #ptr_name {
                    #(#fields_names_1: self.#fields_names_2.offset(count), )*
                }
            }

            /// Similar to [`*const T::offset()`](https://doc.rust-lang.org/std/primitive.pointer.html#method.offset).
            pub fn wrapping_offset(self, count: isize) -> #ptr_name {
                #ptr_name {
                    #(#fields_names_1: self.#fields_names_2.wrapping_offset(count), )*
                }
            }

            /// Similar to [`*const T::add()`](https://doc.rust-lang.org/std/primitive.pointer.html#method.add),
            /// with the same safety caveats.
            pub unsafe fn add(self, count: usize) -> #ptr_name {
                #ptr_name {
                    #(#fields_names_1: self.#fields_names_2.add(count), )*
                }
            }

            /// Similar to [`*const T::sub()`](https://doc.rust-lang.org/std/primitive.pointer.html#method.sub),
            /// with the same safety caveats.
            pub unsafe fn sub(self, count: usize) -> #ptr_name {
                #ptr_name {
                    #(#fields_names_1: self.#fields_names_2.sub(count), )*
                }
            }

            /// Similar to [`*const T::wrapping_add()`](https://doc.rust-lang.org/std/primitive.pointer.html#method.wrapping_add).
            pub fn wrapping_add(self, count: usize) -> #ptr_name {
                #ptr_name {
                    #(#fields_names_1: self.#fields_names_2.wrapping_add(count), )*
                }
            }

            /// Similar to [`*const T::wrapping_sub()`](https://doc.rust-lang.org/std/primitive.pointer.html#method.wrapping_sub).
            pub fn wrapping_sub(self, count: usize) -> #ptr_name {
                #ptr_name {
                    #(#fields_names_1: self.#fields_names_2.wrapping_sub(count), )*
                }
            }

            /// Similar to [`*const T::read()`](https://doc.rust-lang.org/std/primitive.pointer.html#method.read),
            /// with the same safety caveats.
            pub unsafe fn read(self) -> #name {
                #name {
                    #(#fields_names_1: self.#fields_names_2.read(), )*
                }
            }

            /// Similar to [`*const T::read_volatile()`](https://doc.rust-lang.org/std/primitive.pointer.html#method.read_volatile),
            /// with the same safety caveats.
            pub unsafe fn read_volatile(self) -> #name {
                #name {
                    #(#fields_names_1: self.#fields_names_2.read_volatile(), )*
                }
            }

            /// Similar to [`*const T::read_unaligned()`](https://doc.rust-lang.org/std/primitive.pointer.html#method.read_unaligned),
            /// with the same safety caveats.
            pub unsafe fn read_unaligned(self) -> #name {
                #name {
                    #(#fields_names_1: self.#fields_names_2.read_unaligned(), )*
                }
            }
        }

        #[allow(dead_code)]
        impl #ptr_mut_name {
            /// Convert a
            #[doc = #ptr_mut_doc_url]
            /// to a
            #[doc = #ptr_doc_url]
            /// ; *i.e.* do a `*mut T as *const T` transformation
            #visibility fn as_ptr(&self) -> #ptr_name {
                #ptr_name {
                    #(#fields_names_1: self.#fields_names_2, )*
                }
            }

            /// Similar to [`*mut T::is_null()`](https://doc.rust-lang.org/std/primitive.pointer.html#method.is_null).
            pub fn is_null(self) -> bool {
                false #( || self.#fields_names_1.is_null())*
            }

            /// Similar to [`*mut T::as_ref()`](https://doc.rust-lang.org/std/primitive.pointer.html#method.as_ref),
            /// with the same safety caveats.
            pub unsafe fn as_ref<'a>(self) -> Option<#ref_name<'a>> {
                if self.is_null() {
                    None
                } else {
                    Some(#ref_name {
                        #(#fields_names_1: self.#fields_names_2.as_ref().expect("should not be null"), )*
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
                        #(#fields_names_1: self.#fields_names_2.as_mut().expect("should not be null"), )*
                    })
                }
            }

            /// Similar to [`*mut T::offset()`](https://doc.rust-lang.org/std/primitive.pointer.html#method.offset),
            /// with the same safety caveats.
            pub unsafe fn offset(self, count: isize) -> #ptr_mut_name {
                #ptr_mut_name {
                    #(#fields_names_1: self.#fields_names_2.offset(count), )*
                }
            }

            /// Similar to [`*mut T::wrapping_offset()`](https://doc.rust-lang.org/std/primitive.pointer.html#method.wrapping_offset)
            pub fn wrapping_offset(self, count: isize) -> #ptr_mut_name {
                #ptr_mut_name {
                    #(#fields_names_1: self.#fields_names_2.wrapping_offset(count), )*
                }
            }

            /// Similar to [`*mut T::add()`](https://doc.rust-lang.org/std/primitive.pointer.html#method.add),
            /// with the same safety caveats.
            pub unsafe fn add(self, count: usize) -> #ptr_mut_name {
                #ptr_mut_name {
                    #(#fields_names_1: self.#fields_names_2.add(count), )*
                }
            }

            /// Similar to [`*mut T::sub()`](https://doc.rust-lang.org/std/primitive.pointer.html#method.sub),
            /// with the same safety caveats.
            pub unsafe fn sub(self, count: usize) -> #ptr_mut_name {
                #ptr_mut_name {
                    #(#fields_names_1: self.#fields_names_2.sub(count), )*
                }
            }

            /// Similar to [`*mut T::wrapping_add()`](https://doc.rust-lang.org/std/primitive.pointer.html#method.wrapping_add),
            /// with the same safety caveats.
            pub fn wrapping_add(self, count: usize) -> #ptr_mut_name {
                #ptr_mut_name {
                    #(#fields_names_1: self.#fields_names_2.wrapping_add(count), )*
                }
            }

            /// Similar to [`*mut T::wrapping_sub()`](https://doc.rust-lang.org/std/primitive.pointer.html#method.wrapping_sub),
            /// with the same safety caveats.
            pub fn wrapping_sub(self, count: usize) -> #ptr_mut_name {
                #ptr_mut_name {
                    #(#fields_names_1: self.#fields_names_2.wrapping_sub(count), )*
                }
            }

            /// Similar to [`*mut T::read()`](https://doc.rust-lang.org/std/primitive.pointer.html#method.read),
            /// with the same safety caveats.
            pub unsafe fn read(self) -> #name {
                #name {
                    #(#fields_names_1: self.#fields_names_2.read(), )*
                }
            }

            /// Similar to [`*mut T::read_volatile()`](https://doc.rust-lang.org/std/primitive.pointer.html#method.read_volatile),
            /// with the same safety caveats.
            pub unsafe fn read_volatile(self) -> #name {
                #name {
                    #(#fields_names_1: self.#fields_names_2.read_volatile(), )*
                }
            }

            /// Similar to [`*mut T::read_unaligned()`](https://doc.rust-lang.org/std/primitive.pointer.html#method.read_unaligned),
            /// with the same safety caveats.
            pub unsafe fn read_unaligned(self) -> #name {
                #name {
                    #(#fields_names_1: self.#fields_names_2.read_unaligned(), )*
                }
            }

            /// Similar to [`*mut T::write()`](https://doc.rust-lang.org/std/primitive.pointer.html#method.write),
            /// with the same safety caveats.
            pub unsafe fn write(self, val: #name) {
                #(self.#fields_names_1.write(val.#fields_names_2); )*
            }

            /// Similar to [`*mut T::write_volatile()`](https://doc.rust-lang.org/std/primitive.pointer.html#method.write_volatile),
            /// with the same safety caveats.
            pub unsafe fn write_volatile(self, val: #name) {
                #(self.#fields_names_1.write_volatile(val.#fields_names_2); )*
            }

            /// Similar to [`*mut T::write_unaligned()`](https://doc.rust-lang.org/std/primitive.pointer.html#method.write_unaligned),
            /// with the same safety caveats.
            pub unsafe fn write_unaligned(self, val: #name) {
                #(self.#fields_names_1.write_unaligned(val.#fields_names_2); )*
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
                    #(#fields_names_1: self.#fields_names_2, )*
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
                    #(#fields_names_1: self.#fields_names_2, )*
                }
            }

            /// Convert a
            #[doc = #ref_mut_doc_url]
            /// to a
            #[doc = #ptr_mut_doc_url]
            /// ; *i.e.* do a `&mut T as *mut T` transformation
            #visibility fn as_mut_ptr(&mut self) -> #ptr_mut_name {
                #ptr_mut_name {
                    #(#fields_names_1: self.#fields_names_2, )*
                }
            }
        }
    }
}
