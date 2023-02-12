use clap::{command, Arg, ArgAction, ArgMatches};
use rnglib::{Language, RNG};

// ArgMatches {
//     valid_args: ["elven", "fantasy", "goblin", "roman", "curse", "demonic", "dump", "flipmode", "raw", "help", "version"],
//     valid_subcommands: [],
//     args: FlatMap { keys: ["fantasy", "elven", "goblin", "roman", "curse", "demonic", "dump", "flipmode"],
//     values: [MatchedArg { source: Some(CommandLine), indices: [1], type_id: Some(bool), vals: [[AnyValue { inner: bool }]], raw_vals: [["true"]], ignore_case: false }, MatchedArg { source: Some(DefaultValue), indices: [2], type_id: Some(bool), vals: [[AnyValue { inner: bool }]], raw_vals: [["false"]], ignore_case: false }, MatchedArg { source: Some(DefaultValue), indices: [3], type_id: Some(bool), vals: [[AnyValue { inner: bool }]], raw_vals: [["false"]], ignore_case: false }, MatchedArg { source: Some(DefaultValue), indices: [4], type_id: Some(bool), vals: [[AnyValue { inner: bool }]], raw_vals: [["false"]], ignore_case: false }, MatchedArg { source: Some(DefaultValue), indices: [5], type_id: Some(bool), vals: [[AnyValue { inner: bool }]], raw_vals: [["false"]], ignore_case: false }, MatchedArg { source: Some(DefaultValue), indices: [6], type_id: Some(bool), vals: [[AnyValue { inner: bool }]], raw_vals: [["false"]], ignore_case: false }, MatchedArg { source: Some(DefaultValue), indices: [7], type_id: Some(bool), vals: [[AnyValue { inner: bool }]], raw_vals: [["false"]], ignore_case: false }, MatchedArg { source: Some(DefaultValue), indices: [8], type_id: Some(bool), vals: [[AnyValue { inner: bool }]], raw_vals: [["false"]], ignore_case: false }] }, subcommand: None }
//
// ArgMatches {
//     valid_args: ["elven", "fantasy", "goblin", "roman", "curse", "demonic", "dump", "flipmode", "raw", "help", "version"],
//     valid_subcommands: [],
//     args: FlatMap { keys: ["elven", "fantasy", "goblin", "roman", "curse", "demonic", "dump", "flipmode"],
//     values: [MatchedArg { source: Some(CommandLine), indices: [1], type_id: Some(bool), vals: [[AnyValue { inner: bool }]], raw_vals: [["true"]], ignore_case: false }, MatchedArg { source: Some(DefaultValue), indices: [2], type_id: Some(bool), vals: [[AnyValue { inner: bool }]], raw_vals: [["false"]], ignore_case: false }, MatchedArg { source: Some(DefaultValue), indices: [3], type_id: Some(bool), vals: [[AnyValue { inner: bool }]], raw_vals: [["false"]], ignore_case: false }, MatchedArg { source: Some(DefaultValue), indices: [4], type_id: Some(bool), vals: [[AnyValue { inner: bool }]], raw_vals: [["false"]], ignore_case: false }, MatchedArg { source: Some(DefaultValue), indices: [5], type_id: Some(bool), vals: [[AnyValue { inner: bool }]], raw_vals: [["false"]], ignore_case: false }, MatchedArg { source: Some(DefaultValue), indices: [6], type_id: Some(bool), vals: [[AnyValue { inner: bool }]], raw_vals: [["false"]], ignore_case: false }, MatchedArg { source: Some(DefaultValue), indices: [7], type_id: Some(bool), vals: [[AnyValue { inner: bool }]], raw_vals: [["false"]], ignore_case: false }, MatchedArg { source: Some(DefaultValue), indices: [8], type_id: Some(bool), vals: [[AnyValue { inner: bool }]], raw_vals: [["false"]], ignore_case: false }] }, subcommand: None }

fn main() {
    let matches = cmd().get_matches();

    let rng = determine_language(&matches);

    // println!("{:?}", matches);

    // generate_name(rng);

    generate_name(rng)
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
            Arg::new("demonic")
                .short('d')
                .long("demonic")
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
}

fn determine_language(matches: &ArgMatches) -> RNG {
    if matches.get_flag("elven") {
        RNG::new(&Language::Elven).unwrap()
    } else if matches.get_flag("fantasy") {
        RNG::new(&Language::Fantasy).unwrap()
    } else if matches.get_flag("goblin") {
        RNG::new(&Language::Goblin).unwrap()
    } else if matches.get_flag("roman") {
        RNG::new(&Language::Roman).unwrap()
    } else if matches.get_flag("curse") {
        RNG::new(&Language::Curse).unwrap()
    } else if matches.get_flag("demonic") {
        RNG::new(&Language::Demonic).unwrap()
    } else {
        let my_dialect_type: Language = rand::random();
        RNG::new(&my_dialect_type).unwrap()
    }
    // match matches {
    //     &_ => {
    //         RNG::new(&Language::Elven).unwrap()
    //     }
    // }
    // if matches.contains_id("elven") {
    //     RNG::new(&Language::Elven).unwrap()
    // } else if matches.contains_id("fantasy") {
    //     RNG::new(&Language::Fantasy).unwrap()
    // } else {
    //     let my_dialect_type: Language = rand::random();
    //     RNG::new(&my_dialect_type).unwrap()
    // }
    // if matches.get_flag("e") {
    //     RNG::new_from_file(matches.value_of("raw").unwrap().to_string()).unwrap()
    // } else if matches.is_present("elven") {
    //     RNG::new(&Language::Elven).unwrap()
    // } else if matches.is_present("fantasy") {
    //     RNG::new(&Language::Fantasy).unwrap()
    // } else if matches.is_present("goblin") {
    //     RNG::new(&Language::Goblin).unwrap()
    // } else if matches.is_present("roman") {
    //     RNG::new(&Language::Roman).unwrap()
    // } else if matches.is_present("demonic") {
    //     RNG::new(&Language::Demonic).unwrap_err()
    // } else if matches.is_present("curse") {
    //     RNG::new(&Language::Curse).unwrap()
    // } else {
    //     let my_dialect_type: Language = rand::random();
    //     RNG::new(&my_dialect_type).unwrap()
    // }
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

//
// fn get_matches() -> ArgMatches {
//     command!()
//         .arg("-c, --curse 'Use the Curse language [UNDER CONSTRUCTION]'")
//         .arg("-d, --demonic 'Use the Demonic language [UNDER CONSTRUCTION]'")
//         .arg("-e, --elven 'Use the Elven language'")
//         .arg("-f, --fantasy 'Use the Fantasy language'")
//         .arg("-g, --goblin 'Use the Goblin language'")
//         .arg("-r, --roman 'Use the Roman language'")
//         .arg("--dump 'Print out the raw lanuage file'")
//         .arg("-x, --flipmode 'Use a random language'")
//         .arg("--raw=[FILE] 'reads in a raw language file'")
//         .get_matches()
// }

#[test]
fn verify_cmd() {
    cmd().debug_assert();
}
