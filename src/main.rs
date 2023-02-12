use clap::{command, Arg, ArgAction, ArgMatches};
use rnglib::{Language, RNGError, RNG};

fn main() {
    let matches = cmd().get_matches();

    let name = generate(&matches);

    if name.is_ok() {
        println!("{}", name.unwrap());
    } else {
        println!("{:?}", name.unwrap_err());
    }
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

fn generate(matches: &ArgMatches) -> Result<String, RNGError> {
    if matches.get_flag("elven") {
        Ok(generate_name(&RNG::try_from(&Language::Elven)?))
    } else if matches.get_flag("fantasy") {
        Ok(generate_name(&RNG::try_from(&Language::Fantasy)?))
    } else if matches.get_flag("goblin") {
        Ok(generate_name(&RNG::try_from(&Language::Goblin)?))
    } else if matches.get_flag("roman") {
        Ok(generate_name(&RNG::try_from(&Language::Roman)?))
    } else if matches.get_flag("curse") {
        Ok(RNG::try_from(&Language::Curse)?.generate_short())
    } else if matches.get_flag("flipmode") {
        let my_dialect_type: Language = rand::random();
        Ok(generate_name(&RNG::try_from(&my_dialect_type)?))
    } else {
        let raw = matches.get_one::<String>("raw").unwrap();
        let result = RNG::new_from_file(raw.clone());

        match result {
            Ok(rng) => Ok(generate_name(&rng)),
            Err(_) => Err(RNGError::InvalidLanguageFile),
        }
    }
}

fn generate_name(rng: &rnglib::RNG) -> String {
    let first_name = rng.generate_name();
    let last_name = rng.generate_name();

    format!("{}: {} {}", rng.name, first_name, last_name)
}

#[test]
fn verify_cmd() {
    cmd().debug_assert();
}
