use crate::scripts::comand_error_enum::CommandError;
use std::process::Command;

pub fn install_pacman_apps() -> Result<(), CommandError> {
    let pacman_apps = [
        "proton-cachyos-slr",      // system (gaming)
        "ttf-jetbrains-mono-nerd", // system
        "git",
        "gcc",
        "make",
        "ripgrep",
        "fd",
        "tree-sitter-cli",
        "unzip",
        "wl-clipboard",
        "neovim", // workspace apps
        "steam",
        "vesktop-bin",
        "amberol",
    ];

    let status = Command::new("sudo")
        .args(["pacman", "-S", "--needed", "--noconfirm"])
        .args(pacman_apps)
        .status()
        .map_err(CommandError::CouldNotStart)?;

    if status.success() {
        Ok(())
    } else {
        Err(CommandError::ComandFailed(status))
    }
}
