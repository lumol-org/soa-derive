#![allow(unexpected_cfgs)]
#![cfg(rustc_is_at_least_1_78)]

mod particles;
use std::marker::PhantomData;
use std::fmt::Debug;

use particles::ParticleVec;
use soa_derive::{SoAVec, SoASlice, StructOfArray};

use self::particles::Particle;

fn may_iter<T: StructOfArray, V: SoAVec<T>>(vec: &V) -> V::Iter<'_> {
    vec.iter()
}

fn may_push<T: StructOfArray, V: SoAVec<T>>(vec: &mut V, val: T) {
    vec.push(val)
}

fn may_sort_generic<T: StructOfArray, V: SoAVec<T>>(vec: &mut V) where for<'t> V::Ref<'t> : PartialOrd {
    let mut indices: Vec<_> = (0..vec.len()).collect();

    indices.sort_by(|j, k| {
        let a = vec.index(*j);
        let b = vec.index(*k);
        a.partial_cmp(&b).unwrap()
    });

    vec.apply_index(&indices);
}


fn may_closure_sort<V: SoAVec<Particle>, F>(vec: &mut V, mut f: F) where F: FnMut(V::Ref<'_>, V::Ref<'_>) -> std::cmp::Ordering {
    let mut indices: Vec<_> = (0..vec.len()).collect();

    indices.sort_by(|j, k| {
        let a = vec.index(*j);
        let b = vec.index(*k);
        f(a, b)
    });

    vec.apply_index(&indices);
}


#[test]
fn test_generic_type_behavior() {
    let mut x = ParticleVec::new();
    x.push(Particle::new("foo".into(), 100.0));
    let y: Vec<_> = may_iter::<Particle, ParticleVec>(&x).collect();
    assert_eq!(x.len(), y.len());
    assert_eq!(x.first().as_ref(), y.first());
    drop(y);

    let z = Particle::new("bar".into(), 1000.0);
    may_push(&mut x, z);
    assert_eq!(x.len(), 2);

    may_sort_generic(&mut x);
    assert_eq!(x.first().unwrap().name, "bar");

    x.sort_by(|a, b| a.mass.total_cmp(b.mass).reverse());
    // may_sort(&mut x);
    let a = x.index(0);
    let b = x.index(1);
    assert!(a.mass > b.mass);

    may_closure_sort(&mut x, |a, b| a.mass.total_cmp(b.mass));

    let a = x.index(0);
    let b = x.index(1);
    assert!(a.mass < b.mass);
}


#[derive(Debug, Clone)]
struct VecWrap<T: StructOfArray, V: SoAVec<T>> {
    data: V,
    marker: PhantomData<T>
}

impl<T: StructOfArray, V: SoAVec<T>> VecWrap<T, V> {
    fn new() -> Self {
        Self {
            data: V::new(),
            marker: PhantomData
        }
    }

    fn push(&mut self, value: T) {
        self.data.push(value);
    }

    fn iter(&self) -> V::Iter<'_> {
        self.data.iter()
    }

    fn view(&self) -> V::Slice<'_> {
        self.data.as_slice()
    }

    fn first(&self) -> Option<V::Ref<'_>> {
        self.data.first()
    }

    fn sort_by<F>(&mut self, f: F) where F: FnMut(V::Ref<'_>, V::Ref<'_>) -> std::cmp::Ordering {
        self.data.sort_by(f);
    }
}

#[test]
fn test_wrapped() {
    let mut this: VecWrap<Particle, ParticleVec> = VecWrap::new();
    let x = Particle::new("foo".into(), 100.0);
    this.push(x);
    let x = Particle::new("bar".into(), 1000.0);
    this.push(x);
    let x = Particle::new("baz".into(), 10.0);
    this.push(x);

    assert_eq!(this.iter().count(), 3);
    assert_eq!(this.view().len(), 3);
}

fn iter_max_generic<'a, T: StructOfArray, V: SoASlice<T> + 'a>(vec: &'a V) -> Option<V::Ref<'a>>
where
    V::Ref<'a>: PartialOrd + Debug
{
    let x= vec.iter().reduce(|a, b| {
        if a.partial_cmp(&b).unwrap().is_ge() {
            a
        } else {
            b
        }
    });
    x
}

fn iter_max_generic_iter<'a, T: StructOfArray, V: SoAVec<T>>(it: V::Iter<'a>) -> Option<V::Ref<'a>>
where
    V::Ref<'a>: PartialOrd
{
    it.reduce(|a: V::Ref<'_>, b: V::Ref<'_>| {
        if a.partial_cmp(&b).unwrap().is_ge() {
            a
        } else {
            b
        }
    })
}

fn slice_ref_len<T: StructOfArray, V: SoAVec<T>>(vec: &V) -> usize {
    let view = vec.as_slice();
    let n = view.iter().count();
    assert_eq!(view.into_iter().count(), n);
    n
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

    let mut vec = VecWrap::<Particle, ParticleVec>::new();
    vec.push(Particle::new("foo".into(), 100.0));
    vec.push(Particle::new("bar".into(), 1000.0));
    vec.push(Particle::new("baz".into(), 50.0));
    vec.sort_by(|a, b| a.mass.total_cmp(b.mass));

    assert_eq!(vec.first().unwrap().name, "baz");
}
