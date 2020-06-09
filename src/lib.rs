//! Functions and data structures for parsing IRC standup logs

pub mod cli;
use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
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

    /// When no IRC logs are found
    #[error("No IRC log paths found matching {0}")]
    NoIrcLogPathsFound(String),
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

/// Returns the position of the needle &str in haystack Vec, starting from the
/// right. This is used to find the index of the last standup in an ordered log.
fn rpos_str(haystack: &[&str], needle: &str) -> Result<usize, StandupError> {
    haystack
        .iter()
        .rposition(|l| l.contains(needle))
        .ok_or_else(|| StandupError::StringNotFound(needle.to_string()))
}

/// Find and print the last standup in the irc log, then write it to something.
pub fn print_last_standup(
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
pub fn notes_path(sup_dir_notes: &str, project_code: &str) -> PathBuf {
    Path::new(sup_dir_notes).join(format!("{}.md", project_code))
}

/// Returns a vector of IRC log paths which match the pattern string
pub fn find_irc_log_path(
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_notes_path() {
        assert_eq!(
            notes_path("/foo/bar", "ab001"),
            PathBuf::from("/foo/bar/ab001.md")
        );
        assert_ne!(
            notes_path("/foo/bar", "ab001"),
            PathBuf::from("/foo/bar/ab001.txt")
        );
    }
}
