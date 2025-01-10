use soa_derive::prelude::*;

mod particles;
use self::particles::Particle;

fn may_i_do_this<'a, T: SoATypes<'a>>(vec: &'a T::Vec) -> T::Iter {
    let x=  vec.iter();
    x
}

#[test]
fn test_generic_type_behavior() {
    let mut x = <Particle as SoATypes>::Vec::new();
    x.push(Particle::new("foo".into(), 100.0));
    let y: Vec<_> = may_i_do_this::<Particle>(&x).collect();
    assert_eq!(x.len(), y.len());
    assert_eq!(x.get(0).as_ref(), y.get(0));
}