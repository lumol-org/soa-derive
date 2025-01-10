use soa_derive::{StructOfArray, SoAArray, SoACollection};

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



// fn may_i_do_this<T: SoAArray>(vec: &T) {
//     let x = vec.get(0usize);
// }