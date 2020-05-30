//! An IRC standup parser.
use std::error::Error;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process;

mod cli;

use cli::{StandupCmd, StandupOpt, StructOpt};

/// Returns path to standup notes generated from standup dir and project code
fn sup_notes_path(sup_dir_notes: &str, project_code: &str) -> PathBuf {
    Path::new(sup_dir_notes).join(format!("{}.md", project_code))
}

/// Open the standup notes for editing
fn edit(editor: &str, spath: PathBuf) -> io::Result<()> {
    process::Command::new(editor).arg(spath).status()?;
    Ok(())
}

/// Show the standup notes, followed by the next_engineer (search string)
fn show(spath: PathBuf, next_engineer: &str) -> io::Result<()> {
    let snotes = fs::read_to_string(spath)?;
    println!("{}", snotes.trim());

    // Now print the next engineer name
    let next_engs: Vec<&str> = snotes
        .lines()
        .filter(|l| l.starts_with('#'))
        .filter(|l| l.contains(next_engineer))
        .collect();
    for e in next_engs.iter() {
        println!("{}", e);
    }

    Ok(())
}

fn list(sup_irc_log_dir: &str) -> io::Result<()> {
    let mut entries = fs::read_dir(sup_irc_log_dir)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;
    entries.sort();
    for e in entries {
        println!("{}", e.to_string_lossy());
    }
    Ok(())
}

/// Returns an IRC log path, which is the fmt with the inserted project code
//fn irc_log_path(project_code: &str) -> PathBuf {}

/// Perform standup actions
fn run_standup_action(opt: &StandupOpt) -> Result<(), Box<dyn Error>> {
    dbg!(opt);

    match &opt.command {
        StandupCmd::Edit { project_code } => edit(
            opt.editor.as_str(),
            sup_notes_path(opt.sup_dir_notes.as_str(), project_code.as_str()),
        ),
        StandupCmd::Show {
            project_code,
            next_engineer,
        } => show(
            sup_notes_path(opt.sup_dir_notes.as_str(), project_code.as_str()),
            next_engineer.as_str(),
        ),
        StandupCmd::List => list(opt.sup_dir_irc_logs.as_str()),
        //StandupCmd::Format { project_code } => format_irc_log(irc_log_path(project_code.as_str())),
        _ => unimplemented!(),
    }?;
    Ok(())
}

/// Parse the arguments, run the program and return sensible errors.
fn main() {
    let opt = StandupOpt::from_args();
    std::process::exit(match run_standup_action(&opt) {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("sup: error: {}", err);
            1
        }
    });
}

#[cfg(test)]
mod test {
    use super::sup_notes_path;
    use std::path::PathBuf;

    #[test]
    fn test_sup_notes_path() {
        assert_eq!(
            sup_notes_path("/foo/bar", "ab001"),
            PathBuf::from("/foo/bar/ab001.md")
        );
        assert_ne!(
            sup_notes_path("/foo/bar", "ab001"),
            PathBuf::from("/foo/bar/ab001.txt")
        );
    }
}
