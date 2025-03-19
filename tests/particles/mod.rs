use std::fmt::Debug;

use soa_derive::*;

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
