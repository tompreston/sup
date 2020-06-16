//! An IRC standup parser.

use std::fs;
use std::path::PathBuf;
use std::process;
use sup::cli::{StandupCmd, StandupOpt, StructOpt};
use sup::StandupError;

/// Open the standup notes for editing
fn edit(editor: &str, spath: PathBuf) -> Result<(), StandupError> {
    process::Command::new(editor)
        .arg(spath)
        .status()
        .map_err(StandupError::IO)?;
    Ok(())
}

/// Show the standup notes, followed by the next_engineer (search string)
fn show(spath: PathBuf, next_engineer: &str) -> Result<(), StandupError> {
    let snotes = fs::read_to_string(spath).map_err(StandupError::IO)?;
    println!("{}", snotes.trim());

    // Now print the next engineer name
    snotes
        .lines()
        .filter(|l| l.starts_with('#'))
        .filter(|l| l.contains(next_engineer))
        .collect::<Vec<&str>>()
        .iter()
        .for_each(|e| println!("{}", e));
    Ok(())
}

/// Returns a vector of IRC log PathBuf's which match the pattern string
fn filter_irc_log_pathbufs(
    sup_dir_irc_logs: &str,
    pattern: &str,
) -> Result<Vec<PathBuf>, StandupError> {
    Ok(fs::read_dir(sup_dir_irc_logs)
        .map_err(StandupError::IO)?
        .map(|res| res.map(|e| e.path()))
        .filter_map(|res| res.ok())
        .filter(|e| e.to_string_lossy().contains(pattern))
        .collect())
}

/// Search for the IRC log matching pattern, then print the last IRC standup
fn format(opt: &StandupOpt, pattern: &str) -> Result<(), sup::StandupError> {
    let mut lpaths = filter_irc_log_pathbufs(opt.sup_dir_irc_logs.as_str(), pattern)?;
    if lpaths.is_empty() {
        return Err(sup::StandupError::NoIrcLogPathsFound(pattern.to_string()));
    }

    if lpaths.len() == 1 {
        let log_text = fs::read_to_string(&lpaths[0]).map_err(StandupError::IO)?;
        sup::write_last_standup(
            log_text.as_str(),
            opt.sup_pattern_begin.as_str(),
            opt.sup_pattern_discussion.as_str(),
            opt.sup_pattern_end.as_str(),
            std::io::stdout(),
        )
    } else {
        lpaths.sort();
        for lpath in lpaths {
            println!("{}", lpath.to_string_lossy());
        }
        Ok(())
    }
}

/// Perform standup actions
fn run_action(opt: &StandupOpt) -> Result<(), sup::StandupError> {
    match &opt.command {
        StandupCmd::Edit { project_code } => edit(
            opt.editor.as_str(),
            sup::notes_path(opt.sup_dir_notes.as_str(), project_code.as_str()),
        ),
        StandupCmd::Show {
            project_code,
            next_engineer,
        } => show(
            sup::notes_path(opt.sup_dir_notes.as_str(), project_code.as_str()),
            next_engineer.as_str(),
        ),
        StandupCmd::Format { irc_log_pattern } => format(&opt, irc_log_pattern.as_str()),
    }?;
    Ok(())
}

/// Parse the arguments, run the program and return sensible errors.
fn main() {
    let opt = StandupOpt::from_args();
    std::process::exit(match run_action(&opt) {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("sup: error: {}", err);
            1
        }
    });
}
