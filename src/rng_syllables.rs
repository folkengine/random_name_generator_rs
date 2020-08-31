use rand::seq::SliceRandom;

use crate::rng_joiner::{Joiner};
use crate::rng_syllable::{Classification, Syllable};

// region Syllables

#[derive(Debug)]
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

    pub fn get(&self, index: usize) -> Option<&Syllable> {
        self.0.get(index)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn next_from(&self, _from: Syllable) -> Syllable {
        return self.0.choose(&mut rand::thread_rng()).unwrap().clone();
    }
}

impl IntoIterator for Syllables {
    type Item = Syllable;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

// endregion

#[cfg(test)]
#[allow(non_snake_case)]
mod syllables_tests {
    use super::*;

    #[test]
    fn syllables__add() {
        let mut c = Syllables::new_from_array(&["ch", "abc"]);
        c.add(Syllable::new("efg").unwrap());
        c.add(Syllable::new("hij").unwrap());

        assert_eq!(c.len(), 4);
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
        let c = Syllables::new_from_array(&["ch", "abc", "er", "go", "to"]);

        assert_eq!(c.len(), 5);
        assert_eq!(c.get(0).unwrap(), &Syllable::new("ch").unwrap());
        assert_eq!(c.get(1).unwrap(), &Syllable::new("abc").unwrap());
        assert_eq!(c.get(2).unwrap(), &Syllable::new("er").unwrap());
        assert_eq!(c.get(3).unwrap(), &Syllable::new("go").unwrap());
        assert_eq!(c.get(4).unwrap(), &Syllable::new("to").unwrap());
        assert!(c.get(5).is_none());
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
    fn next_from() {
        let b = Syllable::new("b").unwrap();
        let mut v = Syllables::new();
        v.add(b.clone());
        let a = Syllable::new("a").unwrap();

        let actual = v.next_from(a);

        assert_eq!(actual, b);
    }

}