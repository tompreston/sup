use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum StandupError {
    IO(io::Error),
    IrcStandupNotFound(String),
    IrcStandupPositionInvalid(usize, usize, usize),
}

impl fmt::Display for StandupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StandupError::IO(e) => write!(f, "IO error: {}", e),
            StandupError::IrcStandupNotFound(s) => write!(f, "IRC standup not found: {}", s),
            StandupError::IrcStandupPositionInvalid(lstart, ldiscussion, lend) => write!(
                f,
                "IRC standup position invalid: lstart {}, ldiscussion {}, lend {}",
                lstart, ldiscussion, lend,
            ),
        }
    }
}

impl Error for StandupError {}

impl From<io::Error> for StandupError {
    fn from(err: io::Error) -> StandupError {
        StandupError::IO(err)
    }
}
