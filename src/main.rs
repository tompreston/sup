//! An IRC standup parser.
use std::error::Error;
use std::io;
use std::path::Path;
use std::process;

mod cli;

use cli::{Opt, StructOpt};

fn edit(editor: &str, sup_dir: &str, project_code: &str) -> io::Result<process::ExitStatus> {
    let sfilename = format!("{}.md", project_code);
    let spath = Path::new(sup_dir).join(sfilename);
    dbg!(editor, spath.to_str().unwrap());
    process::Command::new(editor).arg(spath).status()
}

/// Perform standup actions
fn run_standup_action(opt: &Opt) -> Result<(), Box<dyn Error>> {
    dbg!(opt);
    match &opt.command {
        cli::Command::Edit { project_code } => edit(
            opt.editor.as_str(),
            opt.sup_dir.as_str(),
            project_code.as_str(),
        ),
        _ => unimplemented!(),
    }?;
    Ok(())
}

/// Parse the arguments, run the program and return sensible errors.
fn main() {
    let opt = Opt::from_args();
    std::process::exit(match run_standup_action(&opt) {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("sup: error: {}", err);
            1
        }
    });
}
