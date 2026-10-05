use crate::scripts::script_error::ScriptError;
use std::process::Command;

pub fn install_pacman_apps() -> Result<(), ScriptError> {
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
        "rustup",
        "neovim", // workspace apps
        "steam",
        "vesktop-bin",
        "amberol",
    ];

    let status = Command::new("sudo")
        .args(["pacman", "-S", "--needed", "--noconfirm"])
        .args(pacman_apps)
        .status()
        .map_err(ScriptError::CouldNotStart)?;

    if status.success() {
        Ok(())
    } else {
        Err(ScriptError::ComandFailed(status))
    }
}
