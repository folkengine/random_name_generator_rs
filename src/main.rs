use clap::{Arg, ArgAction, ArgMatches, command};
use rnglib::{Language, RNG, RNGError};

static HELP_TEMPLATE: &str = "{about} {version}

Usage: rng

Options:
{options}
";

fn main() -> Result<(), RNGError> {
    let matches = cmd().get_matches();

    let count: usize = *get_number(&matches).ok_or(RNGError::ParsingError)?;
    let rng = get_rng(&matches)?;

    if matches.get_flag("no-prefix") {
        println!(
            "{}",
            rng.generate_names_string(count, matches.get_flag("short"))
        );
    } else {
        println!(
            "{}: {}",
            rng.name,
            rng.generate_names_string(count, matches.get_flag("short"))
        );
    }

    Ok(())
}

#[allow(clippy::too_many_lines)]
fn cmd() -> clap::Command {
    command!()
        .help_template(HELP_TEMPLATE)
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
            Arg::new("klingon")
                .short('k')
                .long("klingon")
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
            Arg::new("german-curse")
                .long("german-curse")
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
            Arg::new("short")
                .long("short")
                .required(false)
                .action(ArgAction::SetTrue)
                .help("Creates shorter names"),
        )
        .arg(
            Arg::new("raw")
                .long("raw")
                .required(false)
                .value_name("FILE")
                .help("Reads in a raw language file"),
        )
        .arg(
            Arg::new("no-prefix")
                .short('p')
                .long("no-prefix")
                .required(false)
                .action(ArgAction::SetTrue)
                .help("Don't print language chosen (for use with -x)"),
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
    let is_russian = matches.get_flag("russian");

    if matches.get_flag("demonic") {
        RNG::try_from(&Language::Demonic)
    } else if matches.get_flag("elven") {
        filter_russian(is_russian, &Language::Elven, &Language::Эльфийский)
    } else if matches.get_flag("fantasy") {
        filter_russian(is_russian, &Language::Fantasy, &Language::Фантазия)
    } else if matches.get_flag("goblin") {
        filter_russian(is_russian, &Language::Goblin, &Language::Гоблин)
    } else if matches.get_flag("klingon") {
        RNG::try_from(&Language::Klingon)
    } else if matches.get_flag("roman") {
        filter_russian(is_russian, &Language::Roman, &Language::Римский)
    } else if matches.get_flag("curse") {
        RNG::try_from(&Language::Curse)
    } else if matches.get_flag("german-curse") {
        RNG::try_from(&Language::GermanCurse)
    } else if matches.get_flag("flipmode") {
        Ok(RNG::random())
    } else {
        let raw = matches
            .get_one::<String>("raw")
            .ok_or(RNGError::ParsingError)?;
        RNG::new_from_file(raw.clone())
    }
}

fn filter_russian(
    is_russian: bool,
    english: &Language,
    russian: &Language,
) -> Result<RNG, RNGError> {
    if is_russian {
        RNG::try_from(russian)
    } else {
        RNG::try_from(english)
    }
}

#[test]
fn verify_cmd() {
    cmd().debug_assert();
}
