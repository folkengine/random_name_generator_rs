use clap::{command, Arg, ArgAction, ArgMatches};
use rnglib::{Language, RNG};

fn main() {
    let matches = cmd().get_matches();

    let name = generate(&matches);

    println!("{name}");
}

fn cmd() -> clap::Command {
    command!()
        .arg(
            Arg::new("elven")
                .short('e')
                .long("elven")
                .required(false)
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("fantasy")
                .short('f')
                .long("fantasy")
                .required(false)
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("goblin")
                .short('g')
                .long("goblin")
                .required(false)
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("roman")
                .short('r')
                .long("roman")
                .required(false)
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("curse")
                .short('c')
                .long("curse")
                .required(false)
                .action(ArgAction::SetTrue)
                .help("[UNDER CONSTRUCTION]"),
        )
        .arg(
            Arg::new("flipmode")
                .short('x')
                .long("flipmode")
                .required(false)
                .action(ArgAction::SetTrue)
                .help("Flipmode is the greatest! (Random language)"),
        )
        .arg(
            Arg::new("raw")
                .long("raw")
                .required(false)
                .value_name("FILE")
                .help("Reads in a raw language file"),
        )
        .arg_required_else_help(true)
}

fn generate(matches: &ArgMatches) -> String {
    if matches.get_flag("elven") {
        generate_name(RNG::new(&Language::Elven).unwrap())
    } else if matches.get_flag("fantasy") {
        generate_name(RNG::new(&Language::Fantasy).unwrap())
    } else if matches.get_flag("goblin") {
        generate_name(RNG::new(&Language::Goblin).unwrap())
    } else if matches.get_flag("roman") {
        generate_name(RNG::new(&Language::Roman).unwrap())
    } else if matches.get_flag("curse") {
        RNG::new(&Language::Curse).unwrap().generate_short()
    } else {
        let my_dialect_type: Language = rand::random();
        generate_name(RNG::new(&my_dialect_type).unwrap())
    }
}

fn generate_name(rng: rnglib::RNG) -> String {
    let first_name = rng.generate_name();
    let last_name = rng.generate_name();

    format!("{}: {} {}", rng.name, first_name, last_name)
}

#[test]
fn verify_cmd() {
    cmd().debug_assert();
}
