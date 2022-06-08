#![allow(clippy::needless_return)]

use soa_derive::StructOfArray;

use bencher::{Bencher, benchmark_group, benchmark_main};

#[derive(StructOfArray)]
pub struct Small {
    x: f64,
    y: f64,
    z: f64,
}

impl Small {
    fn new() -> Small {
        Small {
            x: 1.0,
            y: 0.2,
            z: -2.3,
        }
    }

    fn aos_vec(size: usize) -> Vec<Small> {
        let mut vec = Vec::new();
        for _ in 0..size {
            vec.push(Small::new())
        }
        return vec;
    }

    fn soa_vec(size: usize) -> SmallVec {
        let mut vec = SmallVec::new();
        for _ in 0..size {
            vec.push(Small::new())
        }
        return vec;
    }
}

#[derive(StructOfArray)]
pub struct Big {
    position: (f64, f64, f64),
    velocity: (f64, f64, f64),
    id: usize,
    name: String,
    userdata: String
}

impl Big {
    fn new() -> Big {
        Big {
            position: (1.0, 0.2, -2.3),
            velocity: (1.0, 0.2, -2.3),
            id: 67,
            name: "foo".into(),
            userdata: "bar".into()
        }
    }

    fn aos_vec(size: usize) -> Vec<Big> {
        let mut vec = Vec::new();
        for _ in 0..size {
            vec.push(Big::new())
        }
        return vec;
    }

    fn soa_vec(size: usize) -> BigVec {
        let mut vec = BigVec::new();
        for _ in 0..size {
            vec.push(Big::new())
        }
        return vec;
    }
}

fn aos_small_push(bencher: &mut Bencher) {
    let mut vec = Vec::new();
    bencher.iter(||{
        vec.push(Small::new())
    })
}

fn soa_small_push(bencher: &mut Bencher) {
    let mut vec = SmallVec::new();
    bencher.iter(||{
        vec.push(Small::new())
    })
}

fn aos_big_push(bencher: &mut Bencher) {
    let mut vec = Vec::new();
    bencher.iter(||{
        vec.push(Big::new())
    })
}

fn soa_big_push(bencher: &mut Bencher) {
    let mut vec = BigVec::new();
    bencher.iter(||{
        vec.push(Big::new())
    })
}

fn aos_small_do_work_10000(bencher: &mut Bencher) {
    let vec = Small::aos_vec(10000);
    bencher.iter(||{
        let mut s = 0.0;
        for v in &vec {
            s += v.x + v.y;
        }
        s
    })
}

fn soa_small_do_work_10000(bencher: &mut Bencher) {
    let vec = Small::soa_vec(10000);
    bencher.iter(||{
        let mut s = 0.0;
        for (x, y) in vec.x.iter().zip(&vec.y) {
            s += x + y;
        }
        s
    })
}

fn aos_big_do_work_1000(bencher: &mut Bencher) {
    let vec = Big::aos_vec(1000);
    bencher.iter(||{
        let mut s = 0.0;
        for v in &vec {
            s += v.position.0 + v.velocity.0 * 0.1;
        }
        s
    })
}

fn aos_big_do_work_10000(bencher: &mut Bencher) {
    let vec = Big::aos_vec(10000);
    bencher.iter(||{
        let mut s = 0.0;
        for v in &vec {
            s += v.position.0 + v.velocity.0 * 0.1;
        }
        s
    })
}

fn soa_big_do_work_1000(bencher: &mut Bencher) {
    let vec = Big::soa_vec(1000);
    bencher.iter(||{
        let mut s = 0.0;
        for (position, velocity) in vec.position.iter().zip(&vec.velocity) {
            s += position.0 + velocity.0 * 0.1;
        }
        s
    })
}

fn soa_big_do_work_10000(bencher: &mut Bencher) {
    let vec = Big::soa_vec(10000);
    bencher.iter(||{
        let mut s = 0.0;
        for (position, velocity) in vec.position.iter().zip(&vec.velocity) {
            s += position.0 + velocity.0 * 0.1;
        }
        s
    })
}


benchmark_group!(aos,
    aos_small_push, aos_big_push, aos_small_do_work_10000, aos_big_do_work_1000,
    aos_big_do_work_10000
);
benchmark_group!(soa,
    soa_small_push, soa_big_push, soa_small_do_work_10000, soa_big_do_work_1000,
    soa_big_do_work_10000
);
benchmark_main!(soa, aos);
