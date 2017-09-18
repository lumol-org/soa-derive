#[macro_use]
extern crate soa_derive;
mod particles;

use particles::{Particle, ParticleVec};
use particles::zip_particle::{Mass, Name};

#[test]
fn vec() {
    let mut particles = ParticleVec::new();
    particles.push(Particle::new(String::from("Na"), 56.0));
    particles.push(Particle::new(String::from("Na"), 56.0));
    particles.push(Particle::new(String::from("Na"), 56.0));
    particles.push(Particle::new(String::from("Na"), 56.0));

    for name in particles.zip(&Name) {
        assert_eq!(name, "Na");
    }

    for &mass in particles.zip(&Mass) {
        assert_eq!(mass, 56.0);
    }

    assert_eq!(particles.zip(&Name).count(), 4);
    assert_eq!(particles.zip(&Mass).count(), 4);

    for (name, &mass) in particles.zip((&Name, &Mass)) {
        assert_eq!(name, "Na");
        assert_eq!(mass, 56.0);
    }
    assert_eq!(particles.zip((&Name, &Mass)).count(), 4);


    for (_, mass) in particles.zip_mut((&Name, &mut Mass)) {
        *mass = 42.0;
    }

    for (name, _) in particles.zip_mut((&mut Name, &mut Mass)) {
        *name = "Fe".into();
    }

    for (name, &mass) in particles.zip((&Name, &Mass)) {
        assert_eq!(name, "Fe");
        assert_eq!(mass, 42.0);
    }

    // Checking that other variations also compiles
    for (_, _) in particles.zip((&Mass, &Name)) {}
    for (_, _) in particles.zip_mut((&mut Name, &Mass)) {}
    for (_, _) in particles.zip_mut((&mut Mass, &Name)) {}
    for (_, _) in particles.zip_mut((&Mass, &mut Name)) {}
}

#[test]
fn slice() {
    let mut particles = ParticleVec::new();
    particles.push(Particle::new(String::from("Na"), 56.0));
    particles.push(Particle::new(String::from("Na"), 56.0));
    particles.push(Particle::new(String::from("Na"), 56.0));
    particles.push(Particle::new(String::from("Na"), 56.0));

    let particles = particles.as_slice();

    for name in particles.zip(&Name) {
        assert_eq!(name, "Na");
    }

    for &mass in particles.zip(&Mass) {
        assert_eq!(mass, 56.0);
    }

    assert_eq!(particles.zip(&Name).count(), 4);
    assert_eq!(particles.zip(&Mass).count(), 4);

    for (name, &mass) in particles.zip((&Name, &Mass)) {
        assert_eq!(name, "Na");
        assert_eq!(mass, 56.0);
    }
    assert_eq!(particles.zip((&Name, &Mass)).count(), 4);
    for (_, _) in particles.zip((&Mass, &Name)) {}
}

#[test]
fn slice_mut() {
    let mut particles = ParticleVec::new();
    particles.push(Particle::new(String::from("Na"), 56.0));
    particles.push(Particle::new(String::from("Na"), 56.0));
    particles.push(Particle::new(String::from("Na"), 56.0));
    particles.push(Particle::new(String::from("Na"), 56.0));

    let mut particles = particles.as_mut_slice();

    for name in particles.zip(&Name) {
        assert_eq!(name, "Na");
    }

    for &mass in particles.zip(&Mass) {
        assert_eq!(mass, 56.0);
    }

    assert_eq!(particles.zip(&Name).count(), 4);
    assert_eq!(particles.zip(&Mass).count(), 4);

    for (name, &mass) in particles.zip((&Name, &Mass)) {
        assert_eq!(name, "Na");
        assert_eq!(mass, 56.0);
    }
    assert_eq!(particles.zip((&Name, &Mass)).count(), 4);


    for (_, mass) in particles.zip_mut((&Name, &mut Mass)) {
        *mass = 42.0;
    }

    for (name, _) in particles.zip_mut((&mut Name, &mut Mass)) {
        *name = "Fe".into();
    }

    for (name, &mass) in particles.zip((&Name, &Mass)) {
        assert_eq!(name, "Fe");
        assert_eq!(mass, 42.0);
    }


    // Checking that other variations also compiles
    for (_, _) in particles.zip((&Mass, &Name)) {}
    for (_, _) in particles.zip_mut((&mut Name, &Mass)) {}
    for (_, _) in particles.zip_mut((&mut Mass, &Name)) {}
    for (_, _) in particles.zip_mut((&Mass, &mut Name)) {}
}

#[test]
fn external() {
    let mut particles = ParticleVec::new();
    particles.push(Particle::new(String::from("Na"), 56.0));
    particles.push(Particle::new(String::from("Na"), 56.0));
    particles.push(Particle::new(String::from("Na"), 56.0));
    particles.push(Particle::new(String::from("Na"), 56.0));

    let bar = vec![42.0; 4];
    for (mass, &bar) in particles.zip_mut((&mut Mass, &bar)) {
        *mass = bar;
    }

    for &mass in &particles.mass {
        assert_eq!(mass, 42.0);
    }
}
