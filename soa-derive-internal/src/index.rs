use proc_macro2::TokenStream;

use quote::quote;

use crate::input::Input;
use crate::names;

pub fn derive(input: &Input) -> TokenStream {
    let vec_name = names::vec_name(&input.name);
    let slice_name = names::slice_name(&input.name);
    let slice_mut_name = names::slice_mut_name(&input.name);
    let ref_name = names::ref_name(&input.name);
    let ref_mut_name = names::ref_mut_name(&input.name);

    let fields_names = &input.fields.iter()
        .map(|field| field.ident.clone().unwrap())
        .collect::<Vec<_>>();
    let first_field_name = &fields_names[0];


    let get_unchecked = input.map_fields_nested_or(
        |ident, _| quote! { ::soa_derive::SoAIndex::get_unchecked(self.clone(), slice.#ident) },
        |ident, _| quote! { slice.#ident.get_unchecked(self.clone()) },
    ).collect::<Vec<_>>();

    let get_unchecked_mut = input.map_fields_nested_or(
        |ident, _| quote! { ::soa_derive::SoAIndexMut::get_unchecked_mut(self.clone(), slice.#ident) },
        |ident, _| quote! { slice.#ident.get_unchecked_mut(self.clone()) },
    ).collect::<Vec<_>>();

    let index = input.map_fields_nested_or(
        |ident, _| quote! { ::soa_derive::SoAIndex::index(self.clone(), slice.#ident) },
        |ident, _| quote! { & slice.#ident[self.clone()] },
    ).collect::<Vec<_>>();

    let index_mut = input.map_fields_nested_or(
        |ident, _| quote! { ::soa_derive::SoAIndexMut::index_mut(self.clone(), slice.#ident) },
        |ident, _| quote! { &mut slice.#ident[self.clone()] },
    ).collect::<Vec<_>>();

    quote!{
        // usize
        impl<'a> ::soa_derive::SoAIndex<&'a #vec_name> for usize {
            type RefOutput = #ref_name<'a>;

            #[inline]
            fn get(self, soa: &'a #vec_name) -> Option<Self::RefOutput> {
                if self < soa.len() {
                    Some(unsafe { ::soa_derive::SoAIndex::get_unchecked(self, soa) })
                } else {
                    None
                }
            }

            #[inline]
            unsafe fn get_unchecked(self, soa: &'a #vec_name) -> Self::RefOutput {
                ::soa_derive::SoAIndex::get_unchecked(self, soa.as_slice())
            }

            #[inline]
            fn index(self, soa: &'a #vec_name) -> Self::RefOutput {
                ::soa_derive::SoAIndex::index(self, soa.as_slice())
            }
        }

        impl<'a> ::soa_derive::SoAIndexMut<&'a mut #vec_name> for usize {
            type MutOutput = #ref_mut_name<'a>;

            #[inline]
            fn get_mut(self, soa: &'a mut #vec_name) -> Option<Self::MutOutput> {
                if self < soa.len() {
                    Some(unsafe { ::soa_derive::SoAIndexMut::get_unchecked_mut(self, soa) })
                } else {
                    None
                }
            }

            #[inline]
            unsafe fn get_unchecked_mut(self, soa: &'a mut #vec_name) -> Self::MutOutput {
                ::soa_derive::SoAIndexMut::get_unchecked_mut(self, soa.as_mut_slice())
            }

            #[inline]
            fn index_mut(self, soa: &'a mut #vec_name) -> Self::MutOutput {
                ::soa_derive::SoAIndexMut::index_mut(self, soa.as_mut_slice())
            }
        }



        // Range<usize>
        impl<'a> ::soa_derive::SoAIndex<&'a #vec_name> for ::std::ops::Range<usize> {
            type RefOutput = #slice_name<'a>;

            #[inline]
            fn get(self, soa: &'a #vec_name) -> Option<Self::RefOutput> {
                if self.start <= self.end && self.end <= soa.len() {
                    unsafe { Some(::soa_derive::SoAIndex::get_unchecked(self, soa)) }
                } else {
                    None
                }
            }

            #[inline]
            unsafe fn get_unchecked(self, soa: &'a #vec_name) -> Self::RefOutput {
                ::soa_derive::SoAIndex::get_unchecked(self, soa.as_slice())
            }

            #[inline]
            fn index(self, soa: &'a #vec_name) -> Self::RefOutput {
                ::soa_derive::SoAIndex::index(self, soa.as_slice())
            }
        }

        impl<'a> ::soa_derive::SoAIndexMut<&'a mut #vec_name> for ::std::ops::Range<usize> {
            type MutOutput = #slice_mut_name<'a>;

            #[inline]
            fn get_mut(self, soa: &'a mut #vec_name) -> Option<Self::MutOutput> {
                if self.start <= self.end && self.end <= soa.len() {
                    unsafe { Some(::soa_derive::SoAIndexMut::get_unchecked_mut(self, soa)) }
                } else {
                    None
                }
            }

            #[inline]
            unsafe fn get_unchecked_mut(self, soa: &'a mut #vec_name) -> Self::MutOutput {
                ::soa_derive::SoAIndexMut::get_unchecked_mut(self, soa.as_mut_slice())
            }

            #[inline]
            fn index_mut(self, soa: &'a mut #vec_name) -> Self::MutOutput {
               ::soa_derive::SoAIndexMut::index_mut(self, soa.as_mut_slice())
            }
        }

        // RangeTo<usize>
        impl<'a> ::soa_derive::SoAIndex<&'a #vec_name> for ::std::ops::RangeTo<usize> {
            type RefOutput = #slice_name<'a>;

            #[inline]
            fn get(self, soa: &'a #vec_name) -> Option<Self::RefOutput> {
                ::soa_derive::SoAIndex::get(0..self.end, soa)
            }

            #[inline]
            unsafe fn get_unchecked(self, soa: &'a #vec_name) -> Self::RefOutput {
                ::soa_derive::SoAIndex::get_unchecked(0..self.end, soa)
            }

            #[inline]
            fn index(self, soa: &'a #vec_name) -> Self::RefOutput {
                ::soa_derive::SoAIndex::index(0..self.end, soa)
            }
        }

        impl<'a> ::soa_derive::SoAIndexMut<&'a mut #vec_name> for ::std::ops::RangeTo<usize> {
            type MutOutput = #slice_mut_name<'a>;

            #[inline]
            fn get_mut(self, soa: &'a mut #vec_name) -> Option<Self::MutOutput> {
                ::soa_derive::SoAIndexMut::get_mut(0..self.end, soa)
            }

            #[inline]
            unsafe fn get_unchecked_mut(self, soa: &'a mut #vec_name) -> Self::MutOutput {
                ::soa_derive::SoAIndexMut::get_unchecked_mut(0..self.end, soa)
            }

            #[inline]
            fn index_mut(self, soa: &'a mut #vec_name) -> Self::MutOutput {
                ::soa_derive::SoAIndexMut::index_mut(0..self.end, soa)
            }
        }

        // RangeFrom<usize>
        impl<'a> ::soa_derive::SoAIndex<&'a #vec_name> for ::std::ops::RangeFrom<usize> {
            type RefOutput = #slice_name<'a>;

            #[inline]
            fn get(self, soa: &'a #vec_name) -> Option<Self::RefOutput> {
                ::soa_derive::SoAIndex::get(self.start..soa.len(), soa)
            }

            #[inline]
            unsafe fn get_unchecked(self, soa: &'a #vec_name) -> Self::RefOutput {
                ::soa_derive::SoAIndex::get_unchecked(self.start..soa.len(), soa)
            }

            #[inline]
            fn index(self, soa: &'a #vec_name) -> Self::RefOutput {
                ::soa_derive::SoAIndex::index(self.start..soa.len(), soa)
            }
        }

        impl<'a> ::soa_derive::SoAIndexMut<&'a mut #vec_name> for ::std::ops::RangeFrom<usize> {
            type MutOutput = #slice_mut_name<'a>;

            #[inline]
            fn get_mut(self, soa: &'a mut #vec_name) -> Option<Self::MutOutput> {
               ::soa_derive::SoAIndexMut::get_mut(self.start..soa.len(), soa)
            }

            #[inline]
            unsafe fn get_unchecked_mut(self, soa: &'a mut #vec_name) -> Self::MutOutput {
                ::soa_derive::SoAIndexMut::get_unchecked_mut(self.start..soa.len(), soa)
            }

            #[inline]
            fn index_mut(self, soa: &'a mut #vec_name) -> Self::MutOutput {
                ::soa_derive::SoAIndexMut::index_mut(self.start..soa.len(), soa)
            }
        }

        // RangeFull
        impl<'a> ::soa_derive::SoAIndex<&'a #vec_name> for ::std::ops::RangeFull {
            type RefOutput = #slice_name<'a>;

            #[inline]
            fn get(self, soa: &'a #vec_name) -> Option<Self::RefOutput> {
                Some(soa.as_slice())
            }

            #[inline]
            unsafe fn get_unchecked(self, soa: &'a #vec_name) -> Self::RefOutput {
                soa.as_slice()
            }

            #[inline]
            fn index(self, soa: &'a #vec_name) -> Self::RefOutput {
                soa.as_slice()
            }
        }

        impl<'a> ::soa_derive::SoAIndexMut<&'a mut #vec_name> for ::std::ops::RangeFull {
            type MutOutput = #slice_mut_name<'a>;

            #[inline]
            fn get_mut(self, soa: &'a mut #vec_name) -> Option<Self::MutOutput> {
                Some(soa.as_mut_slice())
            }

            #[inline]
            unsafe fn get_unchecked_mut(self, soa: &'a mut #vec_name) -> Self::MutOutput {
                soa.as_mut_slice()
            }

            #[inline]
            fn index_mut(self, soa: &'a mut #vec_name) -> Self::MutOutput {
                soa.as_mut_slice()
            }
        }

        // RangeInclusive<usize>
        impl<'a> ::soa_derive::SoAIndex<&'a #vec_name> for ::std::ops::RangeInclusive<usize> {
            type RefOutput = #slice_name<'a>;

            #[inline]
            fn get(self, soa: &'a #vec_name) -> Option<Self::RefOutput> {
                if *self.end() == usize::MAX {
                    None
                } else {
                    ::soa_derive::SoAIndex::get(*self.start()..self.end() + 1, soa)
                }
            }

            #[inline]
            unsafe fn get_unchecked(self, soa: &'a #vec_name) -> Self::RefOutput {
                ::soa_derive::SoAIndex::get_unchecked(*self.start()..self.end() + 1, soa)
            }

            #[inline]
            fn index(self, soa: &'a #vec_name) -> Self::RefOutput {
                ::soa_derive::SoAIndex::index(*self.start()..self.end() + 1, soa)
            }
        }

        impl<'a> ::soa_derive::SoAIndexMut<&'a mut #vec_name> for ::std::ops::RangeInclusive<usize> {
            type MutOutput = #slice_mut_name<'a>;

            #[inline]
            fn get_mut(self, soa: &'a mut #vec_name) -> Option<Self::MutOutput> {
                if *self.end() == usize::MAX {
                    None
                } else {
                    ::soa_derive::SoAIndexMut::get_mut(*self.start()..self.end() + 1, soa)
                }
            }

            #[inline]
            unsafe fn get_unchecked_mut(self, soa: &'a mut #vec_name) -> Self::MutOutput {
                ::soa_derive::SoAIndexMut::get_unchecked_mut(*self.start()..self.end() + 1, soa)
            }

            #[inline]
            fn index_mut(self, soa: &'a mut #vec_name) -> Self::MutOutput {
                ::soa_derive::SoAIndexMut::index_mut(*self.start()..self.end() + 1, soa)
            }
        }

        // RangeToInclusive<usize>
        impl<'a> ::soa_derive::SoAIndex<&'a #vec_name> for ::std::ops::RangeToInclusive<usize> {
            type RefOutput = #slice_name<'a>;

            #[inline]
            fn get(self, soa: &'a #vec_name) -> Option<Self::RefOutput> {
                ::soa_derive::SoAIndex::get(0..=self.end, soa)
            }

            #[inline]
            unsafe fn get_unchecked(self, soa: &'a #vec_name) -> Self::RefOutput {
                ::soa_derive::SoAIndex::get_unchecked(0..=self.end, soa)
            }

            #[inline]
            fn index(self, soa: &'a #vec_name) -> Self::RefOutput {
                ::soa_derive::SoAIndex::index(0..=self.end, soa)
            }
        }

        impl<'a> ::soa_derive::SoAIndexMut<&'a mut #vec_name> for ::std::ops::RangeToInclusive<usize> {
            type MutOutput = #slice_mut_name<'a>;

            #[inline]
            fn get_mut(self, soa: &'a mut #vec_name) -> Option<Self::MutOutput> {
                ::soa_derive::SoAIndexMut::get_mut(0..=self.end, soa)
            }

            #[inline]
            unsafe fn get_unchecked_mut(self, soa: &'a mut #vec_name) -> Self::MutOutput {
                ::soa_derive::SoAIndexMut::get_unchecked_mut(0..=self.end, soa)
            }

            #[inline]
            fn index_mut(self, soa: &'a mut #vec_name) -> Self::MutOutput {
                ::soa_derive::SoAIndexMut::index_mut(0..=self.end, soa)
            }
        }

        // usize
        impl<'a> ::soa_derive::SoAIndex<#slice_name<'a>> for usize {
            type RefOutput = #ref_name<'a>;

            #[inline]
            fn get(self, slice: #slice_name<'a>) -> Option<Self::RefOutput> {
                if self < slice.#first_field_name.len() {
                    Some(unsafe { ::soa_derive::SoAIndex::get_unchecked(self, slice) })
                } else {
                    None
                }
            }

            #[inline]
            unsafe fn get_unchecked(self, slice: #slice_name<'a>) -> Self::RefOutput {
                #ref_name {
                    #( #fields_names: #get_unchecked, )*
                }
            }

            #[inline]
            fn index(self, slice: #slice_name<'a>) -> Self::RefOutput {
                #ref_name {
                    #( #fields_names: #index, )*
                }
            }
        }

        impl<'a> ::soa_derive::SoAIndexMut<#slice_mut_name<'a>> for usize {
            type MutOutput = #ref_mut_name<'a>;

            #[inline]
            fn get_mut(self, slice: #slice_mut_name<'a>) -> Option<Self::MutOutput> {
                if self < slice.len() {
                    Some(unsafe { ::soa_derive::SoAIndexMut::get_unchecked_mut(self, slice) })
                } else {
                    None
                }
            }

            #[inline]
            unsafe fn get_unchecked_mut(self, slice: #slice_mut_name<'a>) -> Self::MutOutput {
                #ref_mut_name {
                    #( #fields_names: #get_unchecked_mut, )*
                }
            }

            #[inline]
            fn index_mut(self, slice: #slice_mut_name<'a>) -> Self::MutOutput {
                #ref_mut_name {
                    #( #fields_names: #index_mut, )*
                }
            }
        }



        // Range<usize>
        impl<'a> ::soa_derive::SoAIndex<#slice_name<'a>> for ::std::ops::Range<usize> {
            type RefOutput = #slice_name<'a>;

            #[inline]
            fn get(self, slice: #slice_name<'a>) -> Option<Self::RefOutput> {
                if self.start <= self.end && self.end <= slice.#first_field_name.len() {
                    unsafe { Some(::soa_derive::SoAIndex::get_unchecked(self, slice)) }
                } else {
                    None
                }
            }

            #[inline]
            unsafe fn get_unchecked(self, slice: #slice_name<'a>) -> Self::RefOutput {
                #slice_name {
                    #( #fields_names: #get_unchecked, )*
                }
            }

            #[inline]
            fn index(self, slice: #slice_name<'a>) -> Self::RefOutput {
                #slice_name {
                    #( #fields_names: #index, )*
                }
            }
        }

        impl<'a> ::soa_derive::SoAIndexMut<#slice_mut_name<'a>> for ::std::ops::Range<usize> {
            type MutOutput = #slice_mut_name<'a>;

            #[inline]
            fn get_mut(self, slice: #slice_mut_name<'a>) -> Option<Self::MutOutput> {
                if self.start <= self.end && self.end <= slice.#first_field_name.len() {
                    unsafe { Some(::soa_derive::SoAIndexMut::get_unchecked_mut(self, slice)) }
                } else {
                    None
                }
            }

            #[inline]
            unsafe fn get_unchecked_mut(self, slice: #slice_mut_name<'a>) -> Self::MutOutput {
                #slice_mut_name {
                    #( #fields_names: #get_unchecked_mut, )*
                }
            }

            #[inline]
            fn index_mut(self, slice: #slice_mut_name<'a>) -> Self::MutOutput {
                #slice_mut_name {
                    #( #fields_names: #index_mut, )*
                }
            }
        }



        // RangeTo<usize>
        impl<'a> ::soa_derive::SoAIndex<#slice_name<'a>> for ::std::ops::RangeTo<usize> {
            type RefOutput = #slice_name<'a>;

            #[inline]
            fn get(self, slice: #slice_name<'a>) -> Option<Self::RefOutput> {
                ::soa_derive::SoAIndex::get(0..self.end, slice)
            }

            #[inline]
            unsafe fn get_unchecked(self, slice: #slice_name<'a>) -> Self::RefOutput {
                ::soa_derive::SoAIndex::get_unchecked(0..self.end, slice)
            }

            #[inline]
            fn index(self, slice: #slice_name<'a>) -> Self::RefOutput {
                ::soa_derive::SoAIndex::index(0..self.end, slice)
            }
        }

        impl<'a> ::soa_derive::SoAIndexMut<#slice_mut_name<'a>> for ::std::ops::RangeTo<usize> {
            type MutOutput = #slice_mut_name<'a>;

            #[inline]
            fn get_mut(self, slice: #slice_mut_name<'a>) -> Option<Self::MutOutput> {
                ::soa_derive::SoAIndexMut::get_mut(0..self.end, slice)
            }

            #[inline]
            unsafe fn get_unchecked_mut(self, slice: #slice_mut_name<'a>) -> Self::MutOutput {
                ::soa_derive::SoAIndexMut::get_unchecked_mut(0..self.end, slice)
            }

            #[inline]
            fn index_mut(self, slice: #slice_mut_name<'a>) -> Self::MutOutput {
                ::soa_derive::SoAIndexMut::index_mut(0..self.end, slice)
            }
        }


        // RangeFrom<usize>
        impl<'a> ::soa_derive::SoAIndex<#slice_name<'a>> for ::std::ops::RangeFrom<usize> {
            type RefOutput = #slice_name<'a>;

            #[inline]
            fn get(self, slice: #slice_name<'a>) -> Option<Self::RefOutput> {
                ::soa_derive::SoAIndex::get(self.start..slice.len(), slice)
            }

            #[inline]
            unsafe fn get_unchecked(self, slice: #slice_name<'a>) -> Self::RefOutput {
                ::soa_derive::SoAIndex::get_unchecked(self.start..slice.len(), slice)
            }

            #[inline]
            fn index(self, slice: #slice_name<'a>) -> Self::RefOutput {
                ::soa_derive::SoAIndex::index(self.start..slice.len(), slice)
            }
        }

        impl<'a> ::soa_derive::SoAIndexMut<#slice_mut_name<'a>> for ::std::ops::RangeFrom<usize> {
            type MutOutput = #slice_mut_name<'a>;

            #[inline]
            fn get_mut(self, slice: #slice_mut_name<'a>) -> Option<Self::MutOutput> {
                ::soa_derive::SoAIndexMut::get_mut(self.start..slice.len(), slice)
            }

            #[inline]
            unsafe fn get_unchecked_mut(self, slice: #slice_mut_name<'a>) -> Self::MutOutput {
                ::soa_derive::SoAIndexMut::get_unchecked_mut(self.start..slice.len(), slice)
            }

            #[inline]
            fn index_mut(self, slice: #slice_mut_name<'a>) -> Self::MutOutput {
                ::soa_derive::SoAIndexMut::index_mut(self.start..slice.len(), slice)
            }
        }


        // RangeFull
        impl<'a> ::soa_derive::SoAIndex<#slice_name<'a>> for ::std::ops::RangeFull {
            type RefOutput = #slice_name<'a>;

            #[inline]
            fn get(self, slice: #slice_name<'a>) -> Option<Self::RefOutput> {
                Some(slice)
            }

            #[inline]
            unsafe fn get_unchecked(self, slice: #slice_name<'a>) -> Self::RefOutput {
                slice
            }

            #[inline]
            fn index(self, slice: #slice_name<'a>) -> Self::RefOutput {
                slice
            }
        }

        impl<'a> ::soa_derive::SoAIndexMut<#slice_mut_name<'a>> for ::std::ops::RangeFull {
            type MutOutput = #slice_mut_name<'a>;

            #[inline]
            fn get_mut(self, slice: #slice_mut_name<'a>) -> Option<Self::MutOutput> {
                Some(slice)
            }

            #[inline]
            unsafe fn get_unchecked_mut(self, slice: #slice_mut_name<'a>) -> Self::MutOutput {
                slice
            }

            #[inline]
            fn index_mut(self, slice: #slice_mut_name<'a>) -> Self::MutOutput {
                slice
            }
        }


        // RangeInclusive<usize>
        impl<'a> ::soa_derive::SoAIndex<#slice_name<'a>> for ::std::ops::RangeInclusive<usize> {
            type RefOutput = #slice_name<'a>;

            #[inline]
            fn get(self, slice: #slice_name<'a>) -> Option<Self::RefOutput> {
                if *self.end() == usize::MAX {
                    None
                } else {
                    ::soa_derive::SoAIndex::get(*self.start()..self.end() + 1, slice)
                }
            }

            #[inline]
            unsafe fn get_unchecked(self, slice: #slice_name<'a>) -> Self::RefOutput {
                ::soa_derive::SoAIndex::get_unchecked(*self.start()..self.end() + 1, slice)
            }

            #[inline]
            fn index(self, slice: #slice_name<'a>) -> Self::RefOutput {
                ::soa_derive::SoAIndex::index(*self.start()..self.end() + 1, slice)
            }
        }

        impl<'a> ::soa_derive::SoAIndexMut<#slice_mut_name<'a>> for ::std::ops::RangeInclusive<usize> {
            type MutOutput = #slice_mut_name<'a>;

            #[inline]
            fn get_mut(self, slice: #slice_mut_name<'a>) -> Option<Self::MutOutput> {
                if *self.end() == usize::MAX {
                    None
                } else {
                    ::soa_derive::SoAIndexMut::get_mut(*self.start()..self.end() + 1, slice)
                }
            }

            #[inline]
            unsafe fn get_unchecked_mut(self, slice: #slice_mut_name<'a>) -> Self::MutOutput {
                ::soa_derive::SoAIndexMut::get_unchecked_mut(*self.start()..self.end() + 1, slice)
            }

            #[inline]
            fn index_mut(self, slice: #slice_mut_name<'a>) -> Self::MutOutput {
                ::soa_derive::SoAIndexMut::index_mut(*self.start()..self.end() + 1, slice)
            }
        }


        // RangeToInclusive<usize>
        impl<'a> ::soa_derive::SoAIndex<#slice_name<'a>> for ::std::ops::RangeToInclusive<usize> {
            type RefOutput = #slice_name<'a>;

            #[inline]
            fn get(self, slice: #slice_name<'a>) -> Option<Self::RefOutput> {
                ::soa_derive::SoAIndex::get(0..=self.end, slice)
            }

            #[inline]
            unsafe fn get_unchecked(self, slice: #slice_name<'a>) -> Self::RefOutput {
                ::soa_derive::SoAIndex::get_unchecked(0..=self.end, slice)
            }

            #[inline]
            fn index(self, slice: #slice_name<'a>) -> Self::RefOutput {
                ::soa_derive::SoAIndex::index(0..=self.end, slice)
            }
        }

        impl<'a> ::soa_derive::SoAIndexMut<#slice_mut_name<'a>> for ::std::ops::RangeToInclusive<usize> {
            type MutOutput = #slice_mut_name<'a>;

            #[inline]
            fn get_mut(self, slice: #slice_mut_name<'a>) -> Option<Self::MutOutput> {
                ::soa_derive::SoAIndexMut::get_mut(0..=self.end, slice)
            }

            #[inline]
            unsafe fn get_unchecked_mut(self, slice: #slice_mut_name<'a>) -> Self::MutOutput {
                ::soa_derive::SoAIndexMut::get_unchecked_mut(0..=self.end, slice)
            }

            #[inline]
            fn index_mut(self, slice: #slice_mut_name<'a>) -> Self::MutOutput {
                ::soa_derive::SoAIndexMut::index_mut(0..=self.end, slice)
            }
        }
    }
}
