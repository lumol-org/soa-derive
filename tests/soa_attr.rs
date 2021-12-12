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
    #[soa_attr(Slice, deprecated)]
    #[soa_attr(RefMut, deprecated)]
    pub x: f32,
    #[soa_attr(SliceMut, deprecated)]
    #[soa_attr(Ptr, deprecated)]
    pub y: f32,
    #[soa_attr(Vec, serde(skip))]
    #[soa_attr(Ref, deprecated)]
    #[soa_attr(PtrMut, deprecated)]
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

    {
        let slice = soa.as_slice();
        let _ = slice.x[0]; // Should have a deprecate warning
        let _ = slice.y[0]; 
        let _ = slice.meta[0]; 

        let ref_ = slice.get(1).unwrap();
        let _ = ref_.x; 
        let _ = ref_.y; 
        let _ = ref_.meta; // Should have a deprecate warning

        let ptr = ref_.as_ptr();
        let _ = ptr.x; 
        let _ = ptr.y; // Should have a deprecate warning
        let _ = ptr.meta; 

    }
    {
        let mut slice = soa.as_mut_slice();
        let _ = slice.x[0]; 
        let _ = slice.y[0]; // Should have a deprecate warning
        let _ = slice.meta[0]; 

        let ref_mut = slice.get_mut(1).unwrap();
        let _ = ref_mut.x; // Should have a deprecate warning
        let _ = ref_mut.y; 
        let _ = ref_mut.meta; 

        let ptr_mut = ref_mut.as_ptr();
        let _ = ptr_mut.x; 
        let _ = ptr_mut.y; // Should have a deprecate warning
        let _ = ptr_mut.meta; 
    }
    Ok(())
}
