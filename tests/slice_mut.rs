#[macro_use]
extern crate soa_derive;
mod particles;

use particles::{Particle, ParticleVec};

#[test]
fn len() {
    let mut particles = ParticleVec::new();

    {
        let sclice = particles.as_mut_slice();
        assert!(sclice.is_empty());
        assert_eq!(sclice.len(), 0);
    }

    particles.push(Particle::new(String::from("Na"), 56.0));
    particles.push(Particle::new(String::from("Na"), 56.0));
    particles.push(Particle::new(String::from("Na"), 56.0));

    let sclice = particles.as_mut_slice();
    assert_eq!(sclice.len(), 3);
}

#[test]
fn first_last() {
    let mut particles = ParticleVec::new();

    {
        let mut sclice = particles.as_mut_slice();
        assert_eq!(sclice.first_mut(), None);
        assert_eq!(sclice.last_mut(), None);
    }

    particles.push(Particle::new(String::from("Na"), 0.0));
    particles.push(Particle::new(String::from("Zn"), 0.0));
    particles.push(Particle::new(String::from("Cl"), 0.0));

    let mut sclice = particles.as_mut_slice();
    assert_eq!(sclice.first_mut().unwrap().name, "Na");
    assert_eq!(sclice.last_mut().unwrap().name, "Cl");
}

#[test]
fn split() {
    let mut particles = ParticleVec::new();
    {
        let mut sclice = particles.as_mut_slice();
        assert_eq!(sclice.split_first_mut(), None);
        assert_eq!(sclice.split_last_mut(), None);
    }
    particles.push(Particle::new(String::from("Cl"), 0.0));
    particles.push(Particle::new(String::from("Na"), 0.0));
    particles.push(Particle::new(String::from("Br"), 0.0));
    particles.push(Particle::new(String::from("Zn"), 0.0));

    let mut slice = particles.as_mut_slice();
    {
        let (first, end) = slice.split_first_mut().unwrap();
        assert_eq!(first.name, "Cl");
        assert_eq!(end.len(), 3);
    }

    {
        let (last, start) = slice.split_last_mut().unwrap();
        assert_eq!(last.name, "Zn");
        assert_eq!(start.len(), 3);
    }

    {
        let (start, end) = slice.split_at_mut(1);
        assert_eq!(start.len(), 1);
        assert_eq!(start.name[0], "Cl");
        assert_eq!(end.len(), 3);
        assert_eq!(end.name[0], "Na");
    }
}
