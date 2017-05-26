use quote::Tokens;
use structs::Struct;

pub fn derive(input: &Struct) -> Tokens {
    let name = &input.name;
    let visibility = &input.visibility;
    let derives = &input.derive;
    let ref_name = &input.ref_name();
    let ref_mut_name = &input.ref_mut_name();
    let fields_names = input.fields.iter()
                                   .map(|field| field.ident.clone().unwrap())
                                   .collect::<Vec<_>>();
    let fields_names_1 = &fields_names;
    let fields_names_2 = &fields_names;

    let fields_types = &input.fields.iter()
                                    .map(|field| &field.ty)
                                    .collect::<Vec<_>>();

    quote!{
        #derives
        #visibility struct #ref_name<'a> {
            #(pub #fields_names_1: &'a #fields_types,)*
        }

        #derives
        #visibility struct #ref_mut_name<'a> {
            #(pub #fields_names_1: &'a mut #fields_types,)*
        }

        impl #name {
            pub fn as_ref(&self) -> #ref_name {
                #ref_name {
                    #(#fields_names_1: & self.#fields_names_2, )*
                }
            }

            pub fn as_mut(&mut self) -> #ref_mut_name {
                #ref_mut_name {
                    #(#fields_names_1: &mut self.#fields_names_2, )*
                }
            }
        }
    }
}
