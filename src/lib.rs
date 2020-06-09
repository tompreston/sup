//! Functions and data structures for parsing IRC standup logs

pub mod cli;
use cli::StandupOpt;
use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process;
use std::str::FromStr;
use thiserror::Error;

/// Some errors which might occur when running sup
#[derive(Error, Debug)]
pub enum StandupError {
    /// When an IO error occurrs
    #[error("IO error, {0}")]
    IO(io::Error),

    /// When a string is not found in a standup log
    #[error("string not found, {0}")]
    StringNotFound(String),

    /// When the IRC standup position is invalid
    #[error("IRC standup position is invalid, lstart {0}, ldiscussion {0}, lend {0}")]
    IrcStandupPositionInvalid(usize, usize, usize),
}

#[derive(Debug)]
struct IrcLogLine {
    datetime: String,
    username: String,
    content: String,
    in_discussion: bool,
}

impl FromStr for IrcLogLine {
    type Err = StandupError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('\t');
        Ok(Self {
            datetime: split.next().unwrap_or("").to_string(),
            username: split.next().unwrap_or("").to_string(),
            content: split.next().unwrap_or("").to_string(),
            in_discussion: false,
        })
    }
}

impl fmt::Display for IrcLogLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.in_discussion {
            write!(f, "    {}\t{}", self.username, self.content)
        } else if self.content.starts_with('#') {
            write!(f, "\n{}", self.content)
        } else {
            write!(f, "{}", self.content)
        }
    }
}

/// Returns the position of the needle &str in haystack Vec, starting from right
fn rpos_str(haystack: &[&str], needle: &str) -> Result<usize, StandupError> {
    haystack
        .iter()
        .rposition(|l| l.contains(needle))
        .ok_or_else(|| StandupError::StringNotFound(needle.to_string()))
}

/// Find and print the last standup in the log.
fn print_last_standup(
    irc_log: &str,
    start: &str,
    discussion: &str,
    end: &str,
) -> Result<(), StandupError> {
    let lines: Vec<&str> = irc_log.lines().collect();
    let lstart = rpos_str(&lines, start)?;
    let ldiscussion = rpos_str(&lines, discussion)?;
    let lend = rpos_str(&lines, end)?;

    let valid_pos: bool = lstart < ldiscussion && ldiscussion < lend;
    if !valid_pos {
        return Err(StandupError::IrcStandupPositionInvalid(
            lstart,
            ldiscussion,
            lend,
        ));
    }

    // Parse the irc_log_lines
    let mut irc_log_lines: Vec<IrcLogLine> = Vec::new();
    for (i, l) in lines.iter().enumerate().skip(lstart).take(lend - lstart) {
        let mut line: IrcLogLine = l.parse()?;
        line.in_discussion = i > ldiscussion;
        irc_log_lines.push(line);
    }

    for line in irc_log_lines {
        println!("{}", line)
    }

    Ok(())
}

/// Returns path to standup notes generated from standup dir and project code
pub fn standup_notes_path(sup_dir_notes: &str, project_code: &str) -> PathBuf {
    Path::new(sup_dir_notes).join(format!("{}.md", project_code))
}

/// Open the standup notes for editing
pub fn edit(editor: &str, spath: PathBuf) -> Result<(), StandupError> {
    process::Command::new(editor)
        .arg(spath)
        .status()
        .map_err(StandupError::IO)?;
    Ok(())
}

/// Show the standup notes, followed by the next_engineer (search string)
pub fn show(spath: PathBuf, next_engineer: &str) -> Result<(), StandupError> {
    let snotes = fs::read_to_string(spath).map_err(StandupError::IO)?;
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
        .map_err(StandupError::IO)?
        .map(|res| res.map(|e| e.path()))
        .filter_map(|res| res.ok())
        .filter(|e| e.to_string_lossy().contains(pattern))
        .collect())
}

/// Format the IRC log path
fn format_irc_log(opt: &StandupOpt, irc_log_path: &PathBuf) -> Result<(), StandupError> {
    let log_text = fs::read_to_string(&irc_log_path).map_err(StandupError::IO)?;
    print_last_standup(
        log_text.as_str(),
        opt.sup_pattern_begin.as_str(),
        opt.sup_pattern_discussion.as_str(),
        opt.sup_pattern_end.as_str(),
    )
}

pub fn format(opt: &StandupOpt, pattern: &str) -> Result<(), StandupError> {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sup_notes_path() {
        assert_eq!(
            standup_notes_path("/foo/bar", "ab001"),
            PathBuf::from("/foo/bar/ab001.md")
        );
        assert_ne!(
            standup_notes_path("/foo/bar", "ab001"),
            PathBuf::from("/foo/bar/ab001.txt")
        );
    }
}
