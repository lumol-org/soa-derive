#![allow(clippy::float_cmp)]

use soa_derive::soa_zip;
mod particles;
use self::particles::{Particle, ParticleVec, ParticleSlice, ParticleSliceMut};

#[test]
fn vec() {
    let mut particles = ParticleVec::new();
    particles.push(Particle::new(String::from("Na"), 56.0));
    particles.push(Particle::new(String::from("Na"), 56.0));
    particles.push(Particle::new(String::from("Na"), 56.0));
    particles.push(Particle::new(String::from("Na"), 56.0));

    for name in soa_zip!(&particles, [name]) {
        assert_eq!(name, "Na");
    }

    for &mass in soa_zip!(&particles, [mass]) {
        assert_eq!(mass, 56.0);
    }

    for (name, &mass) in soa_zip!(&particles, [name, mass]) {
        assert_eq!(name, "Na");
        assert_eq!(mass, 56.0);
    }

    for (_, mass) in soa_zip!(&mut particles, [name, mut mass]) {
        *mass = 42.0;
    }

    for (name, _) in soa_zip!(&mut particles, [mut name, mut mass]) {
        *name = "Fe".into();
    }

    for (name, &mass) in soa_zip!(&particles, [name, mass]) {
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

    for name in soa_zip!(&particles, [name]) {
        assert_eq!(name, "Na");
    }

    for &mass in soa_zip!(&particles, [mass]) {
        assert_eq!(mass, 56.0);
    }

    for (name, &mass) in soa_zip!(&particles, [name, mass]) {
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

    let mut particles = particles.as_mut_slice();

    for name in soa_zip!(&particles, [name]) {
        assert_eq!(name, "Na");
    }

    for &mass in soa_zip!(&particles, [mass]) {
        assert_eq!(mass, 56.0);
    }

    for (name, &mass) in soa_zip!(&particles, [name, mass]) {
        assert_eq!(name, "Na");
        assert_eq!(mass, 56.0);
    }

    for (_, mass) in soa_zip!(&mut particles, [name, mut mass]) {
        *mass = 42.0;
    }

    for (name, _) in soa_zip!(&mut particles, [mut name, mut mass]) {
        *name = "Fe".into();
    }

    for (name, &mass) in soa_zip!(&particles, [name, mass]) {
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
    for (mass, &bar) in soa_zip!(&mut particles, [mut mass], &bar) {
        *mass = bar;
    }

    for &mass in &particles.mass {
        assert_eq!(mass, 42.0);
    }
}

struct Wrapper {
    particles: ParticleVec
}

impl Wrapper {
    fn new(particles: ParticleVec) -> Wrapper {
        Wrapper {
            particles,
        }
    }

    fn particles(&self) -> ParticleSlice<'_> {
        self.particles.as_slice()
    }

    fn particles_mut(&mut self) -> ParticleSliceMut<'_> {
        self.particles.as_mut_slice()
    }
}

#[test]
fn access_particles_through_function() {
    let mut particles = ParticleVec::new();
    particles.push(Particle::new(String::from("Na"), 56.0));
    particles.push(Particle::new(String::from("Na"), 56.0));
    particles.push(Particle::new(String::from("Na"), 56.0));
    particles.push(Particle::new(String::from("Na"), 56.0));

    let mut wrapper = Wrapper::new(particles);

    for name in soa_zip!(wrapper.particles(), [name]) {
        assert_eq!(name, "Na");
    }

    for &mass in soa_zip!(wrapper.particles(), [mass]) {
        assert_eq!(mass, 56.0);
    }

    for (name, &mass) in soa_zip!(wrapper.particles(), [name, mass]) {
        assert_eq!(name, "Na");
        assert_eq!(mass, 56.0);
    }

    for (_, mass) in soa_zip!(wrapper.particles_mut(), [name, mut mass]) {
        *mass = 42.0;
    }

    for (name, _) in soa_zip!(wrapper.particles_mut(), [mut name, mut mass]) {
        *name = "Fe".into();
    }

    for (name, &mass) in soa_zip!(wrapper.particles(), [name, mass]) {
        assert_eq!(name, "Fe");
        assert_eq!(mass, 42.0);
    }
}
