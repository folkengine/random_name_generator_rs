mod rng_joiner;
mod rng_syllable;
mod rng_syllables;
mod rng_weighted_rnd;

#[macro_use]
extern crate bitflags;
extern crate log;

use anyhow::Result;
use rand::{
    distributions::{Distribution, Standard},
    prelude::*,
};
use rust_embed::RustEmbed;
use std::fmt;
use titlecase::titlecase;

use crate::rng_syllable::{Classification, Syllable};
use crate::rng_syllables::Syllables;
use crate::rng_weighted_rnd::{NORMAL_WEIGHT, SHORT_WEIGHT};

/// RNG (Random Name Generator) is a library that generates random
/// names based upon one of the available Languages.
///
/// # Usage:
/// ```
/// use rnglib::{RNG, Language};
///
/// let rng = RNG::new(&Language::Elven).unwrap();
///
/// let first_name = rng.generate_name();
/// let last_name = rng.generate_name();
///
/// println!("{}: {} {}", rng.name, first_name, last_name)
/// ```
#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Debug, PartialEq)]
pub struct RNG {
    pub name: String,
    pub prefixes: Syllables,
    pub centers: Syllables,
    pub suffixes: Syllables,
    pub bad_syllables: Vec<String>,
}

impl RNG {
    pub fn new(language: &Language) -> Result<RNG, RNG> {
        let rng = RNG::process(language);

        match rng.is_valid() {
            true => Ok(rng),
            false => Err(rng),
        }
    }

    fn process(language: &Language) -> RNG {
        let txt = Asset::get(language.get_filename().as_str()).unwrap();
        RNG::processor(std::str::from_utf8(txt.as_ref()).unwrap(), language.to_string())
    }

    fn process_file(filename: String) -> Result<RNG> {
        let f = std::fs::read_to_string(filename.clone())?;
        Ok(RNG::processor(std::str::from_utf8(f.as_ref()).unwrap(), filename))
    }

    fn processor(txt: &str, language: String) -> RNG {
        RNG::classify(
            txt,
            language,
        )
    }

    fn classify(lines: &str, name: String) -> RNG {
        let mut rng = RNG::empty(name);

        for line in lines.lines() {
            if let Ok(sy) = Syllable::new(line) {
                match sy.classification {
                    Classification::Prefix => rng.prefixes.add(sy),
                    Classification::Center => rng.centers.add(sy),
                    Classification::Suffix => rng.suffixes.add(sy),
                }
            } else {
                rng.bad_syllables.push(line.to_string());
            }
        }
        rng
    }

    pub fn empty(name: String) -> RNG {
        RNG {
            name,
            prefixes: Syllables::new(),
            centers: Syllables::new(),
            suffixes: Syllables::new(),
            bad_syllables: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.name.is_empty()
            && self.prefixes.is_empty()
            && self.centers.is_empty()
            && self.suffixes.is_empty()
            && self.bad_syllables.is_empty()
    }

    pub fn is_valid(&self) -> bool {
        !self.name.is_empty()
            && !self.prefixes.is_empty()
            && !self.centers.is_empty()
            && !self.suffixes.is_empty()
            && self.bad_syllables.is_empty()
    }

    pub fn generate(language: &Language) -> RNG {
        RNG::new(language).unwrap()
    }

    pub fn generate_name(&self) -> String {
        self.generate_name_by_count(NORMAL_WEIGHT.gen())
    }

    pub fn generate_short(&self) -> String {
        self.generate_name_by_count(SHORT_WEIGHT.gen())
    }

    pub fn generate_name_by_count(&self, count: u8) -> String {
        let name = self.generate_syllables_by_count(count).collapse();
        titlecase(name.as_str())
    }

    pub fn generate_syllables(&self) -> Syllables {
        self.generate_syllables_by_count(NORMAL_WEIGHT.gen())
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

#[derive(RustEmbed)]
#[folder = "src/languages/"]
struct Asset;

#[cfg(test)]
#[allow(non_snake_case)]
mod lib_tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn new() {
        let result = RNG::new(&Language::Fantasy).unwrap();

        assert_eq!(result.name, Language::Fantasy.to_string());
        assert!(result.bad_syllables.len() < 1);
        assert!(result.prefixes.len() > 0);
        assert!(result.centers.len() > 0);
        assert!(result.suffixes.len() > 0);
    }

    #[test]
    fn new__demonic() {
        let result = RNG::new(&Language::Demonic).unwrap_err();

        assert_eq!(result.name, Language::Demonic.to_string());
        assert!(result.bad_syllables.len() > 0);
        assert!(result.prefixes.len() > 0);
        assert!(result.centers.len() > 0);
        assert!(result.suffixes.len() > 0);
    }

    #[test]
    fn new__goblin() {
        let result = RNG::new(&Language::Goblin).unwrap();

        assert_eq!(result.name, Language::Goblin.to_string());
        assert!(result.bad_syllables.len() < 1);
        assert!(result.prefixes.len() > 0);
        assert!(result.centers.len() > 0);
        assert!(result.suffixes.len() > 0);
    }

    #[test]
    fn new__roman() {
        let result = RNG::new(&Language::Roman).unwrap();

        assert_eq!(result.name, Language::Roman.to_string());
        assert!(result.bad_syllables.len() < 1);
        assert!(result.prefixes.len() > 0);
        assert!(result.centers.len() > 0);
        assert!(result.suffixes.len() > 0);
    }

    #[test]
    fn process_file() {
        let filename = "src/languages/Test-micro.txt";

        let rng = RNG::process_file(filename.to_string());
        let result = rng.as_ref().unwrap();

        assert!(!rng.is_err());
        assert_eq!(result.name, filename.to_string());
        assert_eq!(result.bad_syllables.len(), 0);
        assert_eq!(result.prefixes.len(), 1);
        assert_eq!(result.centers.len(), 1);
        assert_eq!(result.suffixes.len(), 1);
    }

    #[test]

    #[allow(unused_variables)]
    fn process_file__with_error() {
        let filename = "src/languages/none.txt";

        let result = RNG::process_file(filename.to_string());

        assert!(result.is_err());
    }

    fn create_min() -> RNG {
        RNG {
            name: "Min".to_string(),
            prefixes: Syllables::new_from_array(&["a"]),
            centers: Syllables::new_from_array(&["b"]),
            suffixes: Syllables::new_from_array(&["c"]),
            bad_syllables: vec![],
        }
    }

    #[test]
    fn generate_name() {
        let min = create_min();

        let chain: Vec<String> = (1..10).map(|_| min.generate_name()).collect();

        chain
            .iter()
            .for_each(|name| assert!(name.as_str().starts_with("A")));
        chain
            .iter()
            .for_each(|name| assert!(name.as_str().ends_with("c")));
    }

    #[test]
    fn generate_short() {
        let min = create_min();

        let chain: Vec<String> = (1..10).map(|_| min.generate_short()).collect();

        chain
            .iter()
            .for_each(|name| assert!(name.as_str().starts_with("A")));
        chain
            .iter()
            .for_each(|name| assert!(name.as_str().ends_with("c")));
    }

    #[test]
    fn generate_name_by_count() {
        let min = create_min();

        let chain: Vec<String> = (1..10)
            .map(|_| min.generate_name_by_count(NORMAL_WEIGHT.gen()))
            .collect();

        chain
            .iter()
            .for_each(|name| assert!(name.as_str().starts_with("A")));
        chain
            .iter()
            .for_each(|name| assert!(name.as_str().ends_with("c")));
    }

    #[test]
    fn generate_syllables() {
        let rng = RNG::new(&Language::Elven).unwrap();
        let non: Vec<u8> = vec![0, 1, 6, 7, 8];

        let chain: Vec<Syllables> = (1..10).map(|_| rng.generate_syllables()).collect();

        chain
            .iter()
            .for_each(|i| assert!(NORMAL_WEIGHT.counts.contains(&(i.len() as u8))));
        chain
            .iter()
            .for_each(|i| assert!(!non.contains(&(i.len() as u8))));
    }

    #[test]
    fn is_empty() {
        assert!(RNG::empty("".to_string()).is_empty());
        assert!(!create_min().is_empty());
    }

    #[test]
    fn is_valid() {
        let min = create_min();
        assert!(min.is_valid())
    }

    #[test]
    fn is_valid__not() {
        let bad = RNG {
            name: "bad".to_string(),
            prefixes: Syllables::new(),
            centers: Syllables::new(),
            suffixes: Syllables::new(),
            bad_syllables: vec!["#$@!".to_string()],
        };
        assert!(!bad.is_valid())
    }

    #[test]
    fn generate() {
        let rng = RNG::generate(&Language::Elven);

        assert_eq!(rng.name, "Elven");
    }

    #[test]
    fn generate_syllables_by_count__elven_two() {
        general_generate_dialects_asserts(Language::Elven, 2);
    }

    #[test]
    fn generate_syllables_by_count__elven_three() {
        general_generate_dialects_asserts(Language::Elven, 3);
    }

    #[test]
    fn generate_syllables_by_count__elven_four() {
        general_generate_dialects_asserts(Language::Elven, 4);
    }

    #[test]
    fn generate_syllables_by_count__elven_five() {
        general_generate_dialects_asserts(Language::Elven, 5);
    }

    #[test]
    fn generate_syllables_by_count__fantasy_two() {
        general_generate_dialects_asserts(Language::Fantasy, 2);
    }

    #[test]
    fn generate_syllables_by_count__fantasy_three() {
        general_generate_dialects_asserts(Language::Fantasy, 3);
    }

    #[test]
    fn generate_syllables_by_count__fantasy_four() {
        general_generate_dialects_asserts(Language::Fantasy, 4);
    }

    #[test]
    fn generate_syllables_by_count__fantasy_five() {
        general_generate_dialects_asserts(Language::Fantasy, 5);
    }

    #[test]
    fn generate_syllables_by_count__goblin_two() {
        general_generate_dialects_asserts(Language::Goblin, 2);
    }

    #[test]
    fn generate_syllables_by_count__goblin_three() {
        general_generate_dialects_asserts(Language::Goblin, 3);
    }

    #[test]
    fn generate_syllables_by_count__goblin_four() {
        general_generate_dialects_asserts(Language::Goblin, 4);
    }

    #[test]
    fn generate_syllables_by_count__goblin_five() {
        general_generate_dialects_asserts(Language::Goblin, 5);
    }

    #[test]
    fn generate_syllables_by_count__roman_two() {
        general_generate_dialects_asserts(Language::Roman, 2);
    }

    #[test]
    fn generate_syllables_by_count__roman_three() {
        general_generate_dialects_asserts(Language::Roman, 3);
    }

    #[test]
    fn generate_syllables_by_count__roman_four() {
        general_generate_dialects_asserts(Language::Roman, 4);
    }

    #[test]
    fn generate_syllables_by_count__roman_five() {
        general_generate_dialects_asserts(Language::Roman, 5);
    }

    // region assert functions
    fn general_generate_dialects_asserts(language: Language, count: usize) {
        for _ in 0..9 {
            let rng = RNG::new(&language).unwrap();

            let name = rng.generate_syllables_by_count(count as u8);

            general_generate_syllables_asserts(&rng, &name);
            assert_eq!(name.len(), count);
        }
    }

    fn general_generate_syllables_asserts(rng: &RNG, syllables: &Syllables) {
        assert!(rng.prefixes.contains(syllables.first().unwrap()));
        assert!(!rng.centers.contains(syllables.first().unwrap()));
        assert!(!rng.suffixes.contains(syllables.first().unwrap()));

        let count = syllables.len();
        let mut guard = 1;
        while guard < count - 1 {
            guard += 1;
            assert!(!rng.prefixes.contains(syllables.get(guard - 1).unwrap()));
            assert!(rng.centers.contains(syllables.get(guard - 1).unwrap()));
            assert!(!rng.suffixes.contains(syllables.get(guard - 1).unwrap()));
        }

        assert!(!rng.prefixes.contains(syllables.last().unwrap()));
        assert!(!rng.centers.contains(syllables.last().unwrap()));
        assert!(rng.suffixes.contains(syllables.last().unwrap()));
    }
    // endregion

    proptest! {
        #[test]
        fn test_gen_rnd_syllable_count(_ in 0..100i32) {
            let count = NORMAL_WEIGHT.gen();
            assert!((count < 6) && (count > 1), "count of {} should be less than 6 and greater than 1", count);
        }
    }
}

// region Language

#[derive(Clone, Debug, PartialEq)]
pub enum Language {
    Curse,
    Demonic,
    Elven,
    Fantasy,
    Goblin,
    Roman,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Distribution<Language> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Language {
        match rng.gen_range(1, 5) {
            // 0 => Dialects::Demonic,
            1 => Language::Elven,
            2 => Language::Fantasy,
            3 => Language::Goblin,
            _ => Language::Roman,
        }
    }
}

impl Language {
    pub fn get_filename(&self) -> String {
        format!("{}.txt", self.to_string())
    }

    pub fn get_path(&self) -> String {
        format!("./src/languages/{}", self.get_filename())
    }
}

/// This may come in handy some day.
#[derive(Debug, Clone, PartialEq)]
pub struct BadLanguage;

impl fmt::Display for BadLanguage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid Language")
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test_language {
    use super::*;

    #[test]
    fn to_filename() {
        assert_eq!(String::from("Elven.txt"), Language::Elven.get_filename());
    }

    #[test]
    fn to_string() {
        assert_eq!(String::from("Elven"), Language::Elven.to_string());
    }

    #[test]
    fn get_path() {
        assert_eq!(
            "./src/languages/Fantasy.txt".to_string(),
            Language::Fantasy.get_path()
        );
    }
}
// endregion
