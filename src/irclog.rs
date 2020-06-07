//! Functions and data structures for parsing IRC standup logs

use crate::StandupError;
use std::fmt;
use std::str::FromStr;

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

/// Find and print the last standup in the log.
pub fn print_last_standup(
    irc_log: &str,
    start: &str,
    discussion: &str,
    end: &str,
) -> Result<(), StandupError> {
    // Read all of the lines, rposition requires std::iter::ExactSizeIterator
    let lines: Vec<&str> = irc_log.lines().collect();

    // Mark the standup pattern locations, starting from the end of the log
    let lstart = lines
        .iter()
        .rposition(|l| l.contains(start))
        .ok_or_else(|| StandupError::IrcStandupNotFound(start.to_string()))?;
    let ldiscussion = lines
        .iter()
        .rposition(|l| l.contains(discussion))
        .ok_or_else(|| StandupError::IrcStandupNotFound(discussion.to_string()))?;
    let lend = lines
        .iter()
        .rposition(|l| l.contains(end))
        .ok_or_else(|| StandupError::IrcStandupNotFound(end.to_string()))?;

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

    // Print the irc_log_lines
    for line in irc_log_lines {
        println!("{}", line)
    }

    Ok(())
}
