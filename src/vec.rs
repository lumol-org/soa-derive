use quote::Tokens;
use structs::Struct;

pub fn derive(input: &Struct) -> Tokens {
    let name = &input.name;
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

    quote!{
        #[derive(#(#derives,)*)]
        #visibility struct #vec_name {
            #(pub #fields_names_1: Vec<#fields_types>,)*
        }

        #[allow(dead_code)]
        impl #vec_name {
            pub fn new() -> #vec_name {
                #vec_name {
                    #(#fields_names_1 : Vec::new(),)*
                }
            }

            pub fn with_capacity(capacity: usize) -> #vec_name {
                #vec_name {
                    #(#fields_names_1 : Vec::with_capacity(capacity),)*
                }
            }

            pub fn capacity(&self) -> usize {
                // We use the minimal capacity as the whole capacity
                let capacities = [
                    #(self.#fields_names_1.capacity(),)*
                ];
                *capacities.iter().min().unwrap()
            }

            pub fn reserve(&mut self, additional: usize) {
                #(self.#fields_names_1.reserve(additional);)*
            }

            pub fn reserve_exact(&mut self, additional: usize) {
                #(self.#fields_names_1.reserve_exact(additional);)*
            }

            pub fn shrink_to_fit(&mut self) {
                #(self.#fields_names_1.shrink_to_fit();)*
            }

            pub fn truncate(&mut self, len: usize) {
                #(self.#fields_names_1.truncate(len);)*
            }

            pub fn push(&mut self, value: #name) {
                let #name{#(#fields_names_1),*} = value;
                #(self.#fields_names_1.push(#fields_names_2);)*
            }

            pub fn len(&self) -> usize {
                let len = self.#first_field.len();
                #(debug_assert_eq!(self.#fields_names_1.len(), len);)*
                len
            }

            pub fn is_empty(&self) -> bool {
                let empty = self.#first_field.is_empty();
                #(debug_assert_eq!(self.#fields_names_1.is_empty(), empty);)*
                empty
            }

            pub fn swap_remove(&mut self, index: usize) -> #name {
                #(
                    let #fields_names_1 = self.#fields_names_2.swap_remove(index);
                )*
                #name{#(#fields_names_1: #fields_names_2),*}
            }

            pub fn insert(&mut self, index: usize, element: #name) {
                let #name{#(#fields_names_1),*} = element;
                #(self.#fields_names_1.insert(index, #fields_names_2);)*
            }

            pub fn remove(&mut self, index: usize) -> #name {
                #(
                    let #fields_names_1 = self.#fields_names_2.remove(index);
                )*
                #name{#(#fields_names_1: #fields_names_2),*}
            }

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

            pub fn append(&mut self, other: &mut #vec_name) {
                #(
                    self.#fields_names_1.append(&mut other.#fields_names_2);
                )*
            }

            pub fn clear(&mut self) {
                #(self.#fields_names_1.clear();)*
            }

            pub fn resize<T>(&mut self, new_len: usize, value: #name) {
                #(
                    self.#fields_names_1.resize(new_len, value.#fields_names_2);
                )*
            }

            pub fn split_off(&mut self, at: usize) -> #vec_name {
                #vec_name {
                    #(#fields_names_1 : self.#fields_names_2.split_off(at), )*
                }
            }

            pub fn as_slice(&self) -> #slice_name {
                #slice_name {
                    #(#fields_names_1 : &self.#fields_names_2, )*
                }
            }

            pub fn as_mut_slice(&mut self) -> #slice_mut_name {
                #slice_mut_name {
                    #(#fields_names_1 : &mut self.#fields_names_2, )*
                }
            }

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
