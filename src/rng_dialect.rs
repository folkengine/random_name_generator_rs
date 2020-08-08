use rand::prelude::*;
use rand::distributions::WeightedIndex;
use crate::rng_syllable::Syllable;

pub struct Dialect {
    syllables: Vec<Syllable>,
}

static SYLLABLE_COUNTS: [u8; 4] = [2, 3, 4, 5];
static SYLLABLE_WEIGHTS: [u8; 4] = [4, 10, 3, 1];

fn gen_syllable_count() -> u8 {
    let dist = WeightedIndex::new(&SYLLABLE_WEIGHTS).unwrap();
    let mut rng = thread_rng();
    SYLLABLE_COUNTS[dist.sample(&mut rng)]
}

#[cfg(test)]
mod test_weight {
    use proptest::prelude::*;
    use super::*;

    proptest! {
        #[test]
        fn test_gen_syllable_count(_ in 0..100i32) {
            let count = gen_syllable_count();
            assert!((count < 6) && (count > 1), count);
        }
    }
}