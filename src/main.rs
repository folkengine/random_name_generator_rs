use rnglib::generate_dialect;

use rnglib::rng_dialect::{Dialects};

fn main() {
    let my_dialect_type: Dialects = rand::random();
    let dialect = generate_dialect(&my_dialect_type);

    let first_name = dialect.generate_name();
    let last_name = dialect.generate_name();

    println!("{}: {} {}", dialect.name, first_name, last_name)
}
