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
    #[structopt(short = "n", long, env)]
    pub sup_dir_notes: String,

    /// The IRC log directory
    #[structopt(short = "l", long, env)]
    pub sup_dir_irc_logs: String,

    /// The begin pattern for standups
    #[structopt(long, env)]
    pub sup_pattern_begin: String,

    /// The discussion pattern for standups
    #[structopt(long, env, default_value = "# Discussion")]
    pub sup_pattern_discussion: String,

    /// The end pattern for standups
    #[structopt(long, env, default_value = "tandup ends")]
    pub sup_pattern_end: String,

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

    /// Scrape and print the last standup from this project's IRC log. Use the
    /// pattern to select the log to scrape. See list command.
    Format {
        #[structopt(default_value = "")]
        irc_log_pattern: String,
    },

    /// Create standup notes for a new project
    New { project_code: String },
}
