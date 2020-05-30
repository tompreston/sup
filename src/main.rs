//! An IRC standup parser.
use std::error::Error;
use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process;

mod cli;

use cli::{StandupCmd, StandupOpt, StructOpt};

#[derive(Debug)]
enum StandupError {
    IO(io::Error),
    IrcStandupNotFound(String),
}

impl fmt::Display for StandupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StandupError::IO(e) => write!(f, "IO error: {}", e),
            StandupError::IrcStandupNotFound(s) => write!(f, "IRC standup not found: {}", s),
        }
    }
}

impl Error for StandupError {}

impl From<io::Error> for StandupError {
    fn from(err: io::Error) -> StandupError {
        StandupError::IO(err)
    }
}

/// Returns path to standup notes generated from standup dir and project code
fn sup_notes_path(sup_dir_notes: &str, project_code: &str) -> PathBuf {
    Path::new(sup_dir_notes).join(format!("{}.md", project_code))
}

/// Open the standup notes for editing
fn edit(editor: &str, spath: PathBuf) -> Result<(), StandupError> {
    process::Command::new(editor).arg(spath).status()?;
    Ok(())
}

/// Show the standup notes, followed by the next_engineer (search string)
fn show(spath: PathBuf, next_engineer: &str) -> Result<(), StandupError> {
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

/// Returns a vector of IRC log paths which match the pattern string
fn find_irc_log_path(sup_dir_irc_logs: &str, pattern: &str) -> Result<Vec<PathBuf>, StandupError> {
    Ok(fs::read_dir(sup_dir_irc_logs)?
        .map(|res| res.map(|e| e.path()))
        .filter_map(|res| res.ok())
        .filter(|e| e.to_string_lossy().contains(pattern))
        .collect())
}

/// Format the IRC log path
fn format_irc_log(irc_log_path: &PathBuf) -> Result<(), StandupError> {
    dbg!(irc_log_path);

    let log = fs::read_to_string(irc_log_path)?;
    let log_len = log.lines().count();

    // Find the last standup position (start and end)
    let start_str = "# William";
    let discussion_str = "# Discussion";
    let end_str = "tandup ends";

    let mut log_rlines = log.lines().rev();
    let end = log_len
        - log_rlines
            .position(|l| l.contains(end_str))
            .ok_or(StandupError::IrcStandupNotFound(end_str.to_string()))?;
    let discussion = log_len
        - log_rlines
            .position(|l| l.contains(discussion_str))
            .ok_or(StandupError::IrcStandupNotFound(discussion_str.to_string()))?;
    let start = log_len
        - log_rlines
            .position(|l| l.contains(start_str))
            .ok_or(StandupError::IrcStandupNotFound(start_str.to_string()))?;

    dbg!(start, discussion, end);
    for (i, line) in log.lines().enumerate() {
        if i >= start && i < end {
            let mut split_line = line.split("\t");
            if i <= discussion {
                println!("{}", split_line.nth(2).unwrap_or(""));
            } else {
                println!(
                    "{}\t{}",
                    split_line.nth(1).unwrap_or(""),
                    split_line.next().unwrap_or("")
                );
            }
        }
    }

    Ok(())
}

fn print_irc_logs(logs: Vec<PathBuf>) {
    for l in logs {
        println!("{}", l.to_string_lossy())
    }
}

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
        StandupCmd::Format { pattern } => {
            let mut logs = find_irc_log_path(opt.sup_dir_irc_logs.as_str(), pattern.as_str())?;
            if logs.len() == 0 {
                Ok(println!("No IRC logs found"))
            } else if logs.len() == 1 {
                format_irc_log(&logs[0])
            } else {
                logs.sort();
                Ok(print_irc_logs(logs))
            }
        }
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
