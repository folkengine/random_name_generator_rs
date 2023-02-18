use clap::{command, Arg, ArgAction, ArgMatches};
use rnglib::{Language, RNGError, RNG};

fn main() -> Result<(), RNGError> {
    let matches = cmd().get_matches();

    let _count: usize = *get_number(&matches).unwrap();
    let _rng = get_rng(&matches).unwrap();

    // if matches.get_flag("raw") {
    //     let raw = matches.get_one::<String>("raw").unwrap();
    //     let result = RNG::new_from_file(raw.clone());
    //
    //     match result {
    //         Ok(rng) => Ok(generate_name(&rng)),
    //         Err(_) => Err(RNGError::InvalidLanguageFile),
    //     }
    // } else {
    //     let name = generate(&matches);
    //
    //     if name.is_ok() {
    //         println!("{}", name.unwrap());
    //     } else {
    //         println!("{:?}", name.unwrap_err());
    //     }
    // }

    Ok(())
}

fn cmd() -> clap::Command {
    command!()
        .arg(
            Arg::new("demonic")
                .short('d')
                .long("demonic")
                .required(false)
                .action(ArgAction::SetTrue),
        )
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
            Arg::new("russian")
                .long("russian")
                .required(false)
                .action(ArgAction::SetTrue)
                .help("Use Russian language file, if available"),
        )
        .arg(
            Arg::new("raw")
                .long("raw")
                .required(false)
                .value_name("FILE")
                .help("Reads in a raw language file"),
        )
        .arg(
            Arg::new("number")
                .short('n')
                .long("number")
                .required(false)
                .default_value("2")
                .value_parser(clap::value_parser!(usize))
                .help("Number of names created."),
        )
        .arg_required_else_help(true)
}

fn get_number(matches: &ArgMatches) -> Option<&usize> {
    matches
        .try_get_one::<usize>("number")
        .expect("Could not read a threshold value")
}

fn get_rng(matches: &ArgMatches) -> Result<RNG, RNGError> {
    if matches.get_flag("demonic") {
        Ok(RNG::try_from(&Language::Demonic)?)
    } else if matches.get_flag("elven") {
        if matches.get_flag("russian") {
            Ok(RNG::try_from(&Language::Эльфийский)?)
        } else {
            Ok(RNG::try_from(&Language::Elven)?)
        }
    } else if matches.get_flag("fantasy") {
        if matches.get_flag("russian") {
            Ok(RNG::try_from(&Language::Фантазия)?)
        } else {
            Ok(RNG::try_from(&Language::Fantasy)?)
        }
    } else if matches.get_flag("goblin") {
        if matches.get_flag("russian") {
            Ok(RNG::try_from(&Language::Гоблин)?)
        } else {
            Ok(RNG::try_from(&Language::Goblin)?)
        }
    } else if matches.get_flag("roman") {
        if matches.get_flag("russian") {
            Ok(RNG::try_from(&Language::Римский)?)
        } else {
            Ok(RNG::try_from(&Language::Roman)?)
        }
    } else if matches.get_flag("curse") {
        Ok(RNG::try_from(&Language::Curse)?)
    } else if matches.get_flag("flipmode") {
        let my_dialect_type: Language = rand::random();
        Ok(RNG::try_from(&my_dialect_type)?)
    } else {
        let raw = matches.get_one::<String>("raw").unwrap();
        let result = RNG::new_from_file(raw.clone());

        match result {
            Ok(rng) => Ok(rng),
            Err(_) => Err(RNGError::InvalidLanguageFile),
        }
    }
}

fn _generate(matches: &ArgMatches) -> Result<String, RNGError> {
    if matches.get_flag("demonic") {
        let rng = &RNG::try_from(&Language::Demonic)?;
        Ok(format!("{}: {}", rng.name, rng.generate_name()))
    } else if matches.get_flag("elven") {
        if matches.get_flag("russian") {
            Ok(_generate_name(&RNG::try_from(&Language::Эльфийский)?))
        } else {
            Ok(_generate_name(&RNG::try_from(&Language::Elven)?))
        }
    } else if matches.get_flag("fantasy") {
        if matches.get_flag("russian") {
            Ok(_generate_name(&RNG::try_from(&Language::Фантазия)?))
        } else {
            Ok(_generate_name(&RNG::try_from(&Language::Fantasy)?))
        }
    } else if matches.get_flag("goblin") {
        if matches.get_flag("russian") {
            Ok(_generate_name(&RNG::try_from(&Language::Гоблин)?))
        } else {
            Ok(_generate_name(&RNG::try_from(&Language::Goblin)?))
        }
    } else if matches.get_flag("roman") {
        if matches.get_flag("russian") {
            Ok(_generate_name(&RNG::try_from(&Language::Римский)?))
        } else {
            Ok(_generate_name(&RNG::try_from(&Language::Roman)?))
        }
    } else if matches.get_flag("curse") {
        Ok(RNG::try_from(&Language::Curse)?.generate_short())
    } else if matches.get_flag("flipmode") {
        let my_dialect_type: Language = rand::random();
        Ok(_generate_name(&RNG::try_from(&my_dialect_type)?))
    } else {
        let raw = matches.get_one::<String>("raw").unwrap();
        let result = RNG::new_from_file(raw.clone());

        match result {
            Ok(rng) => Ok(_generate_name(&rng)),
            Err(_) => Err(RNGError::InvalidLanguageFile),
        }
    }
}

fn _generate_name(rng: &rnglib::RNG) -> String {
    let first_name = rng.generate_name();
    let last_name = rng.generate_name();

    format!("{}: {} {}", rng.name, first_name, last_name)
}

#[test]
fn verify_cmd() {
    cmd().debug_assert();
}
