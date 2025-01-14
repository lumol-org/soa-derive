use soa_derive::{StructOfArray, prelude::*};

#[derive(Debug, Clone, PartialEq, StructOfArray)]
pub struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug, Clone, PartialEq, StructOfArray)]
#[soa_derive(Debug, Clone, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Debug, Clone, PartialEq, StructOfArray)]
#[soa_derive(Debug, Clone, PartialEq)]
pub struct Particle {
    pub point: Point,
    #[nested_soa]
    pub color: Color,
    pub mass: f32,
}

#[test]
fn nested_soa() {
    let mut particle_vec = ParticleVec::new();
    particle_vec.push(Particle {
        point: Point { x: 1.0, y: 2.0 },
        color: Color { r: 255, g: 0, b: 0, a: 255 },
        mass: 1.0,
    });
    particle_vec.push(Particle {
        point: Point { x: 2.0, y: 3.0 },
        color: Color { r: 128, g: 255, b: 100, a: 23 },
        mass: 2.0,
    });
    assert_eq!(particle_vec.point[0], Point {
        x: 1.0, y: 2.0
    });
    assert_eq!(particle_vec.color.r[0], 255);
    assert_eq!(particle_vec.color.g[0], 0);
    assert_eq!(particle_vec.color.b[0], 0);
    assert_eq!(particle_vec.color.a[0], 255);
    assert_eq!(particle_vec.point[1], Point {
        x: 2.0, y: 3.0
    });
    assert_eq!(particle_vec.color.r[1], 128);
    assert_eq!(particle_vec.color.g[1], 255);
    assert_eq!(particle_vec.color.b[1], 100);
    assert_eq!(particle_vec.color.a[1], 23);
    assert_eq!(particle_vec.color, ColorVec {
        r: vec![255, 128],
        g: vec![0, 255],
        b: vec![0, 100],
        a: vec![255, 23],
    });
}

// fn generic_f<'a, T: SoATypes<'a>>(vec: &'a T::Vec<'a>) {
//     assert_eq!(vec.len(), 2);
// }

// #[test]
// fn test_nested_generic() {
//     let mut particle_vec = ParticleVec::new();
//     particle_vec.push(Particle {
//         point: Point { x: 1.0, y: 2.0 },
//         color: Color { r: 255, g: 0, b: 0, a: 255 },
//         mass: 1.0,
//     });
//     particle_vec.push(Particle {
//         point: Point { x: 2.0, y: 3.0 },
//         color: Color { r: 128, g: 255, b: 100, a: 23 },
//         mass: 2.0,
//     });

//     generic_f::<Particle>(&particle_vec);
// }