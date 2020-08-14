mod particles;
use self::particles::{Particle, ParticleVec};
use soa_derive::StructOfArray;

#[test]
fn ty() {
    let _: <Particle as StructOfArray>::Type = ParticleVec::new();
}

#[test]
fn push() {
    let mut particles = ParticleVec::new();
    particles.push(Particle::new(String::from("Na"), 56.0));

    assert_eq!(particles.name[0], "Na");
    assert_eq!(particles.mass[0], 56.0);
}

#[test]
fn len() {
    let mut particles = ParticleVec::new();
    assert_eq!(particles.len(), 0);
    assert!(particles.is_empty());

    particles.push(Particle::new(String::from("Na"), 56.0));
    particles.push(Particle::new(String::from("Na"), 56.0));
    particles.push(Particle::new(String::from("Na"), 56.0));
    assert_eq!(particles.len(), 3);

    particles.clear();
    assert_eq!(particles.len(), 0);
}

#[test]
fn capacity() {
    let mut particles = ParticleVec::with_capacity(9);
    assert_eq!(particles.len(), 0);
    assert_eq!(particles.capacity(), 9);

    particles.reserve(42);
    assert!(particles.capacity() >= 42);

    particles.reserve_exact(100);
    assert!(particles.capacity() == 100);

    particles.push(Particle::new(String::from("Na"), 56.0));
    particles.push(Particle::new(String::from("Na"), 56.0));
    assert_eq!(particles.len(), 2);
    assert!(particles.capacity() == 100);

    particles.shrink_to_fit();
    assert_eq!(particles.len(), 2);
    assert_eq!(particles.capacity(), 2);
}

#[test]
fn remove() {
    let mut particles = ParticleVec::new();
    particles.push(Particle::new(String::from("Cl"), 0.0));
    particles.push(Particle::new(String::from("Na"), 0.0));
    particles.push(Particle::new(String::from("Br"), 0.0));
    particles.push(Particle::new(String::from("Zn"), 0.0));

    let particle = particles.remove(1);
    assert_eq!(particle.name, "Na");
    assert_eq!(particles.name[0], "Cl");
    assert_eq!(particles.name[1], "Br");
    assert_eq!(particles.name[2], "Zn");
}

#[test]
fn swap_remove() {
    let mut particles = ParticleVec::new();
    particles.push(Particle::new(String::from("Cl"), 0.0));
    particles.push(Particle::new(String::from("Na"), 0.0));
    particles.push(Particle::new(String::from("Br"), 0.0));
    particles.push(Particle::new(String::from("Zn"), 0.0));

    let particle = particles.swap_remove(1);
    assert_eq!(particle.name, "Na");
    assert_eq!(particles.name[0], "Cl");
    assert_eq!(particles.name[1], "Zn");
    assert_eq!(particles.name[2], "Br");
}

#[test]
fn insert() {
    let mut particles = ParticleVec::new();
    particles.push(Particle::new(String::from("Cl"), 0.0));
    particles.push(Particle::new(String::from("Na"), 0.0));

    particles.insert(1, Particle::new(String::from("Zn"), 0.0));
    assert_eq!(particles.name[0], "Cl");
    assert_eq!(particles.name[1], "Zn");
    assert_eq!(particles.name[2], "Na");
}

#[test]
fn pop() {
    let mut particles = ParticleVec::new();
    particles.push(Particle::new(String::from("Cl"), 0.0));
    particles.push(Particle::new(String::from("Na"), 0.0));

    let particle = particles.pop();
    assert_eq!(particle, Some(Particle::new(String::from("Na"), 0.0)));

    let particle = particles.pop();
    assert_eq!(particle, Some(Particle::new(String::from("Cl"), 0.0)));

    let particle = particles.pop();
    assert_eq!(particle, None)
}

#[test]
fn append() {
    let mut particles = ParticleVec::new();
    particles.push(Particle::new(String::from("Cl"), 0.0));
    particles.push(Particle::new(String::from("Na"), 0.0));

    let mut others = ParticleVec::new();
    others.push(Particle::new(String::from("Zn"), 0.0));
    others.push(Particle::new(String::from("Mg"), 0.0));

    particles.append(&mut others);
    assert_eq!(particles.name[0], "Cl");
    assert_eq!(particles.name[1], "Na");
    assert_eq!(particles.name[2], "Zn");
    assert_eq!(particles.name[3], "Mg");
}

#[test]
fn split_off() {
    let mut particles = ParticleVec::new();
    particles.push(Particle::new(String::from("Cl"), 0.0));
    particles.push(Particle::new(String::from("Na"), 0.0));
    particles.push(Particle::new(String::from("Zn"), 0.0));
    particles.push(Particle::new(String::from("Mg"), 0.0));

    let other = particles.split_off(2);
    assert_eq!(particles.len(), 2);
    assert_eq!(other.len(), 2);

    assert_eq!(particles.name[0], "Cl");
    assert_eq!(particles.name[1], "Na");
    assert_eq!(other.name[0], "Zn");
    assert_eq!(other.name[1], "Mg");
}

#[test]
fn retain() {
    let mut particles = ParticleVec::new();
    particles.push(Particle::new(String::from("Cl"), 0.0));
    particles.push(Particle::new(String::from("Na"), 0.0));
    particles.push(Particle::new(String::from("Zn"), 0.0));
    particles.push(Particle::new(String::from("C"), 0.0));

    particles.retain(|particle| particle.name.starts_with("C"));
    assert_eq!(particles.len(), 2);
    assert_eq!(particles.name[0], "Cl");
    assert_eq!(particles.name[1], "C");
}
