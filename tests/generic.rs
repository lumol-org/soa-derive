// use soa_derive::prelude::*;

mod particles;
use std::marker::PhantomData;

use particles::ParticleVec;
use soa_derive::{SoAVec, StructOfArray};

use self::particles::Particle;

fn may_iter<T: StructOfArray, V: SoAVec<T>>(vec: &V) -> V::Iter<'_> {
    let x=  vec.iter();
    x
}

fn may_push<T: StructOfArray, V: SoAVec<T>>(vec: &mut V, val: T) {
    vec.push(val);
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
    assert_eq!(x.get(0).as_ref(), y.get(0));
    drop(y);

    let z = Particle::new("bar".into(), 1000.0);
    may_push(&mut x, z);
    assert_eq!(x.len(), 2);

    may_sort_generic(&mut x);
    assert_eq!(x.first().unwrap().name, "bar");

    x.sort_by(|a, b| a.mass.total_cmp(&b.mass).reverse());
    // may_sort(&mut x);
    let a = x.index(0);
    let b = x.index(1);
    assert!(a.mass > b.mass);

    may_closure_sort(&mut x, |a, b| a.mass.total_cmp(&b.mass));

    let a = x.index(0);
    let b = x.index(1);
    assert!(a.mass < b.mass);
}


#[derive(Debug, Clone)]
struct Swarm<T: StructOfArray, V: SoAVec<T>> {
    entries: V,
    _t: PhantomData<T>
}

impl<T: StructOfArray, V: SoAVec<T>> Swarm<T, V> {
    fn new() -> Self {
        Self {
            entries: V::new(),
            _t: PhantomData
        }
    }

    fn push(&mut self, value: T) {
        self.entries.push(value);
    }

    fn iter(&self) -> V::Iter<'_> {
        self.entries.iter()
    }

    fn view(&self) -> V::Slice<'_> {
        self.entries.as_slice()
    }
}

#[test]
fn test_wrapped() {
    let mut this: Swarm<Particle, ParticleVec> = Swarm::new();
    let x= Particle::new("foo".into(), 100.0);
    this.push(x);
    let x = Particle::new("bar".into(), 1000.0);
    this.push(x);
    let x = Particle::new("baz".into(), 10.0);
    this.push(x);

    assert_eq!(this.iter().count(), 3);

    assert_eq!(this.view().len(), 3);
}
