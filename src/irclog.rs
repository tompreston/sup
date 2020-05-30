use std::default::Default;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

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

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let log = fs::read_to_string(&path)?;
        Ok(Self::new(path, log.as_str()))
    }
}
//    let log = fs::read_to_string(irc_log_path)?;
//    let log_len = log.lines().count();
//
//    // Find the last standup position (start and end)
//    let start_str = "# William";
//    let discussion_str = "# Discussion";
//    let end_str = "tandup ends";
//
//    let mut log_rlines = log.lines().rev();
//    let end = log_len
//        - log_rlines
//            .position(|l| l.contains(end_str))
//            .ok_or(StandupError::IrcStandupNotFound(end_str.to_string()))?;
//    let discussion = log_len
//        - log_rlines
//            .position(|l| l.contains(discussion_str))
//            .ok_or(StandupError::IrcStandupNotFound(discussion_str.to_string()))?;
//    let start = log_len
//        - log_rlines
//            .position(|l| l.contains(start_str))
//            .ok_or(StandupError::IrcStandupNotFound(start_str.to_string()))?;
//
//    dbg!(start, discussion, end);
//    for (i, line) in log.lines().enumerate() {
//        if i >= start && i < end {
//            let mut split_line = line.split("\t");
//            if i <= discussion {
//                println!("{}", split_line.nth(2).unwrap_or(""));
//            } else {
//                println!(
//                    "{}\t{}",
//                    split_line.nth(1).unwrap_or(""),
//                    split_line.next().unwrap_or("")
//                );
//            }
//        }
//    }
