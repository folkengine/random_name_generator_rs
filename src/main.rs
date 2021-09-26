use clap::{crate_authors, crate_license, crate_name, crate_version, App, ArgMatches};
use rnglib::{Language, RNG};

const PKG_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

fn main() {
    let matches = get_matches();

    let rng = determine_language(&matches);

    if matches.is_present("dump") {
        dump(rng)
    } else if matches.is_present("curse") {
        curse(rng)
    } else {
        generate_name(rng)
    }
}

fn dump(rng: rnglib::RNG) {
    for s in rng.syllables().into_iter() {
        println!("{}", s.to_string())
    }
}

fn curse(rng: rnglib::RNG) {
    let word = rng.generate_short();
    println!("{}", word)
}

fn generate_name(rng: rnglib::RNG) {
    let first_name = rng.generate_name();
    let last_name = rng.generate_name();

    println!("{}: {} {}", rng.name, first_name, last_name)
}

fn determine_language(matches: &ArgMatches) -> RNG {
    if matches.is_present("raw") {
        RNG::new_from_file(matches.value_of("raw").unwrap().to_string()).unwrap()
    } else if matches.is_present("elven") {
        RNG::new(&Language::Elven).unwrap()
    } else if matches.is_present("fantasy") {
        RNG::new(&Language::Fantasy).unwrap()
    } else if matches.is_present("goblin") {
        RNG::new(&Language::Goblin).unwrap()
    } else if matches.is_present("roman") {
        RNG::new(&Language::Roman).unwrap()
    } else if matches.is_present("demonic") {
        RNG::new(&Language::Demonic).unwrap_err()
    } else if matches.is_present("curse") {
        RNG::new(&Language::Curse).unwrap()
    } else {
        let my_dialect_type: Language = rand::random();
        RNG::new(&my_dialect_type).unwrap()
    }
}

fn get_matches() -> ArgMatches {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .license(crate_license!())
        .about(PKG_DESCRIPTION)
        .arg("-c, --curse 'Use the Curse language [UNDER CONSTRUCTION]'")
        .arg("-d, --demonic 'Use the Demonic language [UNDER CONSTRUCTION]'")
        .arg("-e, --elven 'Use the Elven language'")
        .arg("-f, --fantasy 'Use the Fantasy language'")
        .arg("-g, --goblin 'Use the Goblin language'")
        .arg("-r, --roman 'Use the Roman language'")
        .arg("--dump 'Print out the raw lanuage file'")
        .arg("-x, --flipmode 'Use a random language'")
        .arg("--raw=[FILE] 'reads in a raw language file'")
        .get_matches()
}
