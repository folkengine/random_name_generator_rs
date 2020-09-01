#![allow(unused_imports)]

pub mod rng_dialect;
mod rng_joiner;
mod rng_syllable;
mod rng_syllables;

#[macro_use]
extern crate bitflags;
extern crate log;

pub fn generate_dialect(my_dialect: &rng_dialect::Dialects) -> rng_dialect::Dialect {
    rng_dialect::Dialect::new(my_dialect).unwrap()
}

#[cfg(test)]
mod lib_tests {
    use super::*;

    #[test]
    fn generate() {
        let dialect = generate_dialect(&rng_dialect::Dialects::Elven);

        assert_eq!(dialect.name, "Elven");
    }
}
