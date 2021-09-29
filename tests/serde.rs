use serde::{Deserialize, Serialize};
use soa_derive::StructOfArray;

#[derive(Debug, Clone, PartialEq, StructOfArray)]
#[soa_derive = "Debug, Clone, PartialEq, Serialize, Deserialize"]
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

#[test]
fn serde_test() -> Result<(), serde_json::Error> {
    let mut soa = ParticleVec::new();
    soa.push(Particle::new(String::from("Na"), 56.0));
    soa.push(Particle::new(String::from("Cl"), 35.0));

    let json = serde_json::to_string(&soa)?;
    assert_eq!(json, r#"{"name":["Na","Cl"],"mass":[56.0,35.0]}"#);
    let soa2: ParticleVec = serde_json::from_str(&json)?;
    assert_eq!(soa, soa2);
    Ok(())
}
