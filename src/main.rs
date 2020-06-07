//! An IRC standup parser.
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process;
use thiserror::Error;

mod cli;
mod irclog;

use cli::{StandupCmd, StandupOpt, StructOpt};
use irclog::IrcLog;

#[derive(Error, Debug)]
pub enum StandupError {
    /// When an IO error occurrs
    #[error("IO error, {0}")]
    IO(io::Error),

    /// When an IRC standup is not found
    #[error("IRC standup not found, {0}")]
    IrcStandupNotFound(String),

    /// When the IRC standup position is invalid
    #[error("IRC standup position is invalid, lstart {0}, ldiscussion {0}, lend {0}")]
    IrcStandupPositionInvalid(usize, usize, usize),
}

/// Returns path to standup notes generated from standup dir and project code
fn sup_notes_path(sup_dir_notes: &str, project_code: &str) -> PathBuf {
    Path::new(sup_dir_notes).join(format!("{}.md", project_code))
}

/// Open the standup notes for editing
fn edit(editor: &str, spath: PathBuf) -> Result<(), StandupError> {
    process::Command::new(editor)
        .arg(spath)
        .status()
        .map_err(|e| StandupError::IO(e))?;
    Ok(())
}

/// Show the standup notes, followed by the next_engineer (search string)
fn show(spath: PathBuf, next_engineer: &str) -> Result<(), StandupError> {
    let snotes = fs::read_to_string(spath).map_err(|e| StandupError::IO(e))?;
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
    Ok(fs::read_dir(sup_dir_irc_logs)
        .map_err(|e| StandupError::IO(e))?
        .map(|res| res.map(|e| e.path()))
        .filter_map(|res| res.ok())
        .filter(|e| e.to_string_lossy().contains(pattern))
        .collect())
}

/// Format the IRC log path
fn format_irc_log(opt: &StandupOpt, irc_log_path: &PathBuf) -> Result<(), StandupError> {
    let log_text = fs::read_to_string(&irc_log_path).map_err(|e| StandupError::IO(e))?;
    let irc_log = IrcLog::new(log_text.as_str());
    irc_log.print_last_standup(
        opt.sup_pattern_begin.as_str(),
        opt.sup_pattern_discussion.as_str(),
        opt.sup_pattern_end.as_str(),
    )
}

fn format(opt: &StandupOpt, pattern: &str) -> Result<(), StandupError> {
    let mut lpaths = find_irc_log_path(opt.sup_dir_irc_logs.as_str(), pattern)?;
    if lpaths.is_empty() {
        println!("No IRC log paths found");
        Ok(())
    } else if lpaths.len() == 1 {
        format_irc_log(opt, &lpaths[0])
    } else {
        lpaths.sort();
        for lpath in lpaths {
            println!("{}", lpath.to_string_lossy());
        }
        Ok(())
    }
}

/// Perform standup actions
fn run_standup_action(opt: &StandupOpt) -> Result<(), StandupError> {
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
        StandupCmd::Format { irc_log_pattern } => format(&opt, irc_log_pattern.as_str()),
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
