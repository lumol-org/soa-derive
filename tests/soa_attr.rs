use serde::{Serialize, Deserialize};
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

#[derive(StructOfArray)]
#[soa_derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    #[soa_attr(Vec, serde(skip))]
    pub meta: bool
}

#[test]
fn serde_skip_test() -> Result<(), serde_json::Error> {
    let mut soa = PointVec::new();
    soa.push(Point { x: 1.0, y: 2.0, meta: true });
    soa.push(Point { x: 3.0, y: 4.0, meta: false });


    let json = serde_json::to_string(&soa)?;
    assert_eq!(json, r#"{"x":[1.0,3.0],"y":[2.0,4.0]}"#);
    let soa2: PointVec = serde_json::from_str(&json)?;
    assert_eq!(&soa2, &PointVec {
        x: vec![1.0, 3.0],
        y: vec![2.0, 4.0],
        meta: vec![]
    });
    Ok(())
}
