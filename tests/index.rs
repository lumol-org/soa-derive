mod particles;
use self::particles::{Particle, ParticleVec, ParticleRef};


#[test]
fn test_usize() {
    let mut aos = Vec::new();
    let mut soa = ParticleVec::new();

    let particle = Particle::new(String::from("Na"), 56.0);
    aos.push(particle.clone());
    soa.push(particle.clone());

    // SoaIndex
    assert_eq!(soa.get(0).unwrap().name, &aos.get(0).unwrap().name);
    assert_eq!(soa.get(0).unwrap().mass, &aos.get(0).unwrap().mass);
    assert_eq!(aos.get(1), None);
    assert_eq!(soa.get(1), None);

    unsafe {
        assert_eq!(soa.get_unchecked(0).name, &aos.get_unchecked(0).name);
        assert_eq!(soa.get_unchecked(0).mass, &aos.get_unchecked(0).mass);
    }

    assert_eq!(soa.index(0).name, &aos[0].name);
    assert_eq!(soa.index(0).mass, &aos[0].mass);


    // SoaIndexMut
    assert_eq!(soa.get_mut(0).unwrap().name, &aos.get_mut(0).unwrap().name);
    assert_eq!(soa.get_mut(0).unwrap().mass, &aos.get_mut(0).unwrap().mass);
    assert_eq!(aos.get_mut(1), None);
    assert_eq!(soa.get_mut(1), None);

    unsafe {
        assert_eq!(soa.get_unchecked_mut(0).name, &aos.get_unchecked_mut(0).name);
        assert_eq!(soa.get_unchecked_mut(0).mass, &aos.get_unchecked_mut(0).mass);
    }

    assert_eq!(soa.index_mut(0).name, &aos[0].name);
    assert_eq!(soa.index_mut(0).mass, &aos[0].mass);


    *soa.index_mut(0).mass -= 1.;
    assert_eq!(soa.get(0).map(|p| *p.mass), Some(particle.mass - 1.));

    *soa.get_mut(0).unwrap().mass += 1.;
    assert_eq!(soa.get(0).map(|p| *p.mass), Some(particle.mass));
}

fn eq_its<'a, I1, I2>(i1: I1, i2: I2)
where
    I1: Iterator<Item = ParticleRef<'a>>,
    I2: Iterator<Item = &'a Particle>,
{
    for (p1, p2) in i1.zip(i2) {
        assert_eq!(*p1.mass, p2.mass);
    }
}

#[test]
fn test_ranges() {
    let mut particles = Vec::new();
    particles.push(Particle::new(String::from("Cl"), 1.0));
    particles.push(Particle::new(String::from("Na"), 2.0));
    particles.push(Particle::new(String::from("Br"), 3.0));
    particles.push(Particle::new(String::from("Zn"), 4.0));

    let mut soa = ParticleVec::new();

    for particle in particles.iter() {
        soa.push(particle.clone());
    }

    eq_its(soa.iter(), particles.iter());

    // All tests from here are the same only changing the range

    let range = 0..1;
    eq_its(soa.get(range.clone()).unwrap().iter(), particles.get(range.clone()).unwrap().iter());
    unsafe { eq_its(soa.get_unchecked(range.clone()).iter(), particles.get_unchecked(range.clone()).iter()); }
    eq_its(soa.index(range.clone()).iter(), particles[range.clone()].iter());
    eq_its(soa.get_mut(range.clone()).unwrap().iter(), particles.get(range.clone()).unwrap().iter());
    unsafe { eq_its(soa.get_unchecked_mut(range.clone()).iter(), particles.get_unchecked(range.clone()).iter()); }
    eq_its(soa.index_mut(range.clone()).iter(), particles[range.clone()].iter());

    let range = ..3;
    eq_its(soa.get(range.clone()).unwrap().iter(), particles.get(range.clone()).unwrap().iter());
    unsafe { eq_its(soa.get_unchecked(range.clone()).iter(), particles.get_unchecked(range.clone()).iter()); }
    eq_its(soa.index(range.clone()).iter(), particles[range.clone()].iter());
    eq_its(soa.get_mut(range.clone()).unwrap().iter(), particles.get(range.clone()).unwrap().iter());
    unsafe { eq_its(soa.get_unchecked_mut(range.clone()).iter(), particles.get_unchecked(range.clone()).iter()); }
    eq_its(soa.index_mut(range.clone()).iter(), particles[range.clone()].iter());

    let range = 1..;
    eq_its(soa.get(range.clone()).unwrap().iter(), particles.get(range.clone()).unwrap().iter());
    unsafe { eq_its(soa.get_unchecked(range.clone()).iter(), particles.get_unchecked(range.clone()).iter()); }
    eq_its(soa.index(range.clone()).iter(), particles[range.clone()].iter());
    eq_its(soa.get_mut(range.clone()).unwrap().iter(), particles.get(range.clone()).unwrap().iter());
    unsafe { eq_its(soa.get_unchecked_mut(range.clone()).iter(), particles.get_unchecked(range.clone()).iter()); }
    eq_its(soa.index_mut(range.clone()).iter(), particles[range.clone()].iter());

    let range = ..;
    eq_its(soa.get(range.clone()).unwrap().iter(), particles.get(range.clone()).unwrap().iter());
    unsafe { eq_its(soa.get_unchecked(range.clone()).iter(), particles.get_unchecked(range.clone()).iter()); }
    eq_its(soa.index(range.clone()).iter(), particles[range.clone()].iter());
    eq_its(soa.get_mut(range.clone()).unwrap().iter(), particles.get(range.clone()).unwrap().iter());
    unsafe { eq_its(soa.get_unchecked_mut(range.clone()).iter(), particles.get_unchecked(range.clone()).iter()); }
    eq_its(soa.index_mut(range.clone()).iter(), particles[range.clone()].iter());

    let range = 0..=1;
    eq_its(soa.get(range.clone()).unwrap().iter(), particles.get(range.clone()).unwrap().iter());
    unsafe { eq_its(soa.get_unchecked(range.clone()).iter(), particles.get_unchecked(range.clone()).iter()); }
    eq_its(soa.index(range.clone()).iter(), particles[range.clone()].iter());
    eq_its(soa.get_mut(range.clone()).unwrap().iter(), particles.get(range.clone()).unwrap().iter());
    unsafe { eq_its(soa.get_unchecked_mut(range.clone()).iter(), particles.get_unchecked(range.clone()).iter()); }
    eq_its(soa.index_mut(range.clone()).iter(), particles[range.clone()].iter());

    let range = ..=2;
    eq_its(soa.get(range.clone()).unwrap().iter(), particles.get(range.clone()).unwrap().iter());
    unsafe { eq_its(soa.get_unchecked(range.clone()).iter(), particles.get_unchecked(range.clone()).iter()); }
    eq_its(soa.index(range.clone()).iter(), particles[range.clone()].iter());
    eq_its(soa.get_mut(range.clone()).unwrap().iter(), particles.get(range.clone()).unwrap().iter());
    unsafe { eq_its(soa.get_unchecked_mut(range.clone()).iter(), particles.get_unchecked(range.clone()).iter()); }
    eq_its(soa.index_mut(range.clone()).iter(), particles[range.clone()].iter());

}