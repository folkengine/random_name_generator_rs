enum Classification {
    Prefix,
    Center,
    Suffix,
}

enum Rule {
    Consonant,
    Vowel,
    None,
}

struct Syllable {
    value: String,
    classification: Classification,
    rule: Rule,
}

#[cfg(test)]
mod tests {

}