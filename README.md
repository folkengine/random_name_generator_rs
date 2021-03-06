# random_name_generator_rs

[![Build Status](https://api.travis-ci.com/folkengine/random_name_generator_rs.svg?branch=main)](https://travis-ci.com/github/folkengine/random_name_generator_rs)

This is a rust port of the [Ruby port](https://github.com/folkengine/random_name_generator)
of the [Java Random Name Generator library](https://github.com/folkengine/java-random-name-generator).

It generates it's results based upon which [language file](src/languages) is specified.
Currently, the following are supported:

* Elven
* Fantasy
* Goblin
* Roman

The following are in progress:

* Curse
* Demonic

## Using the library

```rust
use rnglib::{RNG, Language};

fn main() {
    let rng = RNG::new(&Language::Elven).unwrap();
    
    let first_name = rng.generate_name();
    let last_name = rng.generate_name();
    
    println!("{}: {} {}", rng.name, first_name, last_name)
}
```

It is possible to control the number of syllables for a generated name:

```rust
use rnglib::{RNG, Language};

fn main() {
    let rng = RNG::new(&Language::Fantasy).unwrap();
    
    let name = rng.generate_name_by_count(3);
}
```

## Running the binary

To get information about the available options, run help.

Using cargo:

```
$> cargo run -- --help
RandomNameGenerator 0.1.0
Christoph <gaoler@electronicpanopticon.com>
Generates random names in various languages

USAGE:
    rng [FLAGS]

FLAGS:
    -c, --curse       Use the Curse language [UNDER CONSTRUCTION]
    -d, --demonic     Use the Demonic language [UNDER CONSTRUCTION]
        --dump        Print out the raw lanuage file
    -e, --elven       Use the Elven language
    -f, --fantasy     Use the Fantasy language
    -x, --flipmode    Use a random language
    -g, --goblin      Use the Goblin language
    -h, --help        Prints help information
    -r, --roman       Use the Roman language
    -V, --version     Prints version information

```

or from the binary:

```
$> rng --help
```

Passing in one of the language flags will generate a name using that Language's source file.

From cargo:

```
$> cargo run -- -g
Goblin: zradogul bargodul
```

From the binary:

```
$> rng -e
Elven: daedar latherdre
```

## Dependencies

* [Bitflags](https://github.com/bitflags/bitflags)
* [Clap](https://github.com/clap-rs/clap)
* [Clippy](https://rust-lang.github.io/rust-clippy/)
* [rust-embed](https://github.com/pyros2097/rust-embed)

## Dev Dependencies

* [Criterion](https://github.com/bheisler/criterion.rs) for benchmarking
  * [Getting Started](https://bheisler.github.io/criterion.rs/book/getting_started.html)
* [Proptest](https://github.com/AltSysrq/proptest) - Hypothesis-like property testing for Rust
* [Rstest](https://github.com/la10736/rstest) - Fixture-based test framework for Rust

## TODO

* Add in Russian support available in the Ruby version.
* Finish Demonic
* Use [clap types](https://github.com/clap-rs/clap/blob/master/examples/12_typed_values.rs) for number of generated words and syllables.

## Further Interest

* [Constructed Languages for Language Geeks](https://www.reddit.com/r/conlangs/)
* [Expletive infixation](https://en.wikipedia.org/wiki/Expletive_infixation)
* [Phonotactics](https://en.wikipedia.org/wiki/Phonotactics)
* [Stanley Unwin](https://en.wikipedia.org/wiki/Stanley_Unwin_(comedian))
* [Vulgar's Atlas](https://www.vulgarlang.com/atlas/)
