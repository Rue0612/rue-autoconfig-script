use crate::scripts::script_error::GitScriptError;
use std::process::Command;

// git config --global user.name
// git config --global user.email
//
pub fn configure_git_info() -> Result<(), GitScriptError> {
    let github_username = "\"Rue Coimbra\"";
    let github_email = "\"69476182+Rue0612@users.noreply.github.com\"";

    let status = Command::new("git")
        .args(["config", "--global", "user.name"])
        .arg(github_username)
        .status()
        .map_err(GitScriptError::CouldNotStart)?;

    if !status.success() {
        return Err(GitScriptError::ComandFailed(status));
    }

    let status = Command::new("git")
        .args(["config", "--global", "user.email"])
        .arg(github_email)
        .status()
        .map_err(GitScriptError::CouldNotStart)?;

    if !status.success() {
        return Err(GitScriptError::ComandFailed(status));
    }

    Ok(())
}
