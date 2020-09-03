use lazy_static::lazy_static;
use rand::{
    distributions::{Distribution, WeightedIndex},
    prelude::*,
};

lazy_static! {
    pub static ref NORMAL_WEIGHT: WeightedRnd = WeightedRnd {
        counts: vec![2, 3, 4, 5],
        weights: vec![4, 10, 3, 1],
    };

    pub static ref SHORT_WEIGHT: WeightedRnd = WeightedRnd {
        counts: vec![2, 3],
        weights: vec![4, 1],
    };
}

pub struct WeightedRnd {
    counts: Vec<u8>,
    weights: Vec<u8>,
}

impl WeightedRnd {
    pub fn gen(&self) -> u8 {
        let dist = WeightedIndex::new(self.weights.as_slice()).unwrap();
        let mut rng = thread_rng();
        self.counts.as_slice()[dist.sample(&mut rng)]
    }
}