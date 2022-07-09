mod particles;
use self::particles::{Particle, ParticleVec};

#[test]
fn len() {
    let mut particles = ParticleVec::new();

    {
        let slice = particles.as_mut_slice();
        assert!(slice.is_empty());
        assert_eq!(slice.len(), 0);
    }

    particles.push(Particle::new(String::from("Na"), 56.0));
    particles.push(Particle::new(String::from("Na"), 56.0));
    particles.push(Particle::new(String::from("Na"), 56.0));

    let slice = particles.as_mut_slice();
    assert_eq!(slice.len(), 3);
}

#[test]
fn first_last() {
    let mut particles = ParticleVec::new();

    {
        let mut slice = particles.as_mut_slice();
        assert_eq!(slice.first_mut(), None);
        assert_eq!(slice.last_mut(), None);
    }

    particles.push(Particle::new(String::from("Na"), 0.0));
    particles.push(Particle::new(String::from("Zn"), 0.0));
    particles.push(Particle::new(String::from("Cl"), 0.0));

    let mut slice = particles.as_mut_slice();
    assert_eq!(slice.first_mut().unwrap().name, "Na");
    assert_eq!(slice.last_mut().unwrap().name, "Cl");
}

#[test]
fn split() {
    let mut particles = ParticleVec::new();
    {
        let mut slice = particles.as_mut_slice();
        assert_eq!(slice.split_first_mut(), None);
        assert_eq!(slice.split_last_mut(), None);
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

#[test]
fn get() {
    let mut particles = ParticleVec::new();
    assert_eq!(particles.as_mut_slice().get(0), None);
    assert_eq!(particles.as_mut_slice().get_mut(0), None);

    particles.push(Particle::new(String::from("Cl"), 0.0));
    particles.push(Particle::new(String::from("Na"), 0.0));
    particles.push(Particle::new(String::from("Br"), 0.0));
    particles.push(Particle::new(String::from("Zn"), 0.0));

    assert_eq!(particles.as_mut_slice().get(0).unwrap().name, "Cl");
    assert_eq!(particles.as_mut_slice().get_mut(1).unwrap().name, "Na");
    assert_eq!(particles.as_mut_slice().get(10), None);
    assert_eq!(particles.as_mut_slice().get_mut(42), None);

    unsafe {
        assert_eq!(particles.as_mut_slice().get_unchecked(0).name, "Cl");
        assert_eq!(particles.as_mut_slice().get_unchecked_mut(2).name, "Br");
    }
}

#[test]
fn split_non_mut() {
    let mut particles = ParticleVec::new();
    {
        let mut_slice = particles.as_mut_slice();
        let slice = mut_slice.as_ref();
        assert_eq!(slice.split_first(), None);
        assert_eq!(slice.split_last(), None);
    }
    particles.push(Particle::new(String::from("Cl"), 0.0));
    particles.push(Particle::new(String::from("Na"), 0.0));
    particles.push(Particle::new(String::from("Br"), 0.0));
    particles.push(Particle::new(String::from("Zn"), 0.0));

    let mut_slice = particles.as_mut_slice();
    let slice = mut_slice.as_ref();

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
fn sort() {
    let mut particles = ParticleVec::new();
    particles.push(Particle::new(String::from("Na3"), 168.0));
    particles.push(Particle::new(String::from("Na"), 56.0));
    particles.push(Particle::new(String::from("Na4"), 224.0));
    particles.push(Particle::new(String::from("Na2"), 112.0));

    let mut slice = particles.as_mut_slice();

    slice.sort_by(|j, k| { j.partial_cmp(&k).unwrap() });

    let mut ordered_particles = ParticleVec::new();
    ordered_particles.push(Particle::new(String::from("Na"), 56.0));
    ordered_particles.push(Particle::new(String::from("Na2"), 112.0));
    ordered_particles.push(Particle::new(String::from("Na3"), 168.0));
    ordered_particles.push(Particle::new(String::from("Na4"), 224.0));

    assert_eq!(particles, ordered_particles);
}