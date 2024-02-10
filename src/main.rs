use std::env; //imports the env module
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    //below is the use of the closure for err

    let config = Config::new(&args).unwrap_or_else(|err: &str| {
        eprintln!("problem parsing arguments:{}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("application error:{}", e);
        process::exit(1);
    }
}
