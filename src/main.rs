use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let conf = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(err) = minigrep::run(conf) {
        eprintln!("Application error: {err}");
        process::exit(1);
    }
}
