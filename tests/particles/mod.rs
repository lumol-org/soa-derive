use std::fmt::Debug;

use soa_derive::*;

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


mod impls {
    use std::cmp::Ordering;
    use std::marker::PhantomData;

    use super::*;

    fn iter_max_generic<'a, T: StructOfArray, V: SoASlice<T> + 'a>(vec: &'a V) -> Option<V::Ref<'a>> where V::Ref<'a>: PartialOrd + Debug {
        let x= vec.iter().reduce(|a, b| {
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

    fn slice_ref_len<'a, T: StructOfArray, V: SoAVec<T>>(vec: &V) -> usize {
        let view = vec.as_slice();
        let n = view.iter().count();
        assert_eq!(view.into_iter().count(), n);
        n
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
}