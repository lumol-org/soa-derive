extern crate soa_derive_example;
use soa_derive_example::ParticleVec;
use soa_derive_example::zip_particle::{Mass, X};

#[test]
fn vec() {
    let mut particles = ParticleVec::new();
    for _ in particles.zip(&Mass) {}
    for (_, _) in particles.zip((&Mass, &X)) {}
    for (_, _) in particles.zip_mut((&mut X, &Mass)) {}
    for (_, _) in particles.zip_mut((&mut Mass, &X)) {}
    for (_, _) in particles.zip_mut((&Mass, &mut X)) {}
}
