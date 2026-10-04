use crate::scripts::comand_error_enum::CommandError;
use std::process::Command;

pub fn install_flatpack_apps() -> Result<(), CommandError> {
    let flatpak_apps = [
        "io.github.wivrn.wvrn", // workspace apps
        "com.github.tchx84.Flatseal",
        "app.zen_browser.zen",
    ];

    let status = Command::new("flatpak")
        .args(["install", "-y", "flathub"])
        .args(flatpak_apps)
        .status()
        .map_err(CommandError::CouldNotStart)?;

    if status.success() {
        Ok(())
    } else {
        Err(CommandError::ComandFailed(status))
    }
}
