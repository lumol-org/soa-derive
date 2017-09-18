extern crate soa_derive_example;
use soa_derive_example::ParticleVec;
use soa_derive_example::zip_particle::{Mass, Position};

#[test]
fn vec() {
    let mut particles = ParticleVec::new();
    for _ in particles.zip(&Mass) {}
    for (_, _) in particles.zip((&Mass, &Position)) {}
    for (_, _) in particles.zip_mut((&mut Position, &Mass)) {}
    for (_, _) in particles.zip_mut((&mut Mass, &Position)) {}
    for (_, _) in particles.zip_mut((&Mass, &mut Position)) {}
}
