use crate::scripts::script_error::ScriptError;
use std::process::Command;

// git config --global user.name
// git config --global user.email
//
pub fn configure_git_info() -> Result<(), ScriptError> {
    let github_username = "\"Rue Coimbra\"";
    let github_email = "\"69476182+Rue0612@users.noreply.github.com\"";

    let status = Command::new("git")
        .args(["config", "--global", "user.name"])
        .arg(github_username)
        .status()
        .map_err(ScriptError::CouldNotStart)?;

    if !status.success() {
        return Err(ScriptError::ComandFailed(status));
    }

    let status = Command::new("git")
        .args(["config", "--global", "user.email"])
        .arg(github_email)
        .status()
        .map_err(ScriptError::CouldNotStart)?;

    if !status.success() {
        return Err(ScriptError::ComandFailed(status));
    }

    Ok(())
}
