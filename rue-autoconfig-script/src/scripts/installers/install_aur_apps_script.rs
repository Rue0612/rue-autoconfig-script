use crate::scripts::comand_error_enum::CommandError;
use std::process::Command;

pub fn install_aur_apps() -> Result<(), CommandError> {
    let aur_apps = [
        "proton-cachyos-rtsp-bin", // system (gaming)
        "wayvr",                   // workspace apps
        "bs-manager-git",
    ];

    let status = Command::new("sudo")
        .args(["paru", "-S"])
        .args(aur_apps)
        .status()
        .map_err(CommandError::CouldNotStart)?;

    if status.success() {
        Ok(())
    } else {
        Err(CommandError::ComandFailed(status))
    }
}
