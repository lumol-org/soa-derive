use quote::Tokens;
use structs::Struct;

pub fn derive(input: &Struct) -> Tokens {
    let name = &input.name;
    let visibility = &input.visibility;
    let ref_name = &input.ref_name();
    let ref_mut_name = &input.ref_mut_name();
    let fields_names = &input.fields.iter()
                                    .map(|field| field.ident.clone().unwrap())
                                    .collect::<Vec<_>>();

    let fields_types = &input.fields.iter()
                                    .map(|field| &field.ty)
                                    .collect::<Vec<_>>();

    quote!{
        #[derive(Debug)]
        #visibility struct #ref_name<'a> {
            #(pub #fields_names: &'a #fields_types,)*
        }

        #[derive(Debug)]
        #visibility struct #ref_mut_name<'a> {
            #(pub #fields_names: &'a mut #fields_types,)*
        }
    }
}
