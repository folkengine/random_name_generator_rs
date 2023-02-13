# random_name_generator

[![Crates.io](https://img.shields.io/crates/v/random_name_generator?style=flat-square)](https://crates.io/crates/random_name_generator)
[![Crates.io](https://img.shields.io/crates/d/random_name_generator?style=flat-square)](https://crates.io/crates/random_name_generator)
[![License](https://img.shields.io/badge/License-BSD%203--Clause-blue.svg)](https://opensource.org/licenses/BSD-3-Clause)
[![Build and Test](https://github.com/folkengine/random_name_generator_rs/actions/workflows/CI.yaml/badge.svg)](https://github.com/folkengine/random_name_generator_rs/actions/workflows/CI.yaml)

This is a rust port of the [Ruby port](https://github.com/folkengine/random_name_generator)
of the [Java Random Name Generator library](https://github.com/folkengine/java-random-name-generator).

It generates it's results based upon which [language file](src/languages) is specified.
Currently, the following are supported in both English and Russian:

* Elven
* Fantasy
* Goblin
* Roman

_Thanks to [Zhumatiy Sergey](https://github.com/zhum), for his contribution of the Russian versions of the language
files, originally in the [Ruby version](https://github.com/folkengine/random_name_generator) of this library!_

The following are in progress:

* Curse
* Demonic

## Using the library

```rust
use rnglib::{RNG, Language};

fn main() {
    let rng = RNG::try_from(&Language::Elven).unwrap();
    
    let first_name = rng.generate_name();
    let last_name = rng.generate_name();
    
    println!("{}: {} {}", rng.name, first_name, last_name)
}
```

It is possible to control the number of syllables for a generated name:

```rust
use rnglib::{RNG, Language};

fn main() {
    let rng = RNG::try_from(&Language::Fantasy).unwrap();
    let name = rng.generate_name_by_count(3);
    println!("{}: {}", rng.name, name)
}
```

One can also pass in custom language files:

```
use rnglib::{RNG};

fn main() {
    let rng = RNG::new_from_file("src/languages/Test-tiny.txt").unwrap();
    let name = rng.generate_name();
    println!("{}: {}", rng.name, name)
}
```

## Running the binary

To get information about the available options, run help.

Using cargo:

```
$> cargo run -- --help
Random Name Generator

Usage: rng [OPTIONS]

Options:
  -e, --elven       
  -f, --fantasy     
  -g, --goblin      
  -r, --roman       
  -c, --curse       [UNDER CONSTRUCTION]
  -x, --flipmode    Flipmode is the greatest! (Random language)
      --russian     Use Russian language file, if available
      --raw <FILE>  Reads in a raw language file
  -h, --help        Print help
  -V, --version     Print version

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

It also now supports Russian language files:

```
❯ cargo run -- --roman --russian
Римский: Дафрибуцио Дубенус

❯ cargo run -- --goblin --russian
Гоблин: Краог Зрашагул

❯ cargo run -- --fantasy --russian
Фантазия: Валорнен Гарлакот

❯ cargo run -- --elven --russian
Эльфийский: Латэнаэлмасан Шелиан
```

From the binary:

```
$> rng -e
Elven: daedar latherdre
```

## Dependencies

* [Anyhow](https://github.com/dtolnay/anyhow)
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

* Finish Demonic

## Further Interest

* [Constructed Languages for Language Geeks](https://www.reddit.com/r/conlangs/)
* [Expletive infixation](https://en.wikipedia.org/wiki/Expletive_infixation)
* [Phonotactics](https://en.wikipedia.org/wiki/Phonotactics)
* [Stanley Unwin](https://en.wikipedia.org/wiki/Stanley_Unwin_(comedian))
* [Vulgar's Atlas](https://www.vulgarlang.com/atlas/)
