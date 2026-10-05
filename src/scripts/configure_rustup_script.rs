use crate::scripts::script_error::ScriptError;

//rustup default stable
//rustup component add rust-analyzer clippy rustfmt
//
pub fn config_rustup() -> Result<(), ScriptError> {
    let rustup_components = ["rust-analyzer", "clippy", "rustfmt"];

    let status = Command::new("rustup")
        .args(["default", "stable"])
        .status()
        .map_err(ScriptError::CouldNotStart)?;

    if !status.success() {
        return Err(ScriptError::ComandFailed(status));
    }

    let status = Command::new("rustup")
        .args(["component", "add"])
        .args(rustup_components)
        .status()
        .map_err(ScriptError::CouldNotStart)?;

    if !status.success() {
        return Err(ScriptError::ComandFailed(status));
    }

    Ok(())
}
