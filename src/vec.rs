use quote::Tokens;
use structs::Struct;

pub fn derive(input: &Struct) -> Tokens {
    let name = &input.name;
    let vec_name_str = format!("Vec<{}>", name);
    let derives = &input.derive;
    let visibility = &input.visibility;
    let vec_name = &input.vec_name();
    let slice_name = &input.slice_name();
    let slice_mut_name = &input.slice_mut_name();
    let ref_name = &input.ref_name();

    let fields_names = input.fields.iter()
                                   .map(|field| field.ident.clone().unwrap())
                                   .collect::<Vec<_>>();
    let fields_names_1 = &fields_names;
    let fields_names_2 = &fields_names;
    let first_field = &fields_names[0];

    let fields_types = &input.fields.iter()
                                    .map(|field| &field.ty)
                                    .collect::<Vec<_>>();

    quote! {
        /// An analog to `
        #[doc = #vec_name_str]
        /// ` with Struct of Array (SoA) layout
        #[allow(dead_code)]
        #derives
        #visibility struct #vec_name {
            #(pub #fields_names_1: Vec<#fields_types>,)*
        }

        #[allow(dead_code)]
        impl #vec_name {
            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::new()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.new)
            pub fn new() -> #vec_name {
                #vec_name {
                    #(#fields_names_1 : Vec::new(),)*
                }
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::with_capacity()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.with_capacity),
            /// initializing all fields with the given `capacity`.
            pub fn with_capacity(capacity: usize) -> #vec_name {
                #vec_name {
                    #(#fields_names_1 : Vec::with_capacity(capacity),)*
                }
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::capacity()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.capacity),
            /// the capacity of all fields should be the same.
            pub fn capacity(&self) -> usize {
                let capacity = self.#first_field.capacity();
                #(debug_assert_eq!(self.#fields_names_1.capacity(), capacity);)*
                capacity
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::reserve()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.reserve),
            /// reserving the same `additional` space for all fields.
            pub fn reserve(&mut self, additional: usize) {
                #(self.#fields_names_1.reserve(additional);)*
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::reserve_exact()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.reserve_exact)
            /// reserving the same `additional` space for all fields.
            pub fn reserve_exact(&mut self, additional: usize) {
                #(self.#fields_names_1.reserve_exact(additional);)*
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::shrink_to_fit()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.shrink_to_fit)
            /// shrinking all fields.
            pub fn shrink_to_fit(&mut self) {
                #(self.#fields_names_1.shrink_to_fit();)*
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::truncate()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.truncate)
            /// truncating all fields.
            pub fn truncate(&mut self, len: usize) {
                #(self.#fields_names_1.truncate(len);)*
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::push()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.push).
            pub fn push(&mut self, value: #name) {
                let #name{#(#fields_names_1),*} = value;
                #(self.#fields_names_1.push(#fields_names_2);)*
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::len()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.len),
            /// all the fields should have the same length.
            pub fn len(&self) -> usize {
                let len = self.#first_field.len();
                #(debug_assert_eq!(self.#fields_names_1.len(), len);)*
                len
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::is_empty()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.is_empty),
            /// all the fields should have the same length.
            pub fn is_empty(&self) -> bool {
                let empty = self.#first_field.is_empty();
                #(debug_assert_eq!(self.#fields_names_1.is_empty(), empty);)*
                empty
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::swap_remove()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.swap_remove).
            pub fn swap_remove(&mut self, index: usize) -> #name {
                #(
                    let #fields_names_1 = self.#fields_names_2.swap_remove(index);
                )*
                #name{#(#fields_names_1: #fields_names_2),*}
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::insert()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.insert).
            pub fn insert(&mut self, index: usize, element: #name) {
                let #name{#(#fields_names_1),*} = element;
                #(self.#fields_names_1.insert(index, #fields_names_2);)*
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::remove()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.remove).
            pub fn remove(&mut self, index: usize) -> #name {
                #(
                    let #fields_names_1 = self.#fields_names_2.remove(index);
                )*
                #name{#(#fields_names_1: #fields_names_2),*}
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::pop()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.pop).
            pub fn pop(&mut self) -> Option<#name> {
                if self.is_empty() {
                    None
                } else {
                    #(
                        let #fields_names_1 = self.#fields_names_2.pop().unwrap();
                    )*
                    Some(#name{#(#fields_names_1: #fields_names_2),*})
                }
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::append()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.append).
            pub fn append(&mut self, other: &mut #vec_name) {
                #(
                    self.#fields_names_1.append(&mut other.#fields_names_2);
                )*
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::clear()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.clear).
            pub fn clear(&mut self) {
                #(self.#fields_names_1.clear();)*
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::resize()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.resize).
            pub fn resize<T>(&mut self, new_len: usize, value: #name) {
                #(
                    self.#fields_names_1.resize(new_len, value.#fields_names_2);
                )*
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::split_off()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.split_off).
            pub fn split_off(&mut self, at: usize) -> #vec_name {
                #vec_name {
                    #(#fields_names_1 : self.#fields_names_2.split_off(at), )*
                }
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::as_slice()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.as_slice).
            pub fn as_slice(&self) -> #slice_name {
                #slice_name {
                    #(#fields_names_1 : &self.#fields_names_2, )*
                }
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::as_mut_slice()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.as_mut_slice).
            pub fn as_mut_slice(&mut self) -> #slice_mut_name {
                #slice_mut_name {
                    #(#fields_names_1 : &mut self.#fields_names_2, )*
                }
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::retain()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.retain).
            pub fn retain<F>(&mut self, mut f: F) where F: FnMut(#ref_name) -> bool {
                let len = self.len();
                let mut del = 0;

                {
                    let mut slice = self.as_mut_slice();
                    for i in 0..len {
                        if !f(slice.get(i).unwrap()) {
                            del += 1;
                        } else if del > 0 {
                            slice.swap(i - del, i);
                        }
                    }
                }
                if del > 0 {
                    self.truncate(len - del);
                }
            }
        }
    }
}
