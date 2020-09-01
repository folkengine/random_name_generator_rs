mod rng_joiner;
mod rng_syllable;
mod rng_syllables;
pub mod rng_language;

#[macro_use]
extern crate bitflags;
extern crate log;

use rand::{
    distributions::{Distribution, WeightedIndex},
    prelude::*,
};
use rust_embed::RustEmbed;
use titlecase::titlecase;
use crate::rng_language::{Language};
use crate::rng_syllable::{Classification, Syllable};
use crate::rng_syllables::{Syllables};

/// RNG (Random Name Generator) is a library that generates random
/// names based upon one of the available Languages.
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
        let txt = Asset::get(language.get_filename().as_str()).unwrap();
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
        let d = RNG {
            name: language.to_string(),
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

    pub fn is_valid(&self) -> bool {
        self.bad_syllables.is_empty()
    }

    pub fn generate_dialect(language: &Language) -> RNG {
        RNG::new(language).unwrap()
    }

    pub fn generate_name(&self)  -> String {
        let name = self.generate_syllables_by_count(gen_rnd_syllable_count()).collapse().clone();
        titlecase(name.as_str()).to_string()
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

#[derive(RustEmbed)]
#[folder = "src/languages/"]
struct Asset;

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
    fn generate_name() {
        let min = RNG {
            name: "Min".to_string(),
            prefixes: Syllables::new_from_array(&["a"]),
            centers: Syllables::new_from_array(&["b"]),
            suffixes: Syllables::new_from_array(&["c"]),
            bad_syllables: vec![],
        };

        for _ in 0..9 {
            let name = min.generate_name();
            assert!(name.as_str().starts_with("A"));
            assert!(name.as_str().ends_with("c"))
        }
    }

    #[test]
    fn generate_syllables() {
        for _ in 0..9 {
            let rng = RNG::new(&Language::Elven).unwrap();

            let syllables = rng.generate_syllables();

            assert!(syllables.len() > 1);
            assert!(syllables.len() < 6);
        }
    }

    #[test]
    fn is_valid() {
        let elven = RNG {
            name: "".to_string(),
            prefixes: Syllables::new(),
            centers: Syllables::new(),
            suffixes: Syllables::new(),
            bad_syllables: vec![],
        };
        assert!(elven.is_valid())
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
        let rng = RNG::generate_dialect(&Language::Elven);

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
            let count = gen_rnd_syllable_count();
            assert!((count < 6) && (count > 1), count);
        }
    }
}
