use crate::{
    scripts::{git_scripts::git_clone_script::git_clone, script_error::GitScriptError},
    util::remove_files_util::remove_files,
};
use std::env;

pub fn configure_nvim_with_ssh() -> Result<(), GitScriptError> {
    let home = env::var("HOME").map_err(GitScriptError::HomeNotAvailable)?;
    let where_to_clone = format!("{home}/.config/nvim/");

    remove_files(&where_to_clone).map_err(GitScriptError::CouldNotStart)?;

    git_clone("git@github.com:Rue0612/kickstart.nvim.git", &where_to_clone)
}
