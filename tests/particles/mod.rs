use std::fmt::Debug;
use soa_derive::*;
use itertools::Itertools;

#[derive(Debug, Clone, PartialOrd, PartialEq, StructOfArray)]
#[soa_derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Particle {
    pub name: String,
    pub mass: f64,
}

impl Particle {
    pub fn new(name: String, mass: f64) -> Self {
        Particle {
            name,
            mass,
        }
    }
}

impl ParticleVec {
    pub fn extend(&mut self, other: &ParticleVec) {
        self.name.extend_from_slice(&other.name);
        self.mass.extend_from_slice(&other.mass);
    }
}

#[test]
fn use_iterator_tools_get() {
    let vec = ParticleVec::new();
    let particle = vec.iter().get(..0).find_or_first(|_|true);
    assert_eq!(particle, None);
}
