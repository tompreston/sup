//! An IRC standup parser.

use sup::cli::{StandupCmd, StandupOpt, StructOpt};

/// Perform standup actions
fn run_action(opt: &StandupOpt) -> Result<(), sup::StandupError> {
    match &opt.command {
        StandupCmd::Edit { project_code } => sup::edit(
            opt.editor.as_str(),
            sup::standup_notes_path(opt.sup_dir_notes.as_str(), project_code.as_str()),
        ),
        StandupCmd::Show {
            project_code,
            next_engineer,
        } => sup::show(
            sup::standup_notes_path(opt.sup_dir_notes.as_str(), project_code.as_str()),
            next_engineer.as_str(),
        ),
        StandupCmd::Format { irc_log_pattern } => sup::format(&opt, irc_log_pattern.as_str()),
    }?;
    Ok(())
}

/// Parse the arguments, run the program and return sensible errors.
fn main() {
    let opt = StandupOpt::from_args();
    std::process::exit(match run_action(&opt) {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("sup: error: {}", err);
            1
        }
    });
}
