#[macro_use]
extern crate soa_derive;

#[derive(Debug, StructOfArray)]
struct Particle {
    name: String,
    mass: f64
}

impl Particle {
    pub fn new(name: String, mass: f64) -> Self {
        Particle {
            name: name,
            mass: mass,
        }
    }
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

    particles.push(Particle::new(String::from("Na"), 56.0));
    particles.push(Particle::new(String::from("Na"), 56.0));
    particles.push(Particle::new(String::from("Na"), 56.0));
    assert_eq!(particles.len(), 3);

    particles.truncate(2);
    assert_eq!(particles.len(), 2);
}

#[test]
fn capacity() {
    let mut particles = ParticleVec::with_capacity(9);
    assert_eq!(particles.len(), 0);
    assert_eq!(particles.capacity(), 9);

    particles.name.reserve_exact(30);
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
