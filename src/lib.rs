mod rng_dialect;
mod rng_syllable;

pub fn test() {
    let elven = rng_dialect::Dialect::new(rng_dialect::Dialects::Elven).unwrap();

    for s in elven.syllables().iter() {
        println!("{}", s.to_string())
    }
}

#[cfg(test)]
mod lib_tests {
    use super::*;

}