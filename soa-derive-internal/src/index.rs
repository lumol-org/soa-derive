use proc_macro2::TokenStream;
use quote::quote;

use crate::input::Input;

pub fn derive(input: &Input) -> TokenStream {
    let vec_name = &input.vec_name();
    let slice_name = &input.slice_name();
    let slice_mut_name = &input.slice_mut_name();
    let ref_name = &input.ref_name();
    let ref_mut_name = &input.ref_mut_name();
    let fields_names = input.fields.iter()
                                   .map(|field| field.ident.clone().unwrap())
                                   .collect::<Vec<_>>();
    let fields_names_1 = &fields_names;
    let fields_names_2 = &fields_names;

    quote!{
        // usize
        impl<'a> ::soa_derive::SoaIndex<&'a #vec_name> for usize {
            type RefOutput = #ref_name<'a>;

            #[inline]
            fn get(self, soa: &'a #vec_name) -> Option<Self::RefOutput> {
                if self < soa.len() {
                    Some(unsafe { self.get_unchecked(soa) })
                } else {
                    None
                }
            }

            #[inline]
            unsafe fn get_unchecked(self, soa: &'a #vec_name) -> Self::RefOutput {
                #ref_name {
                    #(#fields_names_1: soa.#fields_names_2.get_unchecked(self),)*
                }
            }

            #[inline]
            fn index(self, soa: &'a #vec_name) -> Self::RefOutput {
                #ref_name {
                    #(#fields_names_1: & soa.#fields_names_2[self],)*
                }
            }
        }

        impl<'a> ::soa_derive::SoaMutIndex<&'a mut #vec_name> for usize {
            type MutOutput = #ref_mut_name<'a>;

            #[inline]
            fn get_mut(self, soa: &'a mut #vec_name) -> Option<Self::MutOutput> {
                if self < soa.len() {
                    Some(unsafe { self.get_unchecked_mut(soa) })
                } else {
                    None
                }
            }

            #[inline]
            unsafe fn get_unchecked_mut(self, soa: &'a mut #vec_name) -> Self::MutOutput {
                #ref_mut_name {
                    #(#fields_names_1: soa.#fields_names_2.get_unchecked_mut(self),)*
                }
            }

            #[inline]
            fn index_mut(self, soa: &'a mut #vec_name) -> Self::MutOutput {
                #ref_mut_name {
                    #(#fields_names_1: &mut soa.#fields_names_2[self],)*
                }
            }
        }



        // Range<usize>
        impl<'a> ::soa_derive::SoaIndex<&'a #vec_name> for ::std::ops::Range<usize> {
            type RefOutput = #slice_name<'a>;

            #[inline]
            fn get(self, soa: &'a #vec_name) -> Option<Self::RefOutput> {
                if self.start <= self.end && self.end <= soa.len() {
                    unsafe { Some(self.get_unchecked(soa)) }
                } else {
                    None
                }
            }

            #[inline]
            unsafe fn get_unchecked(self, soa: &'a #vec_name) -> Self::RefOutput {
                #slice_name {
                    #(#fields_names_1: soa.#fields_names_2.get_unchecked(self.clone()),)*
                }
            }

            #[inline]
            fn index(self, soa: &'a #vec_name) -> Self::RefOutput {
                #slice_name {
                    #(#fields_names_1: & soa.#fields_names_2[self.clone()],)*
                }
            }
        }

        impl<'a> ::soa_derive::SoaMutIndex<&'a mut #vec_name> for ::std::ops::Range<usize> {
            type MutOutput = #slice_mut_name<'a>;

            #[inline]
            fn get_mut(self, soa: &'a mut #vec_name) -> Option<Self::MutOutput> {
                if self.start <= self.end && self.end <= soa.len() {
                    unsafe { Some(self.get_unchecked_mut(soa)) }
                } else {
                    None
                }
            }

            #[inline]
            unsafe fn get_unchecked_mut(self, soa: &'a mut #vec_name) -> Self::MutOutput {
                #slice_mut_name {
                    #(#fields_names_1: soa.#fields_names_2.get_unchecked_mut(self.clone()),)*
                }
            }

            #[inline]
            fn index_mut(self, soa: &'a mut #vec_name) -> Self::MutOutput {
                #slice_mut_name {
                    #(#fields_names_1: &mut soa.#fields_names_2[self.clone()],)*
                }
            }
        }



        // RangeTo<usize>
        impl<'a> ::soa_derive::SoaIndex<&'a #vec_name> for ::std::ops::RangeTo<usize> {
            type RefOutput = #slice_name<'a>;

            #[inline]
            fn get(self, soa: &'a #vec_name) -> Option<Self::RefOutput> {
                (0..self.end).get(soa)
            }

            #[inline]
            unsafe fn get_unchecked(self, soa: &'a #vec_name) -> Self::RefOutput {
                (0..self.end).get_unchecked(soa)
            }

            #[inline]
            fn index(self, soa: &'a #vec_name) -> Self::RefOutput {
                (0..self.end).index(soa)
            }
        }

        impl<'a> ::soa_derive::SoaMutIndex<&'a mut #vec_name> for ::std::ops::RangeTo<usize> {
            type MutOutput = #slice_mut_name<'a>;

            #[inline]
            fn get_mut(self, soa: &'a mut #vec_name) -> Option<Self::MutOutput> {
                (0..self.end).get_mut(soa)
            }

            #[inline]
            unsafe fn get_unchecked_mut(self, soa: &'a mut #vec_name) -> Self::MutOutput {
                (0..self.end).get_unchecked_mut(soa)
            }

            #[inline]
            fn index_mut(self, soa: &'a mut #vec_name) -> Self::MutOutput {
                (0..self.end).index_mut(soa)
            }
        }


        // RangeFrom<usize>
        impl<'a> ::soa_derive::SoaIndex<&'a #vec_name> for ::std::ops::RangeFrom<usize> {
            type RefOutput = #slice_name<'a>;

            #[inline]
            fn get(self, soa: &'a #vec_name) -> Option<Self::RefOutput> {
                (self.start..soa.len()).get(soa)
            }

            #[inline]
            unsafe fn get_unchecked(self, soa: &'a #vec_name) -> Self::RefOutput {
                (self.start..soa.len()).get_unchecked(soa)
            }

            #[inline]
            fn index(self, soa: &'a #vec_name) -> Self::RefOutput {
                (self.start..soa.len()).index(soa)
            }
        }

        impl<'a> ::soa_derive::SoaMutIndex<&'a mut #vec_name> for ::std::ops::RangeFrom<usize> {
            type MutOutput = #slice_mut_name<'a>;

            #[inline]
            fn get_mut(self, soa: &'a mut #vec_name) -> Option<Self::MutOutput> {
                (self.start..soa.len()).get_mut(soa)
            }

            #[inline]
            unsafe fn get_unchecked_mut(self, soa: &'a mut #vec_name) -> Self::MutOutput {
                (self.start..soa.len()).get_unchecked_mut(soa)
            }

            #[inline]
            fn index_mut(self, soa: &'a mut #vec_name) -> Self::MutOutput {
                (self.start..soa.len()).index_mut(soa)
            }
        }


        // RangeFull
        impl<'a> ::soa_derive::SoaIndex<&'a #vec_name> for ::std::ops::RangeFull {
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

        impl<'a> ::soa_derive::SoaMutIndex<&'a mut #vec_name> for ::std::ops::RangeFull {
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
        impl<'a> ::soa_derive::SoaIndex<&'a #vec_name> for ::std::ops::RangeInclusive<usize> {
            type RefOutput = #slice_name<'a>;

            #[inline]
            fn get(self, soa: &'a #vec_name) -> Option<Self::RefOutput> {
                if *self.end() == usize::MAX {
                    None
                } else {
                    (*self.start()..self.end() + 1).get(soa)
                }
            }

            #[inline]
            unsafe fn get_unchecked(self, soa: &'a #vec_name) -> Self::RefOutput {
                (*self.start()..self.end() + 1).get_unchecked(soa)
            }

            #[inline]
            fn index(self, soa: &'a #vec_name) -> Self::RefOutput {
                (*self.start()..self.end() + 1).index(soa)
            }
        }

        impl<'a> ::soa_derive::SoaMutIndex<&'a mut #vec_name> for ::std::ops::RangeInclusive<usize> {
            type MutOutput = #slice_mut_name<'a>;

            #[inline]
            fn get_mut(self, soa: &'a mut #vec_name) -> Option<Self::MutOutput> {
                if *self.end() == usize::MAX {
                    None
                } else {
                    (*self.start()..self.end() + 1).get_mut(soa)
                }
            }

            #[inline]
            unsafe fn get_unchecked_mut(self, soa: &'a mut #vec_name) -> Self::MutOutput {
                (*self.start()..self.end() + 1).get_unchecked_mut(soa)
            }

            #[inline]
            fn index_mut(self, soa: &'a mut #vec_name) -> Self::MutOutput {
                (*self.start()..self.end() + 1).index_mut(soa)
            }
        }


        // RangeToInclusive<usize>
        impl<'a> ::soa_derive::SoaIndex<&'a #vec_name> for ::std::ops::RangeToInclusive<usize> {
            type RefOutput = #slice_name<'a>;

            #[inline]
            fn get(self, soa: &'a #vec_name) -> Option<Self::RefOutput> {
                (0..=self.end).get(soa)
            }

            #[inline]
            unsafe fn get_unchecked(self, soa: &'a #vec_name) -> Self::RefOutput {
                (0..=self.end).get_unchecked(soa)
            }

            #[inline]
            fn index(self, soa: &'a #vec_name) -> Self::RefOutput {
                (0..=self.end).index(soa)
            }
        }

        impl<'a> ::soa_derive::SoaMutIndex<&'a mut #vec_name> for ::std::ops::RangeToInclusive<usize> {
            type MutOutput = #slice_mut_name<'a>;

            #[inline]
            fn get_mut(self, soa: &'a mut #vec_name) -> Option<Self::MutOutput> {
                (0..=self.end).get_mut(soa)
            }

            #[inline]
            unsafe fn get_unchecked_mut(self, soa: &'a mut #vec_name) -> Self::MutOutput {
                (0..=self.end).get_unchecked_mut(soa)
            }

            #[inline]
            fn index_mut(self, soa: &'a mut #vec_name) -> Self::MutOutput {
                (0..=self.end).index_mut(soa)
            }
        }
    }
}