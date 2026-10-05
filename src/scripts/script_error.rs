use std::{env, process::ExitStatus};

#[derive(Debug)]
pub enum ScriptError {
    CouldNotStart(std::io::Error),
    ComandFailed(ExitStatus),
}

#[derive(Debug)]
pub enum GitScriptError {
    CouldNotStart(std::io::Error),
    ComandFailed(ExitStatus),
    HomeNotAvailable(env::VarError),
}

#[derive(Debug)]
pub enum GitShhScriptError {
    CouldNotStart(std::io::Error),
    ComandFailed(ExitStatus),
    HomeNotAvailable(env::VarError),
    Warning(ScriptError),
}
