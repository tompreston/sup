//! The sup command line interface (CLI).
use std::path::PathBuf;
pub use structopt::StructOpt;

/// Options for the sup program
#[derive(StructOpt, Debug)]
#[structopt(about = "An IRC standup log parser")]
pub struct StandupOpt {
    /// The editor we should use to open standup logs.
    #[structopt(short, long, env)]
    pub editor: String,

    /// The directory where the standup logs are kept.
    #[structopt(short, long, env)]
    pub sup_dir: String,

    #[structopt(subcommand)]
    pub command: StandupCmd,
}

#[derive(StructOpt, Debug)]
pub enum StandupCmd {
    /// Edit the standup notes for the given project
    Edit { project_code: String },

    /// Print a project's standup notes and append the next engineer's name
    Show {
        project_code: String,

        /// A search string for the next engineer, or "# Discussion"
        next_engineer: String,
    },

    /// Scrape and print the last standup from this project's IRC log
    Format { project_code: String },

    /// Attempt to push the standup log_path to its wiki
    Push { log_path: PathBuf },

    /// Create standup notes for a new project
    New { project_code: String },
}
