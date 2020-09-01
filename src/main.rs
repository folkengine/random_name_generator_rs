use clap::{App, ArgMatches};
use rnglib::RNG;
use rnglib::rng_language::Language;

fn main() {
    let matches = get_matches();

    let dialect = determine_dialect(&matches);

    if matches.is_present("dump") {
        dump(dialect)
    } else {
        generate_name(dialect)
    }
}

fn dump(dialect: rnglib::RNG) {
    for s in dialect.syllables().into_iter() {
        println!("{}", s.to_string())
    }
}

fn generate_name(dialect: rnglib::RNG) {
    let first_name = dialect.generate_name();
    let last_name = dialect.generate_name();

    println!("{}: {} {}", dialect.name, first_name, last_name)
}

fn determine_dialect(matches: &ArgMatches) -> RNG {
    if matches.is_present("elven") {
        RNG::generate_dialect(&Language::Elven)
    } else if matches.is_present("fantasy") {
        RNG::generate_dialect(&Language::Fantasy)
    } else if matches.is_present("goblin") {
        RNG::generate_dialect(&Language::Goblin)
    } else if matches.is_present("roman") {
        RNG::generate_dialect(&Language::Roman)
    } else {
        let my_dialect_type: Language = rand::random();
        RNG::generate_dialect(&my_dialect_type)
    }
}

fn get_matches() -> ArgMatches {
    App::new("RandomNameGenerator")
        .version("0.0.1")
        .author("Christoph <gaoler@electronicpanopticon.com>")
        .about("Generates random names in various languages")
        .arg("-e, --elven 'Use the Elven language'")
        .arg("-f, --fantasy 'Use the Fantasy language'")
        .arg("-g, --goblin 'Use the Goblin language'")
        .arg("-r, --roman 'Use the Roman language'")
        .arg("-d, --dump 'Print out the raw lanuage file'")
        .arg("-x, --flipmode 'Use a random language'")
        .get_matches()
}
