mod particles;
use self::particles::{Particle, ParticleVec, ParticleSlice, ParticleSliceMut};

#[test]
fn const_ref() {
    let particle = Particle::new(String::from("Cl"), 0.0);

    let reference = particle.as_ref();
    let ptr = reference.as_ptr();

    unsafe {
        assert_eq!(ptr.as_ref().unwrap().name, "Cl");
        assert_eq!(*ptr.as_ref().unwrap().mass, 0.0);
    }
}

#[test]
fn mut_ref() {
    let mut particle = Particle::new(String::from("Cl"), 0.0);
    let mut reference = particle.as_mut();

    let ptr = reference.as_mut_ptr();

    unsafe {
        *ptr.as_mut().unwrap().name = String::from("Fe");
        *ptr.as_mut().unwrap().mass = 42.0;
    }

    let ptr = reference.as_ptr();

    unsafe {
        assert_eq!(ptr.as_ref().unwrap().name, "Fe");
        assert_eq!(*ptr.as_ref().unwrap().mass, 42.0);
    }
}

#[test]
fn slice() {
    let mut particles = ParticleVec::new();

    particles.push(Particle::new(String::from("Na"), 1.0));
    particles.push(Particle::new(String::from("Zn"), 2.0));
    particles.push(Particle::new(String::from("Fe"), 3.0));

    let slice = particles.as_slice();
    let ptr = slice.as_ptr();

    unsafe {
        assert_eq!(ptr.as_ref().unwrap().name, "Na");
        assert_eq!(*ptr.as_ref().unwrap().mass, 1.0);
    }

    unsafe {
        let slice = ParticleSlice::from_raw_parts(ptr, 2);
        assert_eq!(slice.len(), 2);
        assert_eq!(slice.index(0).name, "Na");
        assert_eq!(slice.index(1).name, "Zn");

        assert_eq!(*slice.index(0).mass, 1.0);
        assert_eq!(*slice.index(1).mass, 2.0);
    }
}

#[test]
fn slice_mut() {
    let mut particles = ParticleVec::new();

    particles.push(Particle::new(String::from("Na"), 1.0));
    particles.push(Particle::new(String::from("Zn"), 2.0));
    particles.push(Particle::new(String::from("Fe"), 3.0));

    let mut slice = particles.as_mut_slice();
    let ptr = slice.as_mut_ptr();

    unsafe {
        *ptr.as_mut().unwrap().name = String::from("Fe");
        *ptr.as_mut().unwrap().mass = 42.0;
    }

    assert_eq!(slice.index(0).name, "Fe");
    assert_eq!(*slice.index(0).mass, 42.0);

    unsafe {
        let slice = ParticleSliceMut::from_raw_parts_mut(ptr, 2);

        for mass in slice.mass {
            *mass = -1.0;
        }
    }

    assert_eq!(*slice.index(0).mass, -1.0);
    assert_eq!(*slice.index(1).mass, -1.0);
    assert_eq!(*slice.index(2).mass, 3.0);
}

#[test]
fn vec() {
    let mut particles = ParticleVec::new();

    particles.push(Particle::new(String::from("Na"), 1.0));
    particles.push(Particle::new(String::from("Zn"), 2.0));
    particles.push(Particle::new(String::from("Fe"), 3.0));

    let len = particles.len();
    let capacity = particles.capacity();
    let ptr = particles.as_mut_ptr();

    std::mem::forget(particles);

    unsafe {
        *ptr.as_mut().unwrap().name = String::from("Fe");
        *ptr.as_mut().unwrap().mass = 42.0;
    }

    let particles = unsafe {
        ParticleVec::from_raw_parts(ptr, len, capacity)
    };

    assert_eq!(particles.len(), len);
    assert_eq!(particles.capacity(), capacity);

    assert_eq!(particles.index(0).name, "Fe");
    assert_eq!(*particles.index(0).mass, 42.0);

    assert_eq!(particles.index(1).name, "Zn");
    assert_eq!(*particles.index(1).mass, 2.0);

    assert_eq!(particles.index(2).name, "Fe");
    assert_eq!(*particles.index(2).mass, 3.0);
}
