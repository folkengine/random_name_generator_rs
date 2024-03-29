#![warn(clippy::pedantic)]

mod rng_joiner;
mod rng_syllable;
mod rng_syllables;
mod rng_weighted_rnd;

#[macro_use]
extern crate bitflags;
extern crate log;

#[derive(Debug, Eq, PartialEq)]
pub enum RNGError {
    GenerationError,
    InvalidLanguageFile,
    ParsingError,
    ReadError,
}

use anyhow::Result;
use rand::{
    distributions::{Distribution, Standard},
    prelude::*,
};
use rust_embed::RustEmbed;
use std::fmt;
use std::str::FromStr;
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
/// let rng = RNG::try_from(&Language::Elven).unwrap();
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
    /// Use if you want to return the RNG entity, even if there are issues with some
    /// of the syllables. Otherwise, use `RNG::from`.
    ///
    /// # Errors
    ///
    /// Errors out if the language file is not able to be processed correctly.
    pub fn new(language: &Language) -> Result<RNG, RNG> {
        let rng = RNG::process(language);

        if rng.is_valid() {
            Ok(rng)
        } else {
            Err(rng)
        }
    }

    /// # Errors
    ///
    /// Errors out if the language file is not able to be processed correctly.
    pub fn new_from_file(filename: String) -> Result<RNG, RNGError> {
        match std::fs::read_to_string(filename.clone()) {
            Ok(f) => match std::str::from_utf8(f.as_ref()) {
                Ok(s) => Ok(RNG::classify(s, filename)),
                Err(_) => Err(RNGError::InvalidLanguageFile),
            },
            Err(_) => Err(RNGError::InvalidLanguageFile),
        }
    }

    #[must_use]
    pub fn random() -> RNG {
        let my_dialect_type: Language = rand::random();
        RNG::process(&my_dialect_type)
    }

    fn process(language: &Language) -> RNG {
        let mut txt = Asset::get(language.get_filename().as_str()).unwrap();
        RNG::classify(
            std::str::from_utf8(txt.data.to_mut()).unwrap(),
            language.to_string(),
        )
    }

    fn classify(lines: &str, name: String) -> RNG {
        let mut rng = RNG::empty(name);

        for line in lines.lines() {
            if let Ok(sy) = Syllable::from_str(line) {
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

    #[must_use]
    pub fn empty(name: String) -> RNG {
        RNG {
            name,
            prefixes: Syllables::new(),
            centers: Syllables::new(),
            suffixes: Syllables::new(),
            bad_syllables: Vec::new(),
        }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.name.is_empty()
            && self.prefixes.is_empty()
            && self.centers.is_empty()
            && self.suffixes.is_empty()
            && self.bad_syllables.is_empty()
    }

    #[must_use]
    pub fn is_valid(&self) -> bool {
        !self.name.is_empty()
            && !self.prefixes.is_empty()
            && !self.centers.is_empty()
            && !self.suffixes.is_empty()
            && self.bad_syllables.is_empty()
    }

    #[must_use]
    pub fn generate_name(&self) -> String {
        self.generate_name_by_count(NORMAL_WEIGHT.gen())
    }

    /// Returns a vector of names based on the number passed in. Returns
    /// short weighted names if `is_short` is set to true.
    #[must_use]
    pub fn generate_names(&self, number: usize, is_short: bool) -> Vec<String> {
        let mut v: Vec<String> = Vec::new();

        for _ in 0..number {
            if is_short {
                v.push(self.generate_short());
            } else {
                v.push(self.generate_name());
            }
        }

        v
    }

    #[must_use]
    pub fn generate_names_string(&self, n: usize, is_short: bool) -> String {
        self.generate_names(n, is_short).join(" ")
    }

    #[must_use]
    pub fn generate_short(&self) -> String {
        self.generate_name_by_count(SHORT_WEIGHT.gen())
    }

    #[must_use]
    pub fn generate_name_by_count(&self, count: u8) -> String {
        let name = self.generate_syllables_by_count(count).collapse();
        titlecase(name.as_str())
    }

    #[must_use]
    pub fn generate_syllables(&self) -> Syllables {
        self.generate_syllables_by_count(NORMAL_WEIGHT.gen())
    }

    /// # Panics
    ///
    /// Errors out if the language file is not able to be processed correctly.
    #[must_use]
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

    #[must_use]
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

impl From<&Language> for RNG {
    fn from(language: &Language) -> Self {
        RNG::process(language)
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
    fn try_from() {
        let rng = RNG::try_from(&Language::Fantasy).unwrap();

        assert_eq!(rng.name, Language::Fantasy.to_string());
        assert!(rng.bad_syllables.len() < 1);
        assert!(rng.prefixes.len() > 0);
        assert!(rng.centers.len() > 0);
        assert!(rng.suffixes.len() > 0);
    }

    #[test]
    fn try_from__demonic() {
        let rng = RNG::new(&Language::Demonic).unwrap();

        assert!(rng.bad_syllables.len() < 1);
        assert!(rng.prefixes.len() > 0);
        assert!(rng.centers.len() > 0);
        assert!(rng.suffixes.len() > 0);
    }

    #[test]
    fn try_from__goblin() {
        let result = RNG::try_from(&Language::Goblin).unwrap();

        assert_eq!(result.name, Language::Goblin.to_string());
        assert!(result.bad_syllables.len() < 1);
        assert!(result.prefixes.len() > 0);
        assert!(result.centers.len() > 0);
        assert!(result.suffixes.len() > 0);
    }

    #[test]
    fn try_from__roman() {
        let result = RNG::try_from(&Language::Roman).unwrap();

        assert_eq!(result.name, Language::Roman.to_string());
        assert!(result.bad_syllables.len() < 1);
        assert!(result.prefixes.len() > 0);
        assert!(result.centers.len() > 0);
        assert!(result.suffixes.len() > 0);
    }

    #[test]
    fn try_from__fantasy_russian() {
        let result = RNG::try_from(&Language::Фантазия).unwrap();

        assert_eq!(result.name, Language::Фантазия.to_string());
        assert!(result.bad_syllables.len() < 1);
        assert!(result.prefixes.len() > 0);
        assert!(result.centers.len() > 0);
        assert!(result.suffixes.len() > 0);
    }

    #[test]
    fn new_from_file() {
        let filename = "src/languages/Test-micro.txt";

        let rng = RNG::new_from_file(filename.to_string());
        let result = rng.as_ref().unwrap();

        assert!(!rng.is_err());
        assert_eq!(result.name, filename.to_string());
        assert_eq!(result.bad_syllables.len(), 0);
        assert_eq!(result.prefixes.len(), 1);
        assert_eq!(result.centers.len(), 1);
        assert_eq!(result.suffixes.len(), 1);
    }

    #[test]
    fn new_from_file__russian_goblin() {
        let filename = "src/languages/Гоблин.txt";

        let rng = RNG::new_from_file(filename.to_string());
        let result = rng.as_ref().unwrap();

        assert!(!rng.is_err());
        assert_eq!(result.name, filename.to_string());
        assert_eq!(result.bad_syllables.len(), 0);
        assert_eq!(result.prefixes.len(), 19);
        assert_eq!(result.centers.len(), 13);
        assert_eq!(result.suffixes.len(), 16);
    }

    #[test]
    fn new_from_file__with_error() {
        let filename = "src/languages/none.txt";

        let result = RNG::new_from_file(filename.to_string());

        assert!(result.is_err());
    }

    #[test]
    fn new_from_file__russian_fantasy() {
        let filename = "src/languages/Фантазия.txt";

        let rng = RNG::new_from_file(filename.to_string());
        let result = rng.as_ref().unwrap();

        assert!(!rng.is_err());
        assert_eq!(result.name, filename.to_string());
        assert_eq!(result.bad_syllables.len(), 0);
        assert_eq!(result.prefixes.len(), 180);
        assert_eq!(result.centers.len(), 157);
        assert_eq!(result.suffixes.len(), 19);
    }

    #[test]
    fn process_file__with_error() {
        let filename = "src/languages/none.txt";

        let result = RNG::new_from_file(filename.to_string());

        assert!(result.is_err());
    }

    #[test]
    fn classify() {
        let raw = "-ваа +c\n-боо +c\n-гар\n-бар\n-дар\n-жар\n-вар\n-кра\n-гра\n-дра\n-зра\n-гоб\n-доб\n-роб\n-фоб\n-зоб\n-раг\n-наг\n-даг\nбра\nга\nда\nдо\nго\nзе\nша\nназ\nзуб\nзу\nна\nгор\nбу +c\n+быр\n+гыр\n+д";
        let filename = "src/languages/goblinRU.txt".to_string();

        let classified = RNG::classify(raw, filename.clone());

        assert_eq!(classified.name, filename);
        assert_eq!(classified.bad_syllables.len(), 0);
        assert_eq!(classified.prefixes.len(), 19);
        assert_eq!(classified.centers.len(), 13);
        assert_eq!(classified.suffixes.len(), 3);
    }

    #[test]
    fn classify__fantasy_russian() {
        let raw = "-а +c\n-аб\n-ак\n-ац\n-ад\n-аф\n-ам\n-ан\n-ап\n-ар\n-ас\n-ат\n-ав\n-аз\n-аэль\n-аэл\n-ао\n-аэр\n-аш\n-арш +v";
        let filename = "src/languages/goblinRU.txt".to_string();

        let classified = RNG::classify(raw, filename.clone());

        assert_eq!(classified.name, filename);
        assert_eq!(classified.bad_syllables.len(), 0);
        // assert_eq!(classified.prefixes.len(), 19);
        // assert_eq!(classified.centers.len(), 13);
        // assert_eq!(classified.suffixes.len(), 3);
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
    fn generate_names() {
        let rng = RNG::try_from(&Language::Roman).unwrap();

        let names = rng.generate_names(5, true);

        assert_eq!(names.len(), 5);
    }

    #[test]
    fn generate_names_string() {
        let rng = RNG::try_from(&Language::Demonic).unwrap();

        let names = rng.generate_names_string(12, true);

        assert_eq!(names.split_whitespace().count(), 12);
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
        let rng = RNG::try_from(&Language::Elven).unwrap();
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
            let rng = RNG::try_from(&language).unwrap();

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
    Эльфийский,
    Fantasy,
    Фантазия,
    Goblin,
    Гоблин,
    Roman,
    Римский,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Distribution<Language> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Language {
        match rng.gen_range(0..5) {
            0 => Language::Demonic,
            1 => Language::Elven,
            2 => Language::Fantasy,
            3 => Language::Goblin,
            _ => Language::Roman,
        }
    }
}

impl Language {
    #[must_use]
    pub fn get_filename(&self) -> String {
        format!("{self}.txt")
    }

    #[must_use]
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
