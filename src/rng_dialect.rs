use rand::prelude::*;
use rand::distributions::WeightedIndex;
use std::string::ToString;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::rng_syllable::Syllable;

#[derive(Debug, PartialEq)]
pub struct Dialect {
    name: String,
    syllables: Vec<Syllable>,
    bad_syllables: Vec<String>,
}

impl Dialect {
    pub fn is_valid(&self) -> bool {
        self.bad_syllables.len() < 1
    }

    pub fn new_from_path(path: String, name: String) -> Result<Dialect, BadDialect> {
        if let Ok(lines) = Dialect::read_lines(path) {
            let d = Dialect {
                name,
                syllables: vec![],
                bad_syllables: vec![],
            };
            Ok(d)
        } else {
            Err(BadDialect)
        }
    }

    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
        where P: AsRef<Path>, {
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
#[derive(Debug, Clone)]
#[derive(PartialEq)]
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
mod test_weight {
    use proptest::prelude::*;
    use super::*;
    use std::string::ToString;

    #[test]
    fn dialect__new_from_path() {
        let result = Dialect::new_from_path(Dialects::Fantasy.get_path(), Dialects::Fantasy.to_string()).unwrap();

        assert_eq!(result.name, Dialects::Fantasy.to_string());
    }

    #[test]
    fn dialect__new_from_path__invalid() {
        let result =
            Dialect::new_from_path("NO_THERE_THERE".to_string(), "NO_THERE_THERE".to_string());

        assert_eq!(result.unwrap_err(), BadDialect);
    }

    #[test]
    fn dialect__is_valid() {
        let elven = Dialect {
            name: "".to_string(),
            syllables: vec![],
            bad_syllables: vec![],
        };
        assert!(elven.is_valid())
    }

    #[test]
    fn dialect__is_valid__not() {
        let bad = Dialect {
            name: "bad".to_string(),
            syllables: vec![],
            bad_syllables: vec!["#$@!".to_string()]
        };
        assert!(!bad.is_valid())
    }
    
    #[test]
    fn dialect_to_string() {
        assert_eq!(String::from("Elven"), Dialects::Elven.to_string());
    }

    #[test]
    fn dialect_get_path() {
        assert_eq!("./src/languages/Fantasy.txt".to_string(), Dialects::Fantasy.get_path());
    }

    proptest! {
        #[test]
        fn test_gen_rnd_syllable_count(_ in 0..100i32) {
            let count = gen_rnd_syllable_count();
            assert!((count < 6) && (count > 1), count);
        }
    }
}