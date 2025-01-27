use soa_derive::StructOfArray;

#[derive(Debug, Clone, PartialEq, StructOfArray)]
#[soa_attr(Vec, cfg_attr(test, derive(PartialEq, Debug)))]
pub struct Particle {
    pub name: String,
    pub mass: f64,
}

impl Particle {
    pub fn new(name: String, mass: f64) -> Self {
        Particle { name, mass }
    }
}

#[test]
fn eq_test() {
    let particles0 = ParticleVec {
        name: vec![String::from("foo"), String::from("bar")],
        mass: vec![1.0, 2.0],
    };
    let particles1 = ParticleVec {
        name: vec![String::from("foo"), String::from("bar")],
        mass: vec![1.0, 2.0],
    };
    assert_eq!(particles0, particles1);
}
