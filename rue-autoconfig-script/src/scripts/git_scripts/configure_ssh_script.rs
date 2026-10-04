use std::{env, fs::File, process::Command, process::Stdio};

use crate::scripts::script_error::GitScriptError;

// ssh-keygen -t ed25519 -C
// ssh-add ~/.ssh/id_ed25519
//
pub fn configure_github_shh() -> Result<(), GitScriptError> {
    let home = env::var("HOME").map_err(GitScriptError::HomeNotAvailable)?;

    let github_email = "\"69476182+Rue0612@users.noreply.github.com\"";
    let shh_default_folder = format!("{home}/.ssh/id_ed25519");

    let status = Command::new("ssh-keygen")
        .args(["-t", "ed25519", "-C"])
        .arg(github_email)
        .status()
        .map_err(GitScriptError::CouldNotStart)?;

    if !status.success() {
        return Err(GitScriptError::ComandFailed(status));
    }

    let status = Command::new("ssh-add")
        .arg(shh_default_folder)
        .status()
        .map_err(GitScriptError::CouldNotStart)?;

    if !status.success() {
        return Err(GitScriptError::ComandFailed(status));
    }

    Ok(())
}

// wl-copy < ~/.ssh/id_ed25519.pub
// ssh -T git@github.com (verificar status do ssh)
//
pub fn test_github_ssh_connection() -> Result<(), GitScriptError> {
    let home = env::var("HOME").map_err(GitScriptError::HomeNotAvailable)?;

    let pubkey_path = format!("{home}/.ssh/id_ed25519.pub");
    let pubkey_file = File::open(&pubkey_path).map_err(GitScriptError::CouldNotStart)?;

    let status = Command::new("wl-copy")
        .stdin(Stdio::from(pubkey_file))
        .status()
        .map_err(GitScriptError::CouldNotStart)?;

    if !status.success() {
        return Err(GitScriptError::ComandFailed(status));
    }

    let output = Command::new("ssh")
        .args(["-T", "git@github.com"])
        .output()
        .map_err(GitScriptError::CouldNotStart)?;

    let stderr = String::from_utf8_lossy(&output.stderr);

    if !stderr.contains("successfully authenticated") {
        return Err(GitScriptError::ComandFailed(output.status));
    }

    Ok(())
}
