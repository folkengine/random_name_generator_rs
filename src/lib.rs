mod rng_dialect;
mod rng_syllable;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const DEMONIC: &'static str = "demonic";
const ELVEN: &'static str = "elven";

pub fn test() {
    if let Ok(lines) = read_lines(lang_path(ELVEN)) {
        for line in lines {
            if let Ok(ip) = line {
                process_line(ip);
            }
        }
    } else {
        println!("nope!");
    }
}

fn process_line(line: String) {
    let sy = rng_syllable::Syllable::new(line.as_str());
    if sy.is_ok() {
        println!("{}", sy.unwrap().to_string());
    } else {
        println!("Invalid syllable: {}", line);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn lang_path(n: &str) -> String {
    format!("./src/languages/{}.txt", n)
}

#[cfg(test)]
mod lib_tests {
    use super::*;

    #[test]
    fn languages() {
        assert_eq!(DEMONIC, "demonic");
    }

    #[test]
    fn file_path() {
        assert_eq!("./src/languages/demonic.txt".to_string(), lang_path(DEMONIC));
    }
}