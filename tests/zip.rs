#[macro_use]
extern crate soa_derive;
mod particles;

use particles::{Particle, ParticleVec};

#[test]
fn vec() {
    let mut particles = ParticleVec::new();
    particles.push(Particle::new(String::from("Na"), 56.0));
    particles.push(Particle::new(String::from("Na"), 56.0));
    particles.push(Particle::new(String::from("Na"), 56.0));
    particles.push(Particle::new(String::from("Na"), 56.0));

    for name in soa_zip!(particles, [name]) {
        assert_eq!(name, "Na");
    }

    for &mass in soa_zip!(particles, [mass]) {
        assert_eq!(mass, 56.0);
    }

    for (name, &mass) in soa_zip!(particles, [name, mass]) {
        assert_eq!(name, "Na");
        assert_eq!(mass, 56.0);
    }

    for (_, mass) in soa_zip!(particles, [name, mut mass]) {
        *mass = 42.0;
    }

    for (name, _) in soa_zip!(particles, [mut name, mut mass]) {
        *name = "Fe".into();
    }

    for (name, &mass) in soa_zip!(particles, [name, mass]) {
        assert_eq!(name, "Fe");
        assert_eq!(mass, 42.0);
    }
}

#[test]
fn slice() {
    let mut particles = ParticleVec::new();
    particles.push(Particle::new(String::from("Na"), 56.0));
    particles.push(Particle::new(String::from("Na"), 56.0));
    particles.push(Particle::new(String::from("Na"), 56.0));
    particles.push(Particle::new(String::from("Na"), 56.0));

    let particles = particles.as_slice();

    for name in soa_zip!(particles, [name]) {
        assert_eq!(name, "Na");
    }

    for &mass in soa_zip!(particles, [mass]) {
        assert_eq!(mass, 56.0);
    }

    for (name, &mass) in soa_zip!(particles, [name, mass]) {
        assert_eq!(name, "Na");
        assert_eq!(mass, 56.0);
    }
}

#[test]
fn slice_mut() {
    let mut particles = ParticleVec::new();
    particles.push(Particle::new(String::from("Na"), 56.0));
    particles.push(Particle::new(String::from("Na"), 56.0));
    particles.push(Particle::new(String::from("Na"), 56.0));
    particles.push(Particle::new(String::from("Na"), 56.0));

    let particles = particles.as_mut_slice();

    for name in soa_zip!(particles, [name]) {
        assert_eq!(name, "Na");
    }

    for &mass in soa_zip!(particles, [mass]) {
        assert_eq!(mass, 56.0);
    }

    for (name, &mass) in soa_zip!(particles, [name, mass]) {
        assert_eq!(name, "Na");
        assert_eq!(mass, 56.0);
    }

    for (_, mass) in soa_zip!(particles, [name, mut mass]) {
        *mass = 42.0;
    }

    for (name, _) in soa_zip!(particles, [mut name, mut mass]) {
        *name = "Fe".into();
    }

    for (name, &mass) in soa_zip!(particles, [name, mass]) {
        assert_eq!(name, "Fe");
        assert_eq!(mass, 42.0);
    }
}

#[test]
fn external() {
    let mut particles = ParticleVec::new();
    particles.push(Particle::new(String::from("Na"), 56.0));
    particles.push(Particle::new(String::from("Na"), 56.0));
    particles.push(Particle::new(String::from("Na"), 56.0));
    particles.push(Particle::new(String::from("Na"), 56.0));

    let bar = vec![42.0; 4];
    for (mass, &bar) in soa_zip!(particles, [mut mass], &bar) {
        *mass = bar;
    }

    for &mass in &particles.mass {
        assert_eq!(mass, 42.0);
    }
}
