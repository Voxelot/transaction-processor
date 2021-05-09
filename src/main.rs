mod domain;

use clap::{App, Arg};

fn main() {
    let matches = App::new("Simple Payment Engine")
        .version("1.0")
        .author("Brandon K. <brandonkite92@gmail.com>")
        .arg(
            Arg::with_name("TRANSACTIONS_FILE")
                .help("A file containing the transactions")
                .required(true),
        )
        .get_matches();

    let file = matches
        .value_of("TRANSACTIONS_FILE")
        // We shouldn't reach this due to usage of `.required(true)` above
        .expect("No transactions file input provided.");

    println!("{}", file)
}
