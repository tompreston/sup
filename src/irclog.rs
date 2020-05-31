use std::default::Default;
use std::fs;
use std::path::{Path, PathBuf};

use crate::standup_error::StandupError;

#[derive(Debug)]
struct IrcLogLineWeechat {
    datetime: String,
    username: String,
    content: String,
}

impl IrcLogLineWeechat {
    pub fn loglines_from_str(s: &str) -> Vec<Self> {
        let mut loglines: Vec<Self> = Vec::new();

        for l in s.lines() {
            let mut split = l.split("\t");
            loglines.push(IrcLogLineWeechat {
                datetime: split.next().unwrap_or("").to_string(),
                username: split.next().unwrap_or("").to_string(),
                content: split.next().unwrap_or("").to_string(),
            });
        }

        loglines
    }

    fn username(&self) -> &String {
        &self.username
    }

    fn content(&self) -> &String {
        &self.content
    }
}

#[derive(Debug, Default)]
pub struct IrcLogWeechat {
    lines: Vec<IrcLogLineWeechat>,
    path: PathBuf,
}

impl IrcLogWeechat {
    pub fn new<P: AsRef<Path>>(path: P, log: &str) -> Self {
        Self {
            lines: IrcLogLineWeechat::loglines_from_str(log),
            path: path.as_ref().to_path_buf(),
        }
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, StandupError> {
        let log = fs::read_to_string(&path)?;
        Ok(Self::new(path, log.as_str()))
    }

    /// Find and print the last standup in the log.
    pub fn print_last_standup(
        &self,
        start: &str,
        discussion: &str,
        end: &str,
    ) -> Result<(), StandupError> {
        // Mark the standup pattern locations, starting from the end of the log.
        let mut log_rlines = self.lines.iter().rev();
        let lrend = log_rlines
            .position(|l| l.content.contains(end))
            .ok_or(StandupError::IrcStandupNotFound(end.to_string()))?;
        let lrdiscussion = log_rlines
            .position(|l| l.content.contains(discussion))
            .ok_or(StandupError::IrcStandupNotFound(discussion.to_string()))?;
        let lrstart = log_rlines
            .position(|l| l.content.contains(start))
            .ok_or(StandupError::IrcStandupNotFound(start.to_string()))?;

        // Reverse the indexes, to get the real standup position
        let index_last = self.lines.len() - 1;
        let lstart = index_last - lrstart;
        let ldiscussion = index_last - lrdiscussion;
        let lend = index_last - lrend;

        let valid_pos: bool = lstart < ldiscussion && ldiscussion < lend;
        if !valid_pos {
            return Err(StandupError::IrcStandupPositionInvalid(
                lstart,
                ldiscussion,
                lend,
            ));
        }

        dbg!(lstart, ldiscussion, lend);
        for (i, line) in self.lines.iter().enumerate() {
            if i < lstart || i > lend {
                continue;
            } else if i <= ldiscussion {
                println!("{}", line.content());
            } else {
                println!("{}\t{}", line.username(), line.content());
            }
        }

        Ok(())
    }
}
