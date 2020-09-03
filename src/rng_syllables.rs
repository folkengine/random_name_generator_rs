use rand::distributions::{Distribution, Uniform};

use crate::rng_joiner::Joiner;
use crate::rng_syllable::Syllable;

/// Syllables is a single field struct containing a Vector of Syllable structs. Syllables facilites
/// filtering on Syllable Joiners allowing for dialects to easily determine the next syllable for
/// a generated name.
#[derive(Clone, Debug, PartialEq)]
pub struct Syllables(Vec<Syllable>);

impl Syllables {
    pub fn new() -> Syllables {
        Syllables::new_from_vector(Vec::new())
    }

    pub fn new_from_vector(v: Vec<Syllable>) -> Syllables {
        Syllables(v)
    }

    pub fn new_from_array(syl_strs: &[&str]) -> Syllables {
        let mut syllables: Vec<Syllable> = Vec::new();
        for s in syl_strs.iter() {
            syllables.push(Syllable::new(s).unwrap());
        }
        Syllables::new_from_vector(syllables)
    }

    pub fn add(&mut self, elem: Syllable) {
        self.0.push(elem);
    }

    pub fn all(&self) -> &Vec<Syllable> {
        &self.0
    }

    pub fn collapse(&self) -> String {
        let mut s = "".to_string();
        for i in 0..self.len() {
            s.push_str(self.get(i).unwrap().value.as_str())
        }
        s
    }

    pub fn contains(&self, syllable: &Syllable) -> bool {
        self.0.contains(syllable)
    }

    pub fn filter_from(&self, from: Joiner) -> Syllables {
        let v = self.0.iter().filter(|s| from.joins(&s.jprevious))
            .cloned()
            .collect();
        Syllables::new_from_vector(v)
    }

    pub fn first(&self) -> Option<&Syllable> {
        self.0.first()
    }

    pub fn get(&self, index: usize) -> Option<&Syllable> {
        self.0.get(index)
    }

    pub fn get_random(&self) -> Option<&Syllable> {
        self.0.get(self.rnd())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn last(&self) -> Option<&Syllable> {
        self.0.last()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn next_from(&self, from_syllable: Syllable) -> Syllable {
        self.filter_from(from_syllable.jnext).get_random().unwrap().clone()
    }

    /// Generates a random value from the length of the Syllable Vector - 1.
    /// https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html#generate-random-numbers-within-a-range
    fn rnd(&self) -> usize {
        let mut rng = rand::thread_rng();
        let length = self.len();
        if length < 2 {
            0
        } else {
            let die = Uniform::from(0..self.len() - 1);
            die.sample(&mut rng)
        }
    }
}

impl IntoIterator for Syllables {
    type Item = Syllable;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod syllables_tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn add() {
        let mut syllables = Syllables::new_from_array(&["ch", "abc"]);
        syllables.add(Syllable::new("efg").unwrap());
        syllables.add(Syllable::new("hij").unwrap());

        assert_eq!(syllables.len(), 4);
    }

    #[test]
    fn all() {
        let c = Syllables::new_from_array(&["ch", "abc"]);
        let all: &Vec<Syllable> = c.all();

        assert_eq!(c.len(), 2);
        assert_eq!(c.get(0).unwrap(), &Syllable::new("ch").unwrap());
        assert_eq!(c.get(1).unwrap(), &Syllable::new("abc").unwrap());
        assert_eq!(all.len(), 2);
        assert_eq!(all.get(0).unwrap(), &Syllable::new("ch").unwrap());
        assert_eq!(all.get(1).unwrap(), &Syllable::new("abc").unwrap());
    }

    #[test]
    fn collapse() {
        let syllables = Syllables::new_from_array(&["ch", "abc"]);

        let s = syllables.collapse();

        assert_eq!("chabc".to_string(), s);
    }

    #[test]
    fn contains() {
        let mut syllables = Syllables::new_from_array(&["ch", "abc"]);
        let efg = Syllable::new("efg").unwrap();
        assert!(!syllables.contains(&efg));

        syllables.add(efg.clone());
        assert!(syllables.contains(&efg));
    }

    #[test]
    fn filter_from() {
        let syllables = Syllables::new_from_array(&["ch", "abc"]);
        let joiner = Joiner::SOME | Joiner::ONLY_VOWEL;

        let filtered = syllables.filter_from(joiner);

        assert!(!filtered.contains(&Syllable::new("ch").unwrap()));
        assert!(filtered.contains(&Syllable::new("abc").unwrap()));
    }

    #[test]
    fn first() {
        let zero = Syllables::new();
        let three = Syllables::new_from_array(&["ch", "abc", "efg"]);

        assert!(zero.first().is_none());
        assert_eq!(three.first().unwrap(), &Syllable::new("ch").unwrap());
    }

    #[test]
    fn last() {
        let zero = Syllables::new();
        let three = Syllables::new_from_array(&["ch", "abc", "efg"]);

        assert!(zero.last().is_none());
        assert_eq!(three.last().unwrap(), &Syllable::new("efg").unwrap());
    }

    #[test]
    fn len() {
        let zero = Syllables::new();
        let three = Syllables::new_from_array(&["ch", "abc", "efg"]);

        assert_eq!(zero.len(), 0);
        assert_eq!(three.len(), 3);
    }

    #[test]
    fn get() {
        let syllables = Syllables::new_from_array(&["ch", "abc", "er", "go", "to"]);

        assert_eq!(syllables.len(), 5);
        assert_eq!(syllables.get(0).unwrap(), &Syllable::new("ch").unwrap());
        assert_eq!(syllables.get(1).unwrap(), &Syllable::new("abc").unwrap());
        assert_eq!(syllables.get(2).unwrap(), &Syllable::new("er").unwrap());
        assert_eq!(syllables.get(3).unwrap(), &Syllable::new("go").unwrap());
        assert_eq!(syllables.get(4).unwrap(), &Syllable::new("to").unwrap());
        assert!(syllables.get(5).is_none());
    }

    #[test]
    fn next_from() {
        let b = Syllable::new("b").unwrap();
        let mut v = Syllables::new();
        v.add(b.clone());
        let a = Syllable::new("a").unwrap();

        let actual = v.next_from(a);

        assert_eq!(actual, b);
    }

    proptest! {
        #[test]
        fn rnd_test(_ in 0..20i32) {
            let c = Syllables::new_from_array(&["ch", "abc", "er", "go", "to"]);
            let n = c.rnd();
            assert!(n < c.len());
        }

        #[test]
        fn get_random(_ in 0..20i32) {
            let syllables = Syllables::new_from_array(&["ch", "abc", "er", "go", "to"]);

            let rnd = syllables.get_random().unwrap();
            let non = Syllable::new("efg").unwrap();

            assert!(syllables.contains(rnd));
            assert!(!syllables.contains(&non));
        }
    }
}