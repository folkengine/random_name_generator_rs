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
    pub counts: Vec<u8>,
    pub weights: Vec<u8>,
}

impl WeightedRnd {
    pub fn gen(&self) -> u8 {
        let dist = WeightedIndex::new(self.weights.as_slice()).unwrap();
        let mut rng = thread_rng();
        self.counts.as_slice()[dist.sample(&mut rng)]
    }
}

#[cfg(test)]
mod test_language {
    use super::*;

    #[test]
    fn normal_weight() {
        let chain: Vec<u8> = (1..100).map(|_| NORMAL_WEIGHT.gen()).collect();
        let non: Vec<u8> = vec![0, 1, 6, 7, 8];

        chain
            .iter()
            .for_each(|i| assert!(NORMAL_WEIGHT.counts.contains(i)));
        chain.iter().for_each(|i| assert!(!non.contains(i)));
    }

    #[test]
    fn short_weight() {
        let chain: Vec<u8> = (1..100).map(|_| SHORT_WEIGHT.gen()).collect();
        let non: Vec<u8> = vec![0, 1, 4, 5, 6];

        chain
            .iter()
            .for_each(|i| assert!(SHORT_WEIGHT.counts.contains(i)));
        chain.iter().for_each(|i| assert!(!non.contains(i)));
    }
}
