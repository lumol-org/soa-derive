use std::cmp::Ordering;
use std::fmt::Debug;
use std::marker::PhantomData;

use soa_derive::prelude::*;

#[derive(Debug, Clone, PartialOrd, PartialEq, StructOfArray)]
#[soa_derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Particle {
    pub name: String,
    pub mass: f64,
}

impl Particle {
    pub fn new(name: String, mass: f64) -> Self {
        Particle {
            name,
            mass,
        }
    }
}

/*
impl<'a> SoASlice<Particle> for ParticleSlice<'a> {
    type Ref<'t>  = ParticleRef<'t> where Self: 't, 'a: 't;
    type Slice<'t> = ParticleSlice<'t> where Self: 't, 'a: 't;
    type Iter<'t> = ParticleIter<'t> where Self: 't, 'a: 't;
    type Ptr = ParticlePtr;

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
*/

/*
impl<'a> SoASliceMut<Particle> for ParticleSliceMut<'a> {
    type Ref<'t>  = ParticleRef<'t> where Self: 't;
    type Slice<'t> = ParticleSlice<'t> where Self: 't;
    type Iter<'t> = ParticleIter<'t> where Self: 't;
    type Ptr = ParticlePtr;

    type RefMut<'t> = ParticleRefMut<'t> where Self: 't;
    type SliceMut<'t> = ParticleSliceMut<'t> where Self: 't;
    type IterMut<'t> = ParticleIterMut<'t> where Self: 't;
    type PtrMut = ParticlePtrMut;

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
        self.apply_permutation(&mut Permutation::oneline(indices).inverse());
    }

    fn as_ptr(&self) -> Self::Ptr {
        self.as_ptr()
    }

    fn as_mut_ptr(&mut self) -> Self::PtrMut {
        self.as_mut_ptr()
    }
}
*/

/*
impl SoAVec<Particle> for ParticleVec {
    type Ref<'t> = ParticleRef<'t>;
    type Slice<'t> = ParticleSlice<'t>;
    type Iter<'t> = ParticleIter<'t>;
    type Ptr = ParticlePtr;

    type RefMut<'t> = ParticleRefMut<'t>;
    type SliceMut<'t> = ParticleSliceMut<'t>;
    type IterMut<'t> = ParticleIterMut<'t>;
    type PtrMut = ParticlePtrMut;

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

    fn push(&mut self, value: Particle) {
        self.push(value);
    }

    fn swap_remove(&mut self, index: usize) -> Particle {
        self.swap_remove(index)
    }

    fn insert(&mut self, index: usize, element: Particle) {
        self.insert(index, element);
    }

    fn replace(&mut self, index: usize, element: Particle) -> Particle {
        self.replace(index, element)
    }

    fn remove(&mut self, index: usize) -> Particle {
        self.remove(index)
    }

    fn pop(&mut self) -> Option<Particle> {
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
*/

fn iter_max_generic<'a, T: StructOfArray, V: SoASlice<T> + 'a>(vec: &'a V) -> Option<V::Ref<'a>> where V::Ref<'a>: PartialOrd + Debug {
    let x= vec.iter().reduce(|a, b| {
        eprintln!("{a:?}");
        if a.partial_cmp(&b).unwrap().is_ge() {
            a
        } else {
            b
        }
    });
    x
}

fn iter_max_generic_iter<'a, T: StructOfArray, V: SoAVec<T>>(it: V::Iter<'a>) -> Option<V::Ref<'a>> where V::Ref<'a>: PartialOrd {
    it.reduce(|a: V::Ref<'_>, b: V::Ref<'_>| {
        if a.partial_cmp(&b).unwrap().is_ge() {
            a
        } else {
            b
        }
    })
}

fn iter_max_generic_slice<T: StructOfArray, S: SoASlice<T>>(vec: &S) -> Option<S::Ref<'_>> where for<'t> S::Ref<'t>: PartialOrd {
    let mut indices: Vec<_> = (0..vec.len()).collect();
    indices.sort_by(|j, k| {
        let a = vec.index(*j);
        let b = vec.index(*k);
        a.partial_cmp(&b).unwrap()
    });
    let i = indices.iter().position(|x| *x == 0).unwrap();
    vec.get(i)
}

fn slice_ref_len<'a, T: StructOfArray, V: SoAVec<T>>(vec: &V) -> usize where for<'t> <<V as SoAVec<T>>::Slice<'t> as SoASlice<T>>::Ref<'t>: PartialOrd {
    let view = vec.as_slice();
    // let _ = iter_max_generic_slice(&view);
    view.iter().count()
}


pub struct VWrap<T: StructOfArray, V: SoAVec<T>> {
    data: V,
    _t: PhantomData<T>
}

impl<T: StructOfArray, V: SoAVec<T>> VWrap<T, V> {
    pub fn empty() -> Self {
        let data = V::new();
        Self { data, _t: PhantomData }
    }

    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }

    pub fn sort_by<F>(&mut self, f: F) where F: FnMut(V::Ref<'_>, V::Ref<'_>) -> Ordering {
        self.data.sort_by(f);
    }

    pub fn first(&self) -> Option<V::Ref<'_>> {
        self.data.first()
    }
}


#[test]
fn test_ops() {
    let mut vec = ParticleVec::new();
    vec.push(Particle::new("foo".into(), 100.0));
    vec.push(Particle::new("bar".into(), 1000.0));
    vec.push(Particle::new("baz".into(), 50.0));
    // let x = iter_max_generic(&view);
    // eprintln!("{x:?}");
    let y = iter_max_generic_iter::<Particle, ParticleVec>(vec.iter());
    eprintln!("{y:?}");
    let k = slice_ref_len(&vec);
    assert_eq!(k, 3);

    let mut view = vec.as_mut_slice();
    view.iter_mut().for_each(|f| {
        *f.mass *= 2.0;
    });

    let view = vec.as_slice();
    let z = iter_max_generic(&view).unwrap();
    assert_eq!(z.name, "foo");

    let n = view.iter().count();
    assert!(n > 0);

    let mut pv = VWrap::<Particle, ParticleVec>::empty();
    pv.push(Particle::new("foo".into(), 100.0));
    pv.push(Particle::new("bar".into(), 1000.0));
    pv.push(Particle::new("baz".into(), 50.0));
    pv.sort_by(|a, b| a.mass.total_cmp(&b.mass));

    assert_eq!(pv.first().unwrap().name, "baz");

}