use crate::scripts::script_error::ScriptError;
use std::process::Command;

pub fn install_aur_apps() -> Result<(), ScriptError> {
    let aur_apps = [
        "proton-cachyos-rtsp-bin", // system (gaming)
        "wayvr",                   // workspace apps
        "bs-manager-git",
    ];

    let status = Command::new("paru")
        .arg("-S")
        .args(aur_apps)
        .status()
        .map_err(ScriptError::CouldNotStart)?;

    if status.success() {
        Ok(())
    } else {
        Err(ScriptError::ComandFailed(status))
    }
}
