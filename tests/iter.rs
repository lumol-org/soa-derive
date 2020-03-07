mod particles;

use self::particles::{Particle, ParticleVec};

#[test]
fn iter() {
    let mut particles = ParticleVec::new();
    particles.push(Particle::new(String::from("Na"), 56.0));
    particles.push(Particle::new(String::from("Cl"), 56.0));
    particles.push(Particle::new(String::from("Zn"), 56.0));

    let mut iter = particles.iter();
    assert_eq!(iter.next().unwrap().name, "Na");
    assert_eq!(iter.next().unwrap().name, "Cl");
    assert_eq!(iter.next().unwrap().name, "Zn");

    assert!(iter.next().is_none());

    let slice = particles.as_slice();
    let mut iter = slice.iter();
    assert_eq!(iter.next().unwrap().name, "Na");
    assert_eq!(iter.next().unwrap().name, "Cl");
    assert_eq!(iter.next().unwrap().name, "Zn");

    assert!(iter.next().is_none());
}

#[test]
fn iter_mut() {
    let mut particles = ParticleVec::new();
    particles.push(Particle::new(String::from("Na"), 0.0));
    particles.push(Particle::new(String::from("Cl"), 0.0));
    particles.push(Particle::new(String::from("Zn"), 0.0));

    for particle in particles.iter_mut() {
        *particle.mass += 1.0;
    }
    assert_eq!(particles.mass[0], 1.0);
    assert_eq!(particles.mass[1], 1.0);
    assert_eq!(particles.mass[2], 1.0);

    {
        let mut slice = particles.as_mut_slice();
        for particle in slice.iter_mut() {
            *particle.mass += 1.0;
        }
    }

    assert_eq!(particles.mass[0], 2.0);
    assert_eq!(particles.mass[1], 2.0);
    assert_eq!(particles.mass[2], 2.0);
}

#[test]
fn from_iter() {
    let mut vec_with_particles = Vec::new();
    vec_with_particles.push(Particle::new(String::from("Na"), 0.0));
    vec_with_particles.push(Particle::new(String::from("Cl"), 0.0));
    vec_with_particles.push(Particle::new(String::from("Zn"), 0.0));

    let particles_from_iter: ParticleVec = vec_with_particles.into_iter().collect();

    let mut particles = ParticleVec::new();
    particles.push(Particle::new(String::from("Na"), 0.0));
    particles.push(Particle::new(String::from("Cl"), 0.0));
    particles.push(Particle::new(String::from("Zn"), 0.0));

    assert_eq!(particles, particles_from_iter)
}
