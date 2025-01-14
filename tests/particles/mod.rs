use soa_derive::{Permutation, SoATypes, StructOfArray};
use soa_derive::{SoAIndex, SoAIndexMut, SoASliceMut, SoASlice, SoAVec};

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
impl<'a> SoASlice<'a, Particle> for ParticleSlice<'a> {
    type Ref<'t>  = ParticleRef<'t> where Self: 't, 'a: 't;
    type Slice<'t> = ParticleSlice<'t> where Self: 't, 'a: 't;
    type Iter<'t> = ParticleIter<'t> where Self: 't, 'a: 't;
    type Ptr = ParticlePtr;
*/

//     fn len(&self) -> usize {
//         self.len()
//     }

//     fn is_empty(&self) -> bool {
//         self.is_empty()
//     }

//     fn as_slice<'c>(&'c self) -> Self::Slice<'c> where 'a: 'c {
//         self.reborrow::<'c>()
//     }

//     fn slice<'c>(&'c self, index: impl core::ops::RangeBounds<usize>) -> Self::Slice<'c> where 'a: 'c {
//         let start = match index.start_bound() {
//             std::ops::Bound::Included(i) | std::ops::Bound::Excluded(i) => *i,
//             std::ops::Bound::Unbounded => 0,
//         };
//         let n = self.len();
//         let end = match index.end_bound() {
//             std::ops::Bound::Included(i) => (*i + 1).min(n),
//             std::ops::Bound::Excluded(i) => *i,
//             std::ops::Bound::Unbounded => n,
//         };
//         self.index(start..end)
//     }

//     fn get<'c>(&'c self, index: usize) -> Option<Self::Ref<'c>> where 'a: 'c {
//         self.get(index)
//     }

//     fn index<'c>(&'c self, index: usize) -> Self::Ref<'c> where 'a: 'c {
//         self.index(index)
//     }

//     fn iter<'c>(&'c self) -> Self::Iter<'c> where 'a: 'c {
//         self.iter()
//     }

//     fn as_ptr(&self) -> Self::Ptr {
//         self.as_ptr()
//     }
// }

/*
impl<'a> SoASliceMut<'a, Particle> for ParticleSliceMut<'a> {
    type Ref<'t>  = ParticleRef<'t> where Self: 't, 'a: 't;
    type Slice<'t> = ParticleSlice<'t> where Self: 't, 'a: 't;
    type Iter<'t> = ParticleIter<'t> where Self: 't, 'a: 't;
    type Ptr = ParticlePtr;

    type RefMut<'t> = ParticleRefMut<'t> where 'a: 't, Self: 't;
    type SliceMut<'t> = ParticleSliceMut<'t> where 'a: 't, Self: 't;
    type IterMut<'t> = ParticleIterMut<'t> where 'a: 't, Self: 't;
    type PtrMut = ParticlePtrMut;

    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn as_slice<'c>(&'c self) -> Self::Slice<'c> where 'a: 'c {
        self.as_slice()
    }

    fn slice<'c>(&'c self, index: impl core::ops::RangeBounds<usize>) -> Self::Slice<'c> where 'a: 'c {
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

    fn get<'c>(&'c self, index: usize) -> Option<Self::Ref<'c>> where 'a: 'c {
        self.get(index)
    }

    fn index<'c>(&'c self, index: usize) -> Self::Ref<'c> where 'a: 'c {
        self.index(index)
    }

    fn iter<'c>(&'c self) -> Self::Iter<'c> where 'a: 'c {
        self.as_ref().into_iter()
    }

    fn as_mut_slice<'c>(&'c mut self) -> Self::SliceMut<'c> where 'a: 'c {
        self.reborrow()
    }

    fn slice_mut<'c>(&'c mut self, index: impl core::ops::RangeBounds<usize>) -> Self::SliceMut<'c> where 'a: 'c {
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

    fn get_mut<'c>(&'c mut self, index: usize) -> Option<Self::RefMut<'c>> where 'a: 'c {
        self.get_mut(index)
    }

    fn index_mut<'c>(&'c mut self, index: usize) -> Self::RefMut<'c> where 'a: 'c {
        self.index_mut(index)
    }

    fn iter_mut<'c>(&'c mut self) -> Self::IterMut<'c> where 'a: 'c {
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

impl<'a> SoAVec<'a, Particle> for ParticleVec {
    type Ref<'t> = ParticleRef<'t> where 'a: 't;
    type Slice<'t> = ParticleSlice<'t> where Self: 'a, 'a: 't;
    type Iter<'t> = ParticleIter<'t> where Self: 'a, 'a: 't;
    type Ptr = ParticlePtr;

    type RefMut<'t> = ParticleRefMut<'t> where Self: 'a, 'a: 't;
    type SliceMut<'t> = ParticleSliceMut<'t> where Self: 'a, 'a: 't;
    type IterMut<'t> = ParticleIterMut<'t> where Self: 'a, 'a: 't;
    type PtrMut = ParticlePtrMut;



    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn as_slice<'c>(&'c self) -> Self::Slice<'c> where 'a: 'c {
        self.as_slice()
    }

    fn slice<'c>(&'c self, index: impl core::ops::RangeBounds<usize>) -> Self::Slice<'c> where 'a: 'c {
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

    fn get<'c>(&'c self, index: usize) -> Option<Self::Ref<'c>> where 'a: 'c {
        self.get(index)
    }

    fn index<'c>(&'c self, index: usize) -> Self::Ref<'c> where 'a: 'c {
        self.index(index)
    }

    fn iter<'c>(&'c self) -> Self::Iter<'c> where 'a: 'c {
        self.iter()
    }

    fn as_mut_slice<'c>(&'c mut self) -> Self::SliceMut<'c> where 'a: 'c {
        self.as_mut_slice()
    }

    fn slice_mut<'c>(&'c mut self, index: impl core::ops::RangeBounds<usize>) -> Self::SliceMut<'c> where 'a: 'c {
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

    fn get_mut<'c>(&'c mut self, index: usize) -> Option<Self::RefMut<'c>> where 'a: 'c {
        self.get_mut(index)
    }

    fn index_mut<'c>(&'c mut self, index: usize) -> Self::RefMut<'c> where 'a: 'c {
        self.index_mut(index)
    }

    fn iter_mut<'c>(&'c mut self) -> Self::IterMut<'c> where 'a: 'c {
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


 impl<'a> SoATypes<'a>  for Particle {
    type Ptr = ParticlePtr;

    type PtrMut = ParticlePtrMut;

    type Vec<'t> = ParticleVec where 'a: 't, Self: 'a;

    type Ref<'t>  = ParticleRef<'t>  where Self: 'a, 'a: 't;

    type Iter<'t> = ParticleIter<'t> where Self: 'a, 'a: 't;

    type Slice<'t> = ParticleSlice<'t> where Self: 'a, 'a: 't;

    type RefMut<'t> = ParticleRefMut<'t> where Self: 'a, 'a: 't;

    type SliceMut<'t> = ParticleSliceMut<'t> where Self: 'a, 'a: 't;

    type IterMut<'t> =  ParticleIterMut<'t> where Self: 'a, 'a: 't;
}

fn order_concrete<'a>(vec: &mut ParticleVec) {
    let mut indices: Vec<_> = (0..vec.len()).collect();
    indices.sort_by(|j, k| {
        let a = vec.index(*j);
        let b = vec.index(*k);
        a.partial_cmp(&b).unwrap()
    });
}

fn order_generic<'a, T: StructOfArray, V: SoASlice<'a, T> + 'a>(vec: &'a V) where V::Ref<'a>: PartialOrd {
    let mut indices: Vec<_> = (0..vec.len()).collect();
    indices.sort_by(|j, k| {
        let a = vec.index(*j);
        let b = vec.index(*k);
        a.partial_cmp(&b).unwrap()
    });
}

fn iter_max_generic<'a, T: StructOfArray, V: SoASlice<'a, T> + 'a>(vec: &'a V) -> Option<V::Ref<'a>> where V::Ref<'a>: PartialOrd {
    let x= vec.iter().reduce(|a, b| {
        if a.partial_cmp(&b).unwrap().is_ge() {
            a
        } else {
            b
        }
    });
    x
}

fn iter_max_generic_iter<'a, T: SoATypes<'a>>(it: T::Iter<'a>) -> Option<T::Ref<'a>> where T::Ref<'a>: PartialOrd {
    it.reduce(|a: T::Ref<'_>, b: T::Ref<'_>| {
        if a.partial_cmp(&b).unwrap().is_ge() {
            a
        } else {
            b
        }
    })
}

fn iter_max_generic_slice<'a, V: StructOfArray, T: SoAVec<'a, V>>(vec: &'a mut T) -> Option<T::Ref<'a>> where T::Ref<'a>: PartialOrd {
    let mut indices: Vec<_> = (0..vec.len()).collect();
    indices.sort_by(|j, k| {
        let a = vec.index(*j);
        let b = vec.index(*k);
        a.partial_cmp(&b).unwrap()
    });
    let i = indices.iter().position(|x| *x == 0).unwrap();
    vec.get(i)
}

fn iter_max_concrete<'a>(vec: &'a mut ParticleVec) -> Option<<ParticleVec as SoAVec<'a, Particle>>::Ref<'a>> {
    let mut indices: Vec<_> = (0..vec.len()).collect();
    indices.sort_by(|j, k| {
        let a = <ParticleVec as SoAVec<'a, Particle>>::index(&vec, *j);
        let b = <ParticleVec as SoAVec<'a, Particle>>::index(&vec, *k);
        a.partial_cmp(&b).unwrap()
    });
    let i = indices.iter().position(|x| *x == 0).unwrap();
    vec.get(i)
}

fn sort_generic<'a: 't, 't: 'c, 'c, T: StructOfArray, V: SoAVec<'t, T>>(vec: &'a mut V) where V::Ref<'t>: PartialOrd {
    let mut indices: Vec<_> = (0..vec.len()).collect();
    indices.sort_by(|j, k| {
        let a: V::Ref<'c> = vec.index::<'c>(*j);
        let b: V::Ref<'_> = vec.index(*k);
        let r = a.partial_cmp(&b).unwrap();
        r
    });
}

#[test]
fn test_ops() {
    let mut vec = ParticleVec::new();
    vec.push(Particle::new("foo".into(), 100.0));
    vec.push(Particle::new("bar".into(), 1000.0));
    vec.push(Particle::new("baz".into(), 50.0));
    let view = vec.as_slice();
    let x = iter_max_generic(&view);
    eprintln!("{x:?}");
    let y = iter_max_generic_iter::<Particle>(vec.iter());
    eprintln!("{y:?}");
    let y = iter_max_generic_iter::<Particle>(view.iter());
    eprintln!("{y:?}");
    let y = iter_max_generic_slice::<Particle>(&view);
    eprintln!("{y:?}");
}