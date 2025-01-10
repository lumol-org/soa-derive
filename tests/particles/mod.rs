use soa_derive::{SoASlice, StructOfArray};

#[derive(Debug, Clone, PartialOrd, PartialEq, StructOfArray)]
#[soa_derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Particle {
    pub name: String,
    pub mass: f64,
}

// impl<'b, 'a: 'b> ::soa_derive::SoASort<'a, Particle> for ParticleSliceMut<'a>  {
//     fn sort_by<F>(&mut self, mut f: F)
//             where
//             F: FnMut(Self::Ref, Self::Ref) -> std::cmp::Ordering {
//                 use soa_derive::Permutation;

//                 let mut permutation: Vec<usize> = (0..self.len()).collect();
//                 permutation.sort_by(|j, k| {
//                     let view = self.reborrow();
//                     let xj = view.index(*j);
//                     let xk = view.index(*k);
//                     let result = f(xj, xk);
//                     drop(xj);
//                     drop(xk);
//                     result
//                 });

//                 let mut permutation = Permutation::oneline(permutation).inverse();
//                 self.apply_permutation(&mut permutation);
//             }
//     }

impl Particle {
    pub fn new(name: String, mass: f64) -> Self {
        Particle {
            name,
            mass,
        }
    }
}


