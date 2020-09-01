use clap::{App, ArgMatches};
use rnglib::RNG;
use rnglib::rng_language::Language;

fn main() {
    let matches = get_matches();

    let rng = determine_language(&matches);

    if matches.is_present("dump") {
        dump(rng)
    } else {
        generate_name(rng)
    }
}

fn dump(rng: rnglib::RNG) {
    for s in rng.syllables().into_iter() {
        println!("{}", s.to_string())
    }
}

fn generate_name(rng: rnglib::RNG) {
    let first_name = rng.generate_name();
    let last_name = rng.generate_name();

    println!("{}: {} {}", rng.name, first_name, last_name)
}

fn determine_language(matches: &ArgMatches) -> RNG {
    if matches.is_present("elven") {
        RNG::new(&Language::Elven).unwrap()
    } else if matches.is_present("fantasy") {
        RNG::new(&Language::Fantasy).unwrap()
    } else if matches.is_present("goblin") {
        RNG::new(&Language::Goblin).unwrap()
    } else if matches.is_present("roman") {
        RNG::new(&Language::Roman).unwrap()
    } else if matches.is_present("demonic") {
        RNG::new(&Language::Demonic).unwrap_err()
    } else {
        let my_dialect_type: Language = rand::random();
        RNG::new(&my_dialect_type).unwrap()
    }
}

fn get_matches() -> ArgMatches {
    App::new("RandomNameGenerator")
        .version("0.0.1")
        .author("Christoph <gaoler@electronicpanopticon.com>")
        .about("Generates random names in various languages")
        .arg("-d, --demonic 'Use the Demonic language [UNDER CONSTRUCTION]'")
        .arg("-e, --elven 'Use the Elven language'")
        .arg("-f, --fantasy 'Use the Fantasy language'")
        .arg("-g, --goblin 'Use the Goblin language'")
        .arg("-r, --roman 'Use the Roman language'")
        .arg("--dump 'Print out the raw lanuage file'")
        .arg("-x, --flipmode 'Use a random language'")
        .get_matches()
}
