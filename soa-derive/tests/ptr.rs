#![allow(clippy::float_cmp)]

use std::sync::atomic::{AtomicUsize, Ordering};

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
        assert_eq!(slice.name[0], "Na");
        assert_eq!(slice.name[1], "Zn");

        assert_eq!(slice.mass[0], 1.0);
        assert_eq!(slice.mass[1], 2.0);
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

    assert_eq!(slice.name[0], "Fe");
    assert_eq!(slice.mass[0], 42.0);

    unsafe {
        let slice = ParticleSliceMut::from_raw_parts_mut(slice.as_mut_ptr(), 2);

        for mass in slice.mass {
            *mass = -1.0;
        }
    }

    assert_eq!(slice.mass[0], -1.0);
    assert_eq!(slice.mass[1], -1.0);
    assert_eq!(slice.mass[2], 3.0);
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

    assert_eq!(particles.name[0], "Fe");
    assert_eq!(particles.mass[0], 42.0);

    assert_eq!(particles.name[1], "Zn");
    assert_eq!(particles.mass[1], 2.0);

    assert_eq!(particles.name[2], "Fe");
    assert_eq!(particles.mass[2], 3.0);
}

#[derive(Clone, soa_derive::StructOfArray)]
#[soa_derive(Clone)]
struct CountOnDrop {
    data: usize,
}

static DROP_COUNTER: AtomicUsize = AtomicUsize::new(0);

impl Drop for CountOnDrop {
    fn drop(&mut self) {
        DROP_COUNTER.fetch_add(1, Ordering::SeqCst);
    }
}

#[test]
fn write() {
    {
        let mut vec = CountOnDropVec::new();
        vec.push(CountOnDrop { data: 0 });

        let ptr = vec.as_mut_ptr();
        unsafe {
            // this does not drop the already existing value in the vec
            ptr.write(CountOnDrop { data: 4 });
        }

        assert_eq!(vec.data[0], 4);
    }

    assert_eq!(DROP_COUNTER.load(Ordering::SeqCst), 1);
}
