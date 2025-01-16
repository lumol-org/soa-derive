#![allow(clippy::float_cmp)]

mod particles;
use self::particles::{Particle, ParticleVec};

#[test]
fn replace_element() {
    let mut soa = ParticleVec::new();
    soa.push(Particle::new(String::from("Na"), 22.990));
    soa.push(Particle::new(String::from("Zn"), 65.380));
    soa.push(Particle::new(String::from("Cl"), 35.453));

    let particle = soa.replace(1, Particle::new(String::from("Br"), 79.904));
    assert_eq!(particle.name, "Zn");
    assert_eq!(particle.mass, 65.380);

    assert_eq!(soa.name[1], "Br");
    assert_eq!(soa.mass[1], 79.904);
}

#[test]
fn replace_mutable_reference() {
    let mut soa = ParticleVec::new();
    soa.push(Particle::new(String::from("Na"), 22.990));
    soa.push(Particle::new(String::from("Zn"), 65.380));
    soa.push(Particle::new(String::from("Cl"), 35.453));

    let particle = soa.index_mut(1).replace(Particle::new(String::from("Br"), 79.904));
    assert_eq!(particle.name, "Zn");
    assert_eq!(particle.mass, 65.380);

    assert_eq!(soa.name[1], "Br");
    assert_eq!(soa.mass[1], 79.904);
}
