use std::default::Default;
use std::str::FromStr;

use crate::StandupError;

#[derive(Debug)]
struct IrcLogLineWeechat {
    datetime: String,
    username: String,
    content: String,
}

impl FromStr for IrcLogLineWeechat {
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

impl IrcLogLineWeechat {
    fn username(&self) -> &String {
        &self.username
    }

    fn content(&self) -> &String {
        &self.content
    }
}

#[derive(Debug, Default)]
pub struct IrcLog<'a> {
    log: &'a str,
}

impl<'a> IrcLog<'a> {
    pub fn new(log: &'a str) -> Self {
        Self { log }
    }

    /// Find and print the last standup in the log.
    pub fn print_last_standup(
        &self,
        start: &str,
        discussion: &str,
        end: &str,
    ) -> Result<(), StandupError> {
        // Mark the standup pattern locations, starting from the end of the log.
        let lrend = self
            .log
            .lines()
            .rev()
            .position(|l| l.contains(end))
            .ok_or_else(|| StandupError::IrcStandupNotFound(end.to_string()))?;
        let lrdiscussion = self
            .log
            .lines()
            .rev()
            .position(|l| l.contains(discussion))
            .ok_or_else(|| StandupError::IrcStandupNotFound(discussion.to_string()))?;
        let lrstart = self
            .log
            .lines()
            .rev()
            .position(|l| l.contains(start))
            .ok_or_else(|| StandupError::IrcStandupNotFound(start.to_string()))?;

        // Reverse the indexes, to get the real standup position
        let index_last = self.log.lines().count() - 1;
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

        for (i, l) in self.log.lines().enumerate() {
            if i >= lstart && i <= lend {
                let line: IrcLogLineWeechat = l.parse()?;
                if i <= ldiscussion {
                    if line.content().starts_with('#') {
                        println!();
                    }
                    println!("{}", line.content());
                } else {
                    println!("    {}\t{}", line.username(), line.content());
                }
            }
        }

        Ok(())
    }
}
