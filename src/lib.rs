mod rng_dialect;
mod rng_syllable;

pub fn test() {
    let _ = rng_dialect::Dialect::new_from_path(
            rng_dialect::Dialects::Elven.get_path(),
            rng_dialect::Dialects::Elven.to_string())
        .unwrap();

}

fn process_line(line: String) {
    let sy = rng_syllable::Syllable::new(line.as_str());
    if sy.is_ok() {
        println!("{}", sy.unwrap().to_string());
    } else {
        println!("Invalid syllable: {}", line);
    }
}

#[cfg(test)]
mod lib_tests {
    use super::*;

}