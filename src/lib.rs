//! Functions and data structures for parsing IRC standup logs

pub mod cli;
use std::fmt;
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

/// Find the last standup in the irc log, then write it to something.
pub fn write_last_standup(
    irc_log: &str,
    start: &str,
    discussion: &str,
    end: &str,
    mut writer: impl io::Write,
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
        writer
            .write(format!("{}\n", line).as_bytes())
            .map_err(StandupError::IO)?;
    }

    Ok(())
}

/// Returns path to standup notes generated from standup dir and project code
///
/// # Example
///
/// ```
/// # use sup::notes_path;
/// # use std::path::PathBuf;
/// let path = notes_path("/foo/bar", "ab001");
/// assert_eq!(path, PathBuf::from("/foo/bar/ab001.md"));
/// ```
pub fn notes_path(sup_dir_notes: &str, project_code: &str) -> PathBuf {
    Path::new(sup_dir_notes).join(format!("{}.md", project_code))
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::{Read, Seek, SeekFrom};

    #[test]
    fn test_irc_log_line() {
        let text = "2020-06-09 16:56:38	tpreston	foobar";
        let irc_log_line: IrcLogLine = text.parse().unwrap();
        assert_eq!(irc_log_line.datetime, "2020-06-09 16:56:38");
        assert_eq!(irc_log_line.username, "tpreston");
        assert_eq!(irc_log_line.content, "foobar");
        assert_eq!(irc_log_line.in_discussion, false);
    }

    #[test]
    fn test_rpos_str() {
        let haystack = ["foo", "bar", "baz", "bar"];
        assert_eq!(rpos_str(&haystack, "foo").ok(), Some(0));
        assert_eq!(rpos_str(&haystack, "bar").ok(), Some(3));
        assert_eq!(rpos_str(&haystack, "baz").ok(), Some(2));
        assert!(rpos_str(&haystack, "bong").is_err());
    }

    #[test]
    fn test_write_last_standup() {
        let irc_log = "\
2020-06-10 10:32:39	john-a	blar blah blar
2020-06-10 10:33:52	john-b	:)
2020-06-10 10:34:21	tpreston	great
2020-06-10 10:34:22	tpreston	standup time!
2020-06-10 10:34:24	tpreston	who is attending?
2020-06-10 10:34:27	john-a	o/
2020-06-10 10:34:28	john-b	o/
2020-06-10 10:35:17	tpreston	order is me, john-a, john-b 
2020-06-10 10:35:29	tpreston	## Thomas Preston (tpreston)
2020-06-10 10:35:29	tpreston	* Done:
2020-06-10 10:35:29	tpreston	  - something
2020-06-10 10:35:29	tpreston	* Doing:
2020-06-10 10:35:29	tpreston	  - something else
2020-06-10 10:35:29	tpreston	* Next:
2020-06-10 10:35:29	tpreston	  - another thing
2020-06-10 10:35:29	tpreston	* Backlog:
2020-06-10 10:35:29	tpreston	  - some thing I'll get around to
2020-06-10 10:35:29	tpreston	## John Ayy (john-a)
2020-06-10 10:35:55	john-a	* Done
2020-06-10 10:35:55	john-a	  - A thing
2020-06-10 10:35:55	john-a	* Today
2020-06-10 10:35:55	john-a	  - This other thing
2020-06-10 10:35:58	john-a	## John Bee (john-b)
2020-06-10 10:36:02	john-b	* Done
2020-06-10 10:36:02	john-b	    - Created the thing
2020-06-10 10:36:02	john-b	    - Built another thing
2020-06-10 10:36:02	john-b	    - Reviewed thingy
2020-06-10 10:36:02	john-b	* Doing
2020-06-10 10:36:04	john-b	    - Add some notes
2020-06-10 10:36:06	john-b	    - Do a doo-hickey
2020-06-10 10:36:08	john-b	    - Create the merge foo
2020-06-10 10:36:10	john-b	# Discussion
2020-06-10 10:36:25	 *	john-a doesn't have much concrete to be working on
2020-06-10 10:36:41	john-a	Other than that, _o_
2020-06-10 10:37:52	john-b	_o_
2020-06-10 10:38:34	tpreston	if no points, closing standup in 5
2020-06-10 10:38:35	tpreston	4
2020-06-10 10:38:35	tpreston	3
2020-06-10 10:38:36	tpreston	2
2020-06-10 10:38:36	tpreston	1
2020-06-10 10:38:38	tpreston	standup ends
2020-06-10 10:38:40	tpreston	thanks all
2020-06-10 10:38:53	john-b	ta tpreston";

        let expected_standup = "
## Thomas Preston (tpreston)
* Done:
  - something
* Doing:
  - something else
* Next:
  - another thing
* Backlog:
  - some thing I'll get around to

## John Ayy (john-a)
* Done
  - A thing
* Today
  - This other thing

## John Bee (john-b)
* Done
    - Created the thing
    - Built another thing
    - Reviewed thingy
* Doing
    - Add some notes
    - Do a doo-hickey
    - Create the merge foo

# Discussion
     *	john-a doesn't have much concrete to be working on
    john-a	Other than that, _o_
    john-b	_o_
    tpreston	if no points, closing standup in 5
    tpreston	4
    tpreston	3
    tpreston	2
    tpreston	1
";
        let start = "## Thomas Preston (tpreston)";
        let discussion = "# Discussion";
        let end = "standup ends";
        let mut out_buffer = io::Cursor::new(vec![0; expected_standup.len()]);

        let res = write_last_standup(&irc_log, start, discussion, end, &mut out_buffer);
        assert!(res.is_ok());

        let mut written_standup = String::new();
        out_buffer
            .seek(SeekFrom::Start(0))
            .expect("Could not seek to start of out_buffer");
        out_buffer
            .read_to_string(&mut written_standup)
            .expect("Could not read out_buffer");

        assert_eq!(written_standup, expected_standup);
    }
}
