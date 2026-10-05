use crate::{
    scripts::{git_scripts::git_clone_script::git_clone, script_error::GitScriptError},
    util::remove_files_util::remove_files,
};
use std::env;

pub fn configure_alacritty_with_shh() -> Result<(), GitScriptError> {
    let home = env::var("HOME").map_err(GitScriptError::HomeNotAvailable)?;
    let where_to_clone = format!("{home}/.config/alacritty/");

    remove_files(&where_to_clone).map_err(GitScriptError::CouldNotStart)?;

    git_clone(
        "git@github.com:Rue0612/rue-alacritty-config.git",
        &where_to_clone,
    )
}
