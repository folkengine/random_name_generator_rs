mod rng_dialect;
mod rng_syllable;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn test() {
    if let Ok(lines) = read_lines(rng_dialect::Dialects::Elven.get_path()) {
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

#[cfg(test)]
mod lib_tests {
    use super::*;

}