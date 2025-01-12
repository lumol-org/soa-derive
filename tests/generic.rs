use soa_derive::prelude::*;

mod particles;
use self::particles::Particle;

fn may_iter<'a: 't, 't, T: SoATypes<'a, 't>>(vec: &'a T::Vec) -> T::Iter {
    let x=  vec.iter();
    x
}

fn may_push<'a: 't, 't, T: SoATypes<'a, 't>>(vec: &'a mut T::Vec, val: T) {
    vec.push(val);
}

fn may_sort<'a: 't, 't>(vec: &mut <Particle as SoATypes<'a, 't>>::Vec) {
    let mut indices: Vec<_> = (0..vec.len()).collect();

    indices.sort_by(|j, k| {
        let a = vec.index(*j);
        let b = vec.index(*k);
        a.mass.total_cmp(b.mass).reverse()
    });

    vec.apply_index(&indices);
}


fn may_closure_sort<'a: 't, 't, F>(vec: &mut <Particle as SoATypes<'a, 't>>::Vec, mut f: F) where F: FnMut(<Particle as SoAIter>::Ref, <Particle as SoAIter>::Ref) -> std::cmp::Ordering {
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
    let mut x = <Particle as SoATypes>::Vec::new();
    x.push(Particle::new("foo".into(), 100.0));
    let y: Vec<_> = may_iter::<Particle>(&x).collect();
    assert_eq!(x.len(), y.len());
    assert_eq!(x.get(0).as_ref(), y.get(0));
    drop(y);

    let z = Particle::new("bar".into(), 1000.0);
    may_push(&mut x, z);
    assert_eq!(x.len(), 2);
    may_sort(&mut x);
    let a = x.index(0);
    let b = x.index(1);
    assert!(a.mass > b.mass);

    may_closure_sort(&mut x, |a, b| a.mass.total_cmp(&b.mass));

    let a = x.index(0);
    let b = x.index(1);
    assert!(a.mass < b.mass);
}


#[derive(Debug, Clone)]
struct Swarm<'a: 't, 't, T: soa_derive::SoATypes<'a, 't> + 'a> {
    entries: T::Vec,
}

impl<'a: 't, 't, T: soa_derive::SoATypes<'a, 't>> Swarm<'a, 't, T> {
    fn new() -> Self {
        Self {
            entries: T::Vec::new()
        }
    }

    fn push(&mut self, value: T) {
        self.entries.push(value);
    }

    fn iter(&'a self) -> T::Iter {
        self.entries.iter()
    }

    fn view(&'a self) -> T::Slice {
        self.entries.as_slice()
    }
}

#[test]
fn test_wrapped() {
    let mut this: Swarm<'_, '_, Particle> = Swarm::new();
    let x= Particle::new("foo".into(), 100.0);
    this.push(x);
    let x = Particle::new("bar".into(), 1000.0);
    this.push(x);
    let x = Particle::new("baz".into(), 10.0);
    this.push(x);

    assert_eq!(this.iter().count(), 3);

    assert_eq!(this.view().len(), 3);
}