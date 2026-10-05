use crate::scripts::{git_scripts::git_clone_script::git_clone, script_error::GitScriptError};
use std::env;

pub fn configure_fastfatch_with_ssh() -> Result<(), GitScriptError> {
    let home = env::var("HOME").map_err(GitScriptError::HomeNotAvailable)?;
    let where_to_clone = format!("{home}/.config/fastfetch/");

    remove_files(&where_to_clone).map_err(GitScriptError::CouldNotStart)?;

    git_clone(
        "git@github.com:Rue0612/rue-fastfetch-config.git",
        &where_to_clone,
    )
}
