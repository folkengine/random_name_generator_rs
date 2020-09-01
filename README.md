# random_name_generator_rs

[![Build Status](https://api.travis-ci.com/folkengine/random_name_generator_rs.svg?branch=main)](https://travis-ci.com/github/folkengine/random_name_generator_rs)

## Running the binary

To get information about the available options, run help.

Using cargo:

```
$> cargo run -- --help
RandomNameGenerator 0.0.1
Christoph <gaoler@electronicpanopticon.com>
Generates random names in various languages

USAGE:
    rng [FLAGS]

FLAGS:
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

Passing in one of the language flags will generate a name in using that Language's source file:


```
$> rng -e
Elven: daedar latherdre
```

## Dependencies

* [Bitflags](https://github.com/bitflags/bitflags)
* [Clap](https://github.com/clap-rs/clap)
* [Clippy](https://rust-lang.github.io/rust-clippy/)
* [rust-embed](https://github.com/pyros2097/rust-embed)
