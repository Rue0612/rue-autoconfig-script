use std::process::ExitStatus;

#[derive(Debug)]
pub enum CommandError {
    CouldNotStart(std::io::Error),
    ComandFailed(ExitStatus),
}
