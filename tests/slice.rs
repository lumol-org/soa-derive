#[macro_use]
extern crate soa_derive;
mod particles;

use particles::{Particle, ParticleVec};

#[test]
fn len() {
    let mut particles = ParticleVec::new();

    {
        let slice = particles.as_slice();
        assert!(slice.is_empty());
        assert_eq!(slice.len(), 0);
    }

    particles.push(Particle::new(String::from("Na"), 56.0));
    particles.push(Particle::new(String::from("Na"), 56.0));
    particles.push(Particle::new(String::from("Na"), 56.0));

    let slice = particles.as_slice();
    assert_eq!(slice.len(), 3);
}

#[test]
fn first_last() {
    let mut particles = ParticleVec::new();

    {
        let slice = particles.as_slice();
        assert_eq!(slice.first(), None);
        assert_eq!(slice.last(), None);
    }

    particles.push(Particle::new(String::from("Na"), 0.0));
    particles.push(Particle::new(String::from("Zn"), 0.0));
    particles.push(Particle::new(String::from("Cl"), 0.0));

    let slice = particles.as_slice();
    assert_eq!(slice.first().unwrap().name, "Na");
    assert_eq!(slice.last().unwrap().name, "Cl");
}

#[test]
fn split() {
    let mut particles = ParticleVec::new();
    {
        let slice = particles.as_slice();
        assert_eq!(slice.split_first(), None);
        assert_eq!(slice.split_last(), None);
    }
    particles.push(Particle::new(String::from("Cl"), 0.0));
    particles.push(Particle::new(String::from("Na"), 0.0));
    particles.push(Particle::new(String::from("Br"), 0.0));
    particles.push(Particle::new(String::from("Zn"), 0.0));

    let slice = particles.as_slice();
    let (first, end) = slice.split_first().unwrap();
    assert_eq!(first.name, "Cl");
    assert_eq!(end.len(), 3);

    let (last, start) = slice.split_last().unwrap();
    assert_eq!(last.name, "Zn");
    assert_eq!(start.len(), 3);

    let (start, end) = slice.split_at(1);
    assert_eq!(start.len(), 1);
    assert_eq!(start.name[0], "Cl");
    assert_eq!(end.len(), 3);
    assert_eq!(end.name[0], "Na");
}

#[test]
fn refs() {
    let mut particle = Particle::new(String::from("Cl"), 0.0);
    assert_eq!(particle.as_ref().name, "Cl");
    {
        let mut_ref = particle.as_mut();
        *mut_ref.mass = 42.0;
    }
    assert_eq!(particle.mass, 42.0);
}

#[test]
fn get() {
    let mut particles = ParticleVec::new();
    assert_eq!(particles.as_slice().get(0), None);

    particles.push(Particle::new(String::from("Cl"), 0.0));
    particles.push(Particle::new(String::from("Na"), 0.0));
    particles.push(Particle::new(String::from("Br"), 0.0));
    particles.push(Particle::new(String::from("Zn"), 0.0));

    assert_eq!(particles.as_slice().get(0).unwrap().name, "Cl");
    assert_eq!(particles.as_slice().get(10), None);

    unsafe {
        assert_eq!(particles.as_slice().get_unchecked(0).name, "Cl");
    }
}
