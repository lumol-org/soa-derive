use proc_macro2::TokenStream;
use quote::quote;

use crate::input::Input;
use crate::names;


pub fn derive_slice(input: &Input) -> TokenStream {
    let name = &input.name;
    let slice_name = names::slice_name(name);
    let ref_name = names::ref_name(&input.name);
    let ptr_name = names::ptr_name(&input.name);
    let iter_name = names::iter_name(name);
    let crate_name = &input.soa_crate;

    let generated = quote! {
        impl<'a> #crate_name::SoASlice<#name> for #slice_name<'a> {
            type Ref<'t>  = #ref_name<'t> where Self: 't, 'a: 't;
            type Slice<'t> = #slice_name<'t> where Self: 't, 'a: 't;
            type Iter<'t> = #iter_name<'t> where Self: 't, 'a: 't;
            type Ptr = #ptr_name;

            fn len(&self) -> usize {
                self.len()
            }

            fn is_empty(&self) -> bool {
                self.is_empty()
            }

            fn as_slice<'c>(&'c self) -> Self::Slice<'c> {
                self.reborrow::<'c>()
            }

            fn slice<'c, 'b: 'c>(&'c self, index: impl core::ops::RangeBounds<usize>) -> Self::Slice<'c> where Self: 'b {
                let start = match index.start_bound() {
                    std::ops::Bound::Included(i) | std::ops::Bound::Excluded(i) => *i,
                    std::ops::Bound::Unbounded => 0,
                };
                let n = self.len();
                let end = match index.end_bound() {
                    std::ops::Bound::Included(i) => (*i + 1).min(n),
                    std::ops::Bound::Excluded(i) => *i,
                    std::ops::Bound::Unbounded => n,
                };
                self.index(start..end)
            }

            fn get<'c>(&'c self, index: usize) -> Option<Self::Ref<'c>> {
                self.get(index)
            }

            fn index<'c>(&'c self, index: usize) -> Self::Ref<'c> {
                self.index(index)
            }

            fn iter<'c>(&'c self) -> Self::Iter<'c> {
                self.iter()
            }

            fn as_ptr(&self) -> Self::Ptr {
                self.as_ptr()
            }
        }
    };

    return generated
}

pub fn derive_slice_mut(input: &Input) -> TokenStream {
    let name = &input.name;
    let slice_name = names::slice_name(name);
    let slice_mut_name = names::slice_mut_name(&input.name);
    let ref_name = names::ref_name(&input.name);
    let ref_mut_name = names::ref_mut_name(&input.name);
    let ptr_name = names::ptr_name(&input.name);
    let ptr_mut_name = names::ptr_mut_name(&input.name);
    let iter_name = names::iter_name(name);
    let iter_mut_name = names::iter_mut_name(name);
    let crate_name = &input.soa_crate;

    let generated = quote! {

        impl<'a> #crate_name::SoASliceMut<#name> for #slice_mut_name<'a> {
            type Ref<'t>  = #ref_name<'t> where Self: 't;
            type Slice<'t> = #slice_name<'t> where Self: 't;
            type Iter<'t> = #iter_name<'t> where Self: 't;
            type Ptr = #ptr_name;

            type RefMut<'t> = #ref_mut_name<'t> where Self: 't;
            type SliceMut<'t> = #slice_mut_name<'t> where Self: 't;
            type IterMut<'t> = #iter_mut_name<'t> where Self: 't;
            type PtrMut = #ptr_mut_name;

            fn len(&self) -> usize {
                self.len()
            }

            fn is_empty(&self) -> bool {
                self.is_empty()
            }

            fn as_slice<'c>(&'c self) -> Self::Slice<'c> {
                self.as_slice()
            }

            fn slice<'c, 'b: 'c>(&'c self, index: impl core::ops::RangeBounds<usize>) -> Self::Slice<'c> where Self: 'b {
                let start = match index.start_bound() {
                    std::ops::Bound::Included(i) | std::ops::Bound::Excluded(i) => *i,
                    std::ops::Bound::Unbounded => 0,
                };
                let n = self.len();
                let end = match index.end_bound() {
                    std::ops::Bound::Included(i) => (*i + 1).min(n),
                    std::ops::Bound::Excluded(i) => *i,
                    std::ops::Bound::Unbounded => n,
                };
                self.index(start..end)
            }

            fn get<'c>(&'c self, index: usize) -> Option<Self::Ref<'c>> {
                self.get(index)
            }

            fn index<'c>(&'c self, index: usize) -> Self::Ref<'c> {
                self.index(index)
            }

            fn iter<'c>(&'c self) -> Self::Iter<'c> {
                self.as_ref().into_iter()
            }

            fn as_mut_slice<'c: 'b, 'b>(&'c mut self) -> Self::SliceMut<'c> where Self: 'b {
                self.reborrow()
            }

            fn slice_mut<'c>(&'c mut self, index: impl core::ops::RangeBounds<usize>) -> Self::SliceMut<'c> {
                let start = match index.start_bound() {
                    std::ops::Bound::Included(i) | std::ops::Bound::Excluded(i) => *i,
                    std::ops::Bound::Unbounded => 0,
                };
                let n = self.len();
                let end = match index.end_bound() {
                    std::ops::Bound::Included(i) => (*i + 1).min(n),
                    std::ops::Bound::Excluded(i) => *i,
                    std::ops::Bound::Unbounded => n,
                };
                self.index_mut(start..end)
            }

            fn get_mut<'c>(&'c mut self, index: usize) -> Option<Self::RefMut<'c>> {
                self.get_mut(index)
            }

            fn index_mut<'c>(&'c mut self, index: usize) -> Self::RefMut<'c> {
                self.index_mut(index)
            }

            fn iter_mut<'c>(&'c mut self) -> Self::IterMut<'c> {
                self.iter_mut()
            }

            fn apply_index(&mut self, indices: &[usize]) {
                self.apply_permutation(&mut #crate_name::Permutation::oneline(indices).inverse());
            }

            fn as_ptr(&self) -> Self::Ptr {
                self.as_ptr()
            }

            fn as_mut_ptr(&mut self) -> Self::PtrMut {
                self.as_mut_ptr()
            }
        }
    };

    return generated
}

pub fn derive_vec(input: &Input) -> TokenStream {
    let name = &input.name;
    let vec_name = names::vec_name(&input.name);
    let slice_name = names::slice_name(name);
    let slice_mut_name = names::slice_mut_name(&input.name);
    let ref_name = names::ref_name(&input.name);
    let ref_mut_name = names::ref_mut_name(&input.name);
    let ptr_name = names::ptr_name(&input.name);
    let ptr_mut_name = names::ptr_mut_name(&input.name);
    let iter_name = names::iter_name(name);
    let iter_mut_name = names::iter_mut_name(name);
    let crate_name = &input.soa_crate;

    let generated = quote! {

        impl #crate_name::SoAVec<#name> for #vec_name {
            type Ref<'t> = #ref_name<'t>;
            type Slice<'t> = #slice_name<'t>;
            type Iter<'t> = #iter_name<'t>;
            type Ptr = #ptr_name;

            type RefMut<'t> = #ref_mut_name<'t>;
            type SliceMut<'t> = #slice_mut_name<'t>;
            type IterMut<'t> = #iter_mut_name<'t>;
            type PtrMut = #ptr_mut_name;

            fn len(&self) -> usize {
                self.len()
            }

            fn is_empty(&self) -> bool {
                self.is_empty()
            }

            fn as_slice<'c, 'a: 'c>(&'c self) -> Self::Slice<'c> where Self: 'a {
                self.as_slice()
            }

            fn slice<'c, 'a: 'c>(&'c self, index: impl core::ops::RangeBounds<usize>) -> Self::Slice<'c> where Self: 'a {
                let start = match index.start_bound() {
                    std::ops::Bound::Included(i) | std::ops::Bound::Excluded(i) => *i,
                    std::ops::Bound::Unbounded => 0,
                };
                let n = self.len();
                let end = match index.end_bound() {
                    std::ops::Bound::Included(i) => (*i + 1).min(n),
                    std::ops::Bound::Excluded(i) => *i,
                    std::ops::Bound::Unbounded => n,
                };
                self.index(start..end)
            }

            fn get<'c>(&'c self, index: usize) -> Option<Self::Ref<'c>> {
                self.get(index)
            }

            fn index<'c>(&'c self, index: usize) -> Self::Ref<'c> {
                self.index(index)
            }

            fn iter<'c>(&'c self) -> Self::Iter<'c> {
                self.iter()
            }

            fn as_mut_slice<'c, 'a: 'c>(&'c mut self) -> Self::SliceMut<'c> where Self: 'a {
                self.as_mut_slice()
            }

            fn slice_mut<'c>(&'c mut self, index: impl core::ops::RangeBounds<usize>) -> Self::SliceMut<'c> {
                let start = match index.start_bound() {
                    std::ops::Bound::Included(i) | std::ops::Bound::Excluded(i) => *i,
                    std::ops::Bound::Unbounded => 0,
                };
                let n = self.len();
                let end = match index.end_bound() {
                    std::ops::Bound::Included(i) => (*i + 1).min(n),
                    std::ops::Bound::Excluded(i) => *i,
                    std::ops::Bound::Unbounded => n,
                };
                self.index_mut(start..end)
            }

            fn get_mut<'c>(&'c mut self, index: usize) -> Option<Self::RefMut<'c>> {
                self.get_mut(index)
            }

            fn index_mut<'c>(&'c mut self, index: usize) -> Self::RefMut<'c> {
                self.index_mut(index)
            }

            fn iter_mut<'c>(&'c mut self) -> Self::IterMut<'c> {
                self.iter_mut()
            }

            fn apply_index(&mut self, indices: &[usize]) {
                use #crate_name::SoASliceMut;
                self.as_mut_slice().apply_index(indices);
            }

            fn new() -> Self {
                Self::new()
            }

            fn with_capacity(capacity: usize) -> Self {
                Self::with_capacity(capacity)
            }

            fn capacity(&self) -> usize {
                self.capacity()
            }

            fn reserve(&mut self, additional: usize) {
                self.reserve(additional);
            }

            fn reserve_exact(&mut self, additional: usize) {
                self.reserve_exact(additional);
            }

            fn shrink_to_fit(&mut self) {
                self.shrink_to_fit();
            }

            fn truncate(&mut self, len: usize) {
                self.truncate(len);
            }

            fn push(&mut self, value: #name) {
                self.push(value);
            }

            fn swap_remove(&mut self, index: usize) -> #name {
                self.swap_remove(index)
            }

            fn insert(&mut self, index: usize, element: #name) {
                self.insert(index, element);
            }

            fn replace(&mut self, index: usize, element: #name) -> #name {
                self.replace(index, element)
            }

            fn remove(&mut self, index: usize) -> #name {
                self.remove(index)
            }

            fn pop(&mut self) -> Option<#name> {
                self.pop()
            }

            fn append(&mut self, other: &mut Self) {
                self.append(other);
            }

            fn clear(&mut self) {
                self.clear();
            }

            fn split_off(&mut self, at: usize) -> Self {
                self.split_off(at)
            }

            fn as_ptr(&self) -> Self::Ptr {
                self.as_ptr()
            }

            fn as_mut_ptr(&mut self) -> Self::PtrMut {
                self.as_mut_ptr()
            }
        }
    };

    return generated
}