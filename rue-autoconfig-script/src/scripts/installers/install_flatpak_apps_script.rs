use crate::scripts::script_error::ScriptError;
use std::process::Command;

pub fn install_flatpack_apps() -> Result<(), ScriptError> {
    let flatpak_apps = [
        "io.github.wivrn.wvrn", // workspace apps
        "com.github.tchx84.Flatseal",
        "app.zen_browser.zen",
    ];

    let status = Command::new("flatpak")
        .args(["install", "-y", "flathub"])
        .args(flatpak_apps)
        .status()
        .map_err(ScriptError::CouldNotStart)?;

    if status.success() {
        Ok(())
    } else {
        Err(ScriptError::ComandFailed(status))
    }
}
