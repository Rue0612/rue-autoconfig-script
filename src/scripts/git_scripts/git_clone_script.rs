use crate::scripts::script_error::GitScriptError;
use std::process::Command;

pub(super) fn git_clone(repository: &str, where_to_clone: &str) -> Result<(), GitScriptError> {
    let status = Command::new("git")
        .arg("clone")
        .arg(repository)
        .arg(where_to_clone)
        .status()
        .map_err(GitScriptError::CouldNotStart)?;

    if status.success() {
        Ok(())
    } else {
        Err(GitScriptError::ComandFailed(status))
    }
}
