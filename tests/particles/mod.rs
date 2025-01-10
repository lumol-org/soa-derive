use soa_derive::StructOfArray;

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
    fn my_iter(&self) -> <Particle as soa_derive::SoAIter>::Iter {
        self.iter()
    }
}
