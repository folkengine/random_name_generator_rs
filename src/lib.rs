#![allow(unused_imports)]

mod rng_joiner;
mod rng_dialect;
mod rng_syllable;
mod rng_syllables;

#[macro_use]
extern crate bitflags;
extern crate log;

pub fn test() {
    let elven = rng_dialect::Dialect::new(rng_dialect::Dialects::Elven).unwrap();

    for s in elven.syllables().into_iter() {
        println!("{}", s.to_string())
    }
}

// #[cfg(test)]
// mod lib_tests {
//     use super::*;
// }
