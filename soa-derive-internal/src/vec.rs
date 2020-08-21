use proc_macro2::{Span, TokenStream};
use syn::Ident;
use quote::TokenStreamExt;
use quote::quote;

use crate::input::Input;

pub fn derive(input: &Input) -> TokenStream {
    let name = &input.name;
    let vec_name_str = format!("Vec<{}>", name);
    let other_derive = &input.derive();
    let visibility = &input.visibility;
    let vec_name = &input.vec_name();
    let vec_fields_name = &input.vec_fields_name();
    let slice_name = &input.slice_name();
    let slice_mut_name = &input.slice_mut_name();
    let ptr_name = &input.ptr_name();
    let ptr_mut_name = &input.ptr_mut_name();

    let fields_names = input.fields.iter()
                                   .map(|field| field.ident.clone().unwrap())
                                   .collect::<Vec<_>>();
    let fields_names_1 = &fields_names;
    let fields_names_2 = &fields_names;

    let fields_types = &input.fields.iter()
                                    .map(|field| &field.ty)
                                    .collect::<Vec<_>>();

    let imports = quote!{
        extern crate alloc;

        use alloc::raw_vec::RawVec;
        use std::ptr;
        use std::slice;
    };

    let mut generated = quote! {
        #imports

        /// Contains the RawVecs of the SoA
        #other_derive
        struct #vec_fields_name {
            #(
                pub #fields_names_1: RawVec<#fields_types>,
            )*
        }

        /// An analog to `
        #[doc = #vec_name_str]
        /// ` with Struct of Array (SoA) layout
        #[allow(dead_code)]
        #other_derive
        #visibility struct #vec_name {
            data: #vec_fields_name,
            len: usize,
        }

        #[allow(dead_code)]
        impl #vec_name {
            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::new()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.new)
            pub fn new() -> #vec_name {
                #vec_name {
                    data: #vec_fields_name{
                        #(#fields_names_1: RawVec::new(),)*
                    },
                    len: 0,
                }
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::with_capacity()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.with_capacity),
            /// initializing all fields with the given `capacity`.
            pub fn with_capacity(capacity: usize) -> #vec_name {
                #vec_name {
                    data: #vec_fields_name {
                        #(#fields_names_1 : RawVec::with_capacity(capacity),)*
                    },
                    len: 0,
                }
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::capacity()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.capacity),
            /// the capacity of all fields should be the same.
            pub fn capacity(&self) -> usize {
                let vec: Vec<usize> = vec![#(self.data.#fields_names_1.capacity()),*];
                match vec.iter().min() {
                    None => usize::MAX, // If there are no fields, capacity is the maximum possible (no need to allocate anything).
                    Some(result) => *result
                }
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::reserve()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.reserve),
            /// reserving the same `additional` space for all fields.
            pub fn reserve(&mut self, additional: usize) {
                #(self.data.#fields_names_1.reserve(self.len, additional);)*
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::reserve_exact()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.reserve_exact)
            /// reserving the same `additional` space for all fields.
            pub fn reserve_exact(&mut self, additional: usize) {
                #(self.data.#fields_names_1.reserve_exact(self.len, additional);)*
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::truncate()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.truncate)
            /// truncating all fields.
            pub fn truncate(&mut self, len: usize) {
                unsafe {
                    // Destroy the elements that are outside the given length
                    while len < self.len {
                        let i = self.len - 1;

                        // Decrement len before calling drop_in_place() so a panic on Drop
                        //   doesn't try to drop it a second time.
                        self.len -= 1;

                        #(ptr::drop_in_place(self.data.#fields_names_1.ptr().add(i));)*
                    }
                }
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::push()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.push).
            pub fn push(&mut self, value: #name) {
                fn write_to_raw_vec<T>(buf: &mut RawVec<T>, value: T, index: usize) {
                    unsafe {
                        let ptr = buf.ptr().add(index);
                        ptr::write(ptr, value);
                    }
                }

                let #name{#(#fields_names_1),*} = value;
                self.reserve(1);
                #(write_to_raw_vec(&mut self.data.#fields_names_1, #fields_names_2, self.len);)*
                self.len += 1;
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::len()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.len)
            pub fn len(&self) -> usize {
                self.len
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::is_empty()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.is_empty),
            /// all the fields should have the same length.
            pub fn is_empty(&self) -> bool {
                self.len == 0
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::swap_remove()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.swap_remove).
            pub fn swap_remove(&mut self, index: usize) -> #name {
                let length = self.len;
                let slices = self.as_mut_slice();
                #(slices.#fields_names_1.swap(index, length - 1);)*
                self.pop().unwrap()
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::insert()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.insert).
            pub fn insert(&mut self, index: usize, element: #name) {
                fn insert_into_raw_vec<T>(buf: &mut RawVec<T>, len: usize, value: T, index: usize) {
                    unsafe {
                        // infallible
                        // The spot to put the new value
                        let p = buf.ptr().add(index);
                        // Shift everything over to make space. (Duplicating the
                        // `index`th element into two consecutive places.)
                        ptr::copy(p, p.offset(1), len - index);
                        // Write it in, overwriting the first copy of the `index`th
                        // element.
                        ptr::write(p, value);
                    }
                }

                let #name{#(#fields_names_1),*} = element;
                let len = self.len();
                assert!(index <= len);

                self.reserve(1);
                #(insert_into_raw_vec(&mut self.data.#fields_names_1, len, #fields_names_2, index);)*
                self.len += 1;
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::remove()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.remove).
            pub fn remove(&mut self, index: usize) -> #name {
                fn raw_vec_remove<T>(buf: &mut RawVec<T>, len: usize, index: usize) -> T {
                    unsafe {
                        // infallible
                        let ret;
                        {
                            // the place we are taking from.
                            let ptr = buf.ptr().add(index);
                            // copy it out, unsafely having a copy of the value on
                            // the stack and in the vector at the same time.
                            ret = ptr::read(ptr);

                            // Shift everything down to fill in that spot.
                            ptr::copy(ptr.offset(1), ptr, len - index - 1);
                        }
                        ret
                    }
                }

                let len = self.len();
                assert!(index < len);
                self.len -= 1;
                #name{#(#fields_names_1: raw_vec_remove(&mut self.data.#fields_names_2, len, index)),*}
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::pop()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.pop).
            pub fn pop(&mut self) -> Option<#name> {
                if self.is_empty() {
                    None
                } else {
                    self.len -= 1;

                    unsafe {
                        #(
                            let #fields_names_1 = ptr::read(self.data.#fields_names_2.ptr().offset(self.len as isize));
                        )*
                        Some(#name{#(#fields_names_1: #fields_names_2),*})
                    }
                }
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::append()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.append).
            pub fn append(&mut self, other: &mut #vec_name) {
                fn append_raw_vec<T>(src: &RawVec<T>, srclen: usize, dst: &mut RawVec<T>, dstlen: usize) {
                    dst.reserve(dstlen, srclen);
                    unsafe {
                        ptr::copy_nonoverlapping(src.ptr(), dst.ptr().add(dstlen), srclen)
                    }
                }

                let len = self.len();
                let otherlen = other.len();
                self.reserve(otherlen);
                #(
                    append_raw_vec(&other.data.#fields_names_1, otherlen, &mut self.data.#fields_names_2, len);
                )*
                other.len = 0;
                self.len += otherlen;
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::clear()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.clear).
            pub fn clear(&mut self) {
                self.truncate(0);
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::as_slice()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.as_slice).
            pub fn as_slice(&self) -> #slice_name {
                unsafe {
                    #slice_name {
                        #(#fields_names_1 : slice::from_raw_parts(self.data.#fields_names_2.ptr(), self.len()), )*
                    }
                }
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::as_mut_slice()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.as_mut_slice).
            pub fn as_mut_slice(&mut self) -> #slice_mut_name {
                unsafe {
                    #slice_mut_name {
                        #(#fields_names_1 : slice::from_raw_parts_mut(self.data.#fields_names_2.ptr(), self.len()), )*
                    }
                }
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::as_ptr()`](https://doc.rust-lang.org/std/struct.Vec.html#method.as_ptr).
            pub fn as_ptr(&self) -> #ptr_name {
                #ptr_name {
                    #(#fields_names_1: self.data.#fields_names_2.ptr(),)*
                }
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::as_mut_ptr()`](https://doc.rust-lang.org/std/struct.Vec.html#method.as_mut_ptr).
            pub fn as_mut_ptr(&mut self) -> #ptr_mut_name {
                #ptr_mut_name {
                    #(#fields_names_1: self.data.#fields_names_2.ptr(),)*
                }
            }
        }
    };

    return generated;
}
