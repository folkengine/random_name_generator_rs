use clap::{App, ArgMatches};
use rnglib::generate_dialect;

use rnglib::rng_dialect::{Dialects, Dialect};

fn main() {
    let matches = get_matches();

    let dialect = determine_dialect(&matches);

    let first_name = dialect.generate_name();
    let last_name = dialect.generate_name();

    println!("{}: {} {}", dialect.name, first_name, last_name)
}

fn determine_dialect(matches: &ArgMatches) -> Dialect {
    if matches.is_present("elven") {
        generate_dialect(&Dialects::Elven)
    } else if matches.is_present("fantasy") {
        generate_dialect(&Dialects::Fantasy)
    } else if matches.is_present("goblin") {
        generate_dialect(&Dialects::Goblin)
    } else if matches.is_present("roman") {
        generate_dialect(&Dialects::Roman)
    } else {
        let my_dialect_type: Dialects = rand::random();
        generate_dialect(&my_dialect_type)
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
        .arg("-x, --flipmode 'Use a random language'")
        .get_matches()
}
