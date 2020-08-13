use rand::distributions::WeightedIndex;
use rand::prelude::*;
use rand::seq::SliceRandom;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::string::ToString;

use crate::rng_syllable::{Classification, Syllable};

#[derive(Clone, Debug, PartialEq)]
pub struct Dialect {
    pub name: String,
    pub prefixes: Vec<Syllable>,
    pub centers: Vec<Syllable>,
    pub suffixes: Vec<Syllable>,
    pub bad_syllables: Vec<String>,
}

impl Dialect {
    pub fn is_valid(&self) -> bool {
        self.bad_syllables.is_empty()
    }

    pub fn new(dialect: Dialects) -> Result<Dialect, BadDialect> {
        Dialect::new_from_path(dialect.get_path(), dialect.to_string())
    }

    pub fn new_from_path(path: String, name: String) -> Result<Dialect, BadDialect> {
        if let Ok(lines) = Dialect::read_lines(path) {
            let mut prefixes: Vec<Syllable> = Vec::new();
            let mut centers: Vec<Syllable> = Vec::new();
            let mut suffixes: Vec<Syllable> = Vec::new();
            let mut bad: Vec<String> = Vec::new();

            for line in lines {
                if let Ok(l) = line {
                    if let Ok(sy) = Syllable::new(l.as_str()) {
                        match sy.classification {
                            Classification::Prefix => prefixes.push(sy),
                            Classification::Center => centers.push(sy),
                            Classification::Suffix => suffixes.push(sy),
                        }
                    } else {
                        bad.push(l);
                    }
                }
            }

            let d = Dialect {
                name,
                prefixes,
                centers,
                suffixes,
                bad_syllables: bad,
            };
            Ok(d)
        } else {
            Err(BadDialect)
        }
    }

    pub fn syllables(&self) -> Vec<Syllable> {
        [
            self.prefixes.clone(),
            self.centers.clone(),
            self.suffixes.clone(),
        ]
        .concat()
    }

    fn rand_prefix(&self) -> Option<&Syllable> {
        self.prefixes.choose(&mut rand::thread_rng())
    }

    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
}

// region Dialects
#[derive(Debug, PartialEq)]
pub enum Dialects {
    Elven,
    Fantasy,
    Goblin,
    Roman,
}

impl fmt::Display for Dialects {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Dialects {
    pub fn get_path(&self) -> String {
        format!("./src/languages/{}.txt", self.to_string())
    }
}

// endregion

// region BadDialect
#[derive(Debug, Clone, PartialEq)]
pub struct BadDialect;

impl fmt::Display for BadDialect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid Dialect")
    }
}

// endregion BadDialect

// region rnd syllable count
static SYLLABLE_COUNTS: [u8; 4] = [2, 3, 4, 5];
static SYLLABLE_WEIGHTS: [u8; 4] = [4, 10, 3, 1];

fn gen_rnd_syllable_count() -> u8 {
    let dist = WeightedIndex::new(&SYLLABLE_WEIGHTS).unwrap();
    let mut rng = thread_rng();
    SYLLABLE_COUNTS[dist.sample(&mut rng)]
}
// endregion

#[cfg(test)]
#[allow(non_snake_case)]
mod test_weight {
    use super::*;
    use proptest::prelude::*;
    use std::string::ToString;

    // region old tests

    #[test]
    fn dialect__new_from_path() {
        let result =
            Dialect::new_from_path(Dialects::Fantasy.get_path(), Dialects::Fantasy.to_string())
                .unwrap();

        assert_eq!(result.name, Dialects::Fantasy.to_string());
        assert!(result.bad_syllables.len() < 1);
        assert!(result.prefixes.len() > 0);
        assert!(result.centers.len() > 0);
        assert!(result.suffixes.len() > 0);
    }

    #[test]
    fn dialect__new_from_path__invalid() {
        let result =
            Dialect::new_from_path("NO_THERE_THERE".to_string(), "NO_THERE_THERE".to_string());

        assert_eq!(result.unwrap_err(), BadDialect);
    }

    #[test]
    fn dialect__new() {
        let result = Dialect::new(Dialects::Roman).unwrap();

        assert_eq!(result.name, Dialects::Roman.to_string());
        assert!(result.bad_syllables.len() < 1);
        assert!(result.prefixes.len() > 0);
        assert!(result.centers.len() > 0);
        assert!(result.suffixes.len() > 0);
    }

    #[test]
    fn dialect__is_valid() {
        let elven = Dialect {
            name: "".to_string(),
            prefixes: vec![],
            centers: vec![],
            suffixes: vec![],
            bad_syllables: vec![],
        };
        assert!(elven.is_valid())
    }

    #[test]
    fn dialect__is_valid__not() {
        let bad = Dialect {
            name: "bad".to_string(),
            prefixes: vec![],
            centers: vec![],
            suffixes: vec![],
            bad_syllables: vec!["#$@!".to_string()],
        };
        assert!(!bad.is_valid())
    }

    #[test]
    fn dialect_to_string() {
        assert_eq!(String::from("Elven"), Dialects::Elven.to_string());
    }

    #[test]
    fn dialect_get_path() {
        assert_eq!(
            "./src/languages/Fantasy.txt".to_string(),
            Dialects::Fantasy.get_path()
        );
    }

    #[test]
    fn test_rand_prefix() {
        let result = Dialect::new(Dialects::Roman).unwrap();

        for _ in 1..100 {
            let sy = result.rand_prefix().unwrap();
            assert_eq!(Classification::Prefix, sy.classification);
        }
    }

    proptest! {
        #[test]
        fn test_gen_rnd_syllable_count(_ in 0..100i32) {
            let count = gen_rnd_syllable_count();
            assert!((count < 6) && (count > 1), count);
        }
    }
    // endregion
}
