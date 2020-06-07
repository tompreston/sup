//! Functions and data structures for parsing IRC standup logs

use crate::StandupError;
use std::str::FromStr;

#[derive(Debug)]
struct IrcLogLine {
    datetime: String,
    username: String,
    content: String,
}

impl FromStr for IrcLogLine {
    type Err = StandupError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('\t');
        Ok(Self {
            datetime: split.next().unwrap_or("").to_string(),
            username: split.next().unwrap_or("").to_string(),
            content: split.next().unwrap_or("").to_string(),
        })
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

    // Mark the standup pattern locations, starting from the end of the log.
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

    for (i, l) in irc_log.lines().enumerate() {
        if i >= lstart && i <= lend {
            let line: IrcLogLine = l.parse()?;
            if i <= ldiscussion {
                if line.content.starts_with('#') {
                    println!();
                }
                println!("{}", line.content);
            } else {
                println!("    {}\t{}", line.username, line.content);
            }
        }
    }

    Ok(())
}
