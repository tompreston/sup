//! An IRC standup parser.
use std::error::Error;

mod cli;

use cli::{StructOpt, SupOpt};

/// Perform standup actions
fn run_sup(opt: SupOpt) -> Result<(), Box<dyn Error>> {
    println!("{:#?}", opt);
    Ok(())
}

/// Parse the arguments, run the program and return sensible errors.
fn main() {
    let opt = SupOpt::from_args();
    std::process::exit(match run_sup(opt) {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("sup: error: {}", err);
            1
        }
    });
}
