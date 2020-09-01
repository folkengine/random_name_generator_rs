use rand::{
    distributions::{Distribution, Standard, WeightedIndex},
    prelude::*,
    seq::SliceRandom,
    Rng,
};

use rust_embed::RustEmbed;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::string::ToString;

use crate::rng_joiner::{Joiner};
use crate::rng_syllable::{Classification, Syllable};
use crate::rng_syllables::{Syllables};

/// A Dialect is a specific collection of Syllables parsed and sorted from one of the available
/// language files.
///
///
#[derive(Clone, Debug, PartialEq)]
pub struct Dialect {
    pub name: String,
    pub prefixes: Syllables,
    pub centers: Syllables,
    pub suffixes: Syllables,
    pub bad_syllables: Vec<String>,
}

impl Dialect {
    pub fn is_valid(&self) -> bool {
        self.bad_syllables.is_empty()
    }

    pub fn new(dialect: &Dialects) -> Result<Dialect, Dialect> {
        let txt = Asset::get(dialect.get_filename().as_str()).unwrap();
        let mut prefixes: Vec<Syllable> = Vec::new();
        let mut centers: Vec<Syllable> = Vec::new();
        let mut suffixes: Vec<Syllable> = Vec::new();
        let mut bad: Vec<String> = Vec::new();

        for line in std::str::from_utf8(txt.as_ref()).unwrap().lines() {
            if let Ok(sy) = Syllable::new(line) {
                match sy.classification {
                    Classification::Prefix => prefixes.push(sy),
                    Classification::Center => centers.push(sy),
                    Classification::Suffix => suffixes.push(sy),
                }
            } else {
                bad.push(line.to_string());
            }
        }
        let d = Dialect {
            name: dialect.to_string(),
            prefixes: Syllables::new_from_vector(prefixes),
            centers: Syllables::new_from_vector(centers),
            suffixes: Syllables::new_from_vector(suffixes),
            bad_syllables: bad,
        };

        if d.bad_syllables.len() > 0 {
            Err(d)
        } else {
            Ok(d)
        }
    }

    pub fn generate_name(&self)  -> String {
        self.generate_syllables_by_count(gen_rnd_syllable_count()).collapse().clone()
    }

    pub fn generate_syllables(&self)  -> Syllables {
        self.generate_syllables_by_count(gen_rnd_syllable_count())
    }

    pub fn generate_syllables_by_count(&self, mut syllable_count: u8) -> Syllables {
        let mut syllables = Syllables::new();
        let mut last = self.prefixes.get_random().unwrap().clone();
        syllables.add(last.clone());

        while syllable_count > 2 {
            let center_syllables = self.centers.filter_from(last.jnext);
            last = center_syllables.get_random().unwrap().clone();
            syllables.add(last.clone());
            syllable_count -= 1;
        }

        let last_syllables = self.suffixes.filter_from(last.jnext);

        syllables.add(last_syllables.get_random().unwrap().clone());

        syllables
    }

    pub fn syllables(&self) -> Syllables {
        let v = [
            self.prefixes.all().clone(),
            self.centers.all().clone(),
            self.suffixes.all().clone(),
        ]
        .concat();
        Syllables::new_from_vector(v)
    }
}

// region Dialects
#[derive(Clone, Debug, PartialEq)]
pub enum Dialects {
    Demonic,
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

impl Distribution<Dialects> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Dialects {
        match rng.gen_range(1, 5) {
            // 0 => Dialects::Demonic,
            1 => Dialects::Elven,
            2 => Dialects::Fantasy,
            3 => Dialects::Goblin,
            _ => Dialects::Roman,
        }
    }
}

impl Dialects {
    pub fn get_filename(&self) -> String {
        format!("{}.txt", self.to_string())
    }

    pub fn get_path(&self) -> String {
        format!("./src/languages/{}", self.get_filename())
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

#[derive(RustEmbed)]
#[folder = "src/languages/"]
struct Asset;

#[cfg(test)]
#[allow(non_snake_case)]
mod test_dialect {
    use super::*;
    use proptest::prelude::*;
    use std::string::ToString;

    // region generate tests
    #[test]
    fn generate_name() {
        let min = Dialect {
            name: "Min".to_string(),
            prefixes: Syllables::new_from_array(&["a"]),
            centers: Syllables::new_from_array(&["b"]),
            suffixes: Syllables::new_from_array(&["c"]),
            bad_syllables: vec![],
        };

        for _ in 0..9 {
            let name = min.generate_name();
            assert!(name.as_str().starts_with("a"));
            assert!(name.as_str().ends_with("c"))
        }
    }

    #[test]
    fn generate_syllables() {
        for _ in 0..9 {
            let dialect = Dialect::new(&Dialects::Elven).unwrap();

            let syllables = dialect.generate_syllables();

            assert!(syllables.len() > 1);
            assert!(syllables.len() < 6);
        }
    }

    #[test]
    fn generate_syllables_by_count__elven_two() {
        general_generate_dialects_asserts(Dialects::Elven, 2);
    }

    #[test]
    fn generate_syllables_by_count__elven_three() {
        general_generate_dialects_asserts(Dialects::Elven, 3);
    }

    #[test]
    fn generate_syllables_by_count__elven_four() {
        general_generate_dialects_asserts(Dialects::Elven, 4);
    }

    #[test]
    fn generate_syllables_by_count__elven_five() {
        general_generate_dialects_asserts(Dialects::Elven, 5);
    }

    #[test]
    fn generate_syllables_by_count__fantasy_two() {
        general_generate_dialects_asserts(Dialects::Fantasy, 2);
    }

    #[test]
    fn generate_syllables_by_count__fantasy_three() {
        general_generate_dialects_asserts(Dialects::Fantasy, 3);
    }

    #[test]
    fn generate_syllables_by_count__fantasy_four() {
        general_generate_dialects_asserts(Dialects::Fantasy, 4);
    }

    #[test]
    fn generate_syllables_by_count__fantasy_five() {
        general_generate_dialects_asserts(Dialects::Fantasy, 5);
    }

    #[test]
    fn generate_syllables_by_count__goblin_two() {
        general_generate_dialects_asserts(Dialects::Goblin, 2);
    }

    #[test]
    fn generate_syllables_by_count__goblin_three() {
        general_generate_dialects_asserts(Dialects::Goblin, 3);
    }

    #[test]
    fn generate_syllables_by_count__goblin_four() {
        general_generate_dialects_asserts(Dialects::Goblin, 4);
    }

    #[test]
    fn generate_syllables_by_count__goblin_five() {
        general_generate_dialects_asserts(Dialects::Goblin, 5);
    }

    #[test]
    fn generate_syllables_by_count__roman_two() {
        general_generate_dialects_asserts(Dialects::Roman, 2);
    }

    #[test]
    fn generate_syllables_by_count__roman_three() {
        general_generate_dialects_asserts(Dialects::Roman, 3);
    }

    #[test]
    fn generate_syllables_by_count__roman_four() {
        general_generate_dialects_asserts(Dialects::Roman, 4);
    }

    #[test]
    fn generate_syllables_by_count__roman_five() {
        general_generate_dialects_asserts(Dialects::Roman, 5);
    }
    // endregion

    // region new tests
    #[test]
    fn new() {
        let result = Dialect::new(&Dialects::Fantasy).unwrap();

        assert_eq!(result.name, Dialects::Fantasy.to_string());
        assert!(result.bad_syllables.len() < 1);
        assert!(result.prefixes.len() > 0);
        assert!(result.centers.len() > 0);
        assert!(result.suffixes.len() > 0);
    }

    #[test]
    fn new__demonic() {
        let result = Dialect::new(&Dialects::Demonic).unwrap_err();

        assert_eq!(result.name, Dialects::Demonic.to_string());
        assert!(result.bad_syllables.len() > 0);
        assert!(result.prefixes.len() > 0);
        assert!(result.centers.len() > 0);
        assert!(result.suffixes.len() > 0);
    }

    #[test]
    fn new__goblin() {
        let result = Dialect::new(&Dialects::Goblin).unwrap();

        assert_eq!(result.name, Dialects::Goblin.to_string());
        assert!(result.bad_syllables.len() < 1);
        assert!(result.prefixes.len() > 0);
        assert!(result.centers.len() > 0);
        assert!(result.suffixes.len() > 0);
    }

    #[test]
    fn new__roman() {
        let result = Dialect::new(&Dialects::Roman).unwrap();

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
            prefixes: Syllables::new(),
            centers: Syllables::new(),
            suffixes: Syllables::new(),
            bad_syllables: vec![],
        };
        assert!(elven.is_valid())
    }

    #[test]
    fn dialect__is_valid__not() {
        let bad = Dialect {
            name: "bad".to_string(),
            prefixes: Syllables::new(),
            centers: Syllables::new(),
            suffixes: Syllables::new(),
            bad_syllables: vec!["#$@!".to_string()],
        };
        assert!(!bad.is_valid())
    }

    #[test]
    fn dialects_to_filename() {
        assert_eq!(String::from("Elven.txt"), Dialects::Elven.get_filename());
    }

    #[test]
    fn dialects_to_string() {
        assert_eq!(String::from("Elven"), Dialects::Elven.to_string());
    }

    #[test]
    fn dialect_get_path() {
        assert_eq!(
            "./src/languages/Fantasy.txt".to_string(),
            Dialects::Fantasy.get_path()
        );
    }

    proptest! {
        #[test]
        fn test_gen_rnd_syllable_count(_ in 0..100i32) {
            let count = gen_rnd_syllable_count();
            assert!((count < 6) && (count > 1), count);
        }
    }
    // endregion

    // region assert functions

    fn general_generate_dialects_asserts(dialects: Dialects, count: usize) {
        for _ in 0..9 {
            let dialect = Dialect::new(&dialects).unwrap();

            let name = dialect.generate_syllables_by_count(count as u8);

            general_generate_syllables_asserts(&dialect, &name);
            assert_eq!(name.len(), count);
        }
    }

    fn general_generate_syllables_asserts(dialect: &Dialect, syllables: &Syllables) {
        assert!(dialect.prefixes.contains(syllables.first().unwrap()));
        assert!(!dialect.centers.contains(syllables.first().unwrap()));
        assert!(!dialect.suffixes.contains(syllables.first().unwrap()));

        let count = syllables.len();
        let mut guard = 1;
        while guard < count - 1 {
            guard += 1;
            assert!(!dialect.prefixes.contains(syllables.get(guard - 1).unwrap()));
            assert!(dialect.centers.contains(syllables.get(guard - 1).unwrap()));
            assert!(!dialect.suffixes.contains(syllables.get(guard - 1).unwrap()));
        }

        assert!(!dialect.prefixes.contains(syllables.last().unwrap()));
        assert!(!dialect.centers.contains(syllables.last().unwrap()));
        assert!(dialect.suffixes.contains(syllables.last().unwrap()));
    }
    // endregion
}
