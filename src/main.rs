mod scripts;
mod util;

use std::io;

use crate::{
    scripts::{
        git_scripts::{
            configure_alacritty_script::configure_alacritty_with_shh,
            configure_fastfetch_script::configure_fastfatch_with_ssh,
            configure_fish_script::configure_fish_script_with_ssh,
            configure_git_info_script::configure_git_info,
            configure_niri_script::{configure_niri, configure_niri_with_ssh},
            configure_nvim_script::configure_nvim_with_ssh,
            configure_ssh_script::{configure_git_shh, copy_git_ssh, test_github_ssh_connection},
        },
        installers::{
            install_aur_apps_script::install_aur_apps,
            install_flatpak_apps_script::install_flatpack_apps,
            install_pacman_apps_script::install_pacman_apps,
        },
        script_error::{GitScriptError, GitShhScriptError, ScriptError},
    },
    util::confirm_dialog_util::{ConfirmDialogResponse, confirm_dialog},
};

fn main() {
    println!("Hello There !!!!!");
    println!("Soooooooo :3");
    println!("It seems that you are trying to format your computer!!!!");
    println!("Welllllllll");
    println!("Do you wish to continuee? Or was this a missunderstanding? misu misu misurantangi?");

    if confirm_dialog(ConfirmDialogResponse::Yes) == ConfirmDialogResponse::No {
        return;
    }

    println!("\n\nNoowwww that that is settled, let get this going shaw weeee? 0w0");

    println!("\n\nDo you want to install the pacman packages??");
    if confirm_dialog(ConfirmDialogResponse::Yes) == ConfirmDialogResponse::Yes {
        loop {
            match install_pacman_apps() {
                Err(ScriptError::CouldNotStart(e)) => {
                    println!(
                        "An Error ocurred! It seems that pacman wasnt able to start.... Maybe an SUDO error?"
                    );
                    println!("{e}");
                    println!("Would you like to try again?");
                    if confirm_dialog(ConfirmDialogResponse::Yes) == ConfirmDialogResponse::No {
                        break;
                    }
                }
                Err(ScriptError::ComandFailed(e)) => {
                    println!(
                        "An Error ocurred! It seems that pacman wasnt able to download... Maybe an app doenst exist anymore or it has a typo?"
                    );
                    println!("{e}");
                    println!("Would you like to try again?");
                    if confirm_dialog(ConfirmDialogResponse::Yes) == ConfirmDialogResponse::No {
                        break;
                    }
                }
                Ok(_) => {
                    println!("All clear!! Pacman apps are dowloaded!");
                    break;
                }
            };
        }
    }

    println!("\n\nAll done! Now, do you want to install the flatpack packages? ^-^");
    if confirm_dialog(ConfirmDialogResponse::Yes) == ConfirmDialogResponse::Yes {
        loop {
            match install_flatpack_apps() {
                Err(ScriptError::CouldNotStart(e)) => {
                    println!(
                        "An Error ocurred! It seems that flatpack wasnt able to start.... Waaaaa?"
                    );
                    println!("{e}");
                    println!("Would you like to try again?");
                    if confirm_dialog(ConfirmDialogResponse::Yes) == ConfirmDialogResponse::No {
                        break;
                    }
                }
                Err(ScriptError::ComandFailed(e)) => {
                    println!(
                        "An Error ocurred! It seems that flatpack wasnt able to download... Maybe an app doenst exist anymore or it has a typo?"
                    );
                    println!("{e}");
                    println!("Would you like to try again?");
                    if confirm_dialog(ConfirmDialogResponse::Yes) == ConfirmDialogResponse::No {
                        break;
                    }
                }
                Ok(_) => {
                    println!("All clear!! Flatpack apps are dowloaded!");
                    break;
                }
            };
        }
    }

    println!("\n\nAll done! Now, do you want to install the FORBIDEN AUR PACKAGES? 0_0");
    if confirm_dialog(ConfirmDialogResponse::Yes) == ConfirmDialogResponse::Yes {
        loop {
            match install_aur_apps() {
                Err(ScriptError::CouldNotStart(e)) => {
                    println!(
                        "An Error ocurred! It seems that paru wasnt able to start.... Waaaaa?"
                    );
                    println!("{e}");
                    println!("Would you like to try again?");
                    if confirm_dialog(ConfirmDialogResponse::Yes) == ConfirmDialogResponse::No {
                        break;
                    }
                }
                Err(ScriptError::ComandFailed(e)) => {
                    println!(
                        "An Error ocurred! It seems that paru wasnt able to download... Maybe an app doenst exist anymore or it has a typo?"
                    );
                    println!("{e}");
                    println!("Would you like to try again?");
                    if confirm_dialog(ConfirmDialogResponse::Yes) == ConfirmDialogResponse::No {
                        break;
                    }
                }
                Ok(_) => {
                    println!("All clear!! All PARU apps are dowloaded!");
                    break;
                }
            };
        }
    }

    println!("\n\nAll done!!!!!!! Now, lets start configuring your computer!!!");
    println!("In some cases, you keyboart might be a little bit buggy, want me to solve it? :)");

    if confirm_dialog(ConfirmDialogResponse::Yes) == ConfirmDialogResponse::Yes {
        loop {
            match configure_niri() {
                Err(GitScriptError::HomeNotAvailable(e)) => {
                    println!("It seem that there is no home for me... Im kinda homelesss......");
                    println!("{e}");
                    println!("Would you like to try again?");
                    if confirm_dialog(ConfirmDialogResponse::Yes) == ConfirmDialogResponse::No {
                        break;
                    }
                }
                Err(GitScriptError::CouldNotStart(e)) => {
                    println!("An Error ocurred! It seems that git wasnt able to start.... Waaaaa?");
                    println!("{e}");
                    println!("Would you like to try again?");
                    if confirm_dialog(ConfirmDialogResponse::Yes) == ConfirmDialogResponse::No {
                        break;
                    }
                }
                Err(GitScriptError::ComandFailed(e)) => {
                    println!(
                        "An Error ocurred! It seems that git wasnt able to download... Maybe the repo doenst exist anymore or it has a typo?"
                    );
                    println!("{e}");
                    println!("Would you like to try again?");
                    if confirm_dialog(ConfirmDialogResponse::Yes) == ConfirmDialogResponse::No {
                        break;
                    }
                }
                Ok(_) => {
                    println!("All clear!! Niri configured!");
                    break;
                }
            }
        }
    }

    println!("\n\nAll done!!!! Now, do you want to configure ssh?");
    let mut is_github_configured = true;
    if confirm_dialog(ConfirmDialogResponse::Yes) == ConfirmDialogResponse::Yes {
        loop {
            let is_gitinfo_configured = match configure_git_info() {
                Err(ScriptError::CouldNotStart(e)) => {
                    println!("An Error ocurred! It seems that git wasnt able to start.... Waaaaa?");
                    println!("{e}");
                    println!("Would you like to try again?");
                    if confirm_dialog(ConfirmDialogResponse::Yes) == ConfirmDialogResponse::No {
                        break;
                    }
                    false
                }
                Err(ScriptError::ComandFailed(e)) => {
                    println!(
                        "An Error ocurred! It seems that git wasnt able to execute the command... Waaaaaaa?"
                    );
                    println!("{e}");
                    println!("Would you like to try again?");
                    if confirm_dialog(ConfirmDialogResponse::Yes) == ConfirmDialogResponse::No {
                        break;
                    }
                    false
                }
                Ok(_) => {
                    println!("All clear!! Git info configured!");
                    true
                }
            };

            let is_shh_configured = if is_gitinfo_configured {
                println!("\nNow, lets create a new shh!");
                match configure_git_shh() {
                    Err(GitShhScriptError::HomeNotAvailable(e)) => {
                        println!("It seems that HOME is not available...");
                        println!("{e}");
                        println!("Would you like to try again?");
                        if confirm_dialog(ConfirmDialogResponse::Yes) == ConfirmDialogResponse::No {
                            break;
                        }
                        false
                    }
                    Err(GitShhScriptError::CouldNotStart(e)) => {
                        println!("It seems that the shh command could not start...");
                        println!("{e}");
                        println!("Would you like to try again?");
                        if confirm_dialog(ConfirmDialogResponse::Yes) == ConfirmDialogResponse::No {
                            break;
                        }
                        false
                    }
                    Err(GitShhScriptError::ComandFailed(e)) => {
                        println!("It seems that the shh command coud not execute...");
                        println!("{e}");
                        println!("Would you like to try again?");
                        if confirm_dialog(ConfirmDialogResponse::Yes) == ConfirmDialogResponse::No {
                            break;
                        }
                        false
                    }
                    Err(GitShhScriptError::Warning(e)) => {
                        println!("Warning, connection to shh client not found");
                        println!("{e:?}");
                        true
                    }
                    Ok(_) => {
                        println!("All clear!! Shh created!");
                        true
                    }
                }
            } else {
                false
            };

            let is_shh_copyed = if is_shh_configured {
                println!("\nNow, lets copy the shh!");
                match copy_git_ssh() {
                    Err(GitScriptError::HomeNotAvailable(e)) => {
                        println!("It seems that there is no HOME available...");
                        println!("{e}");
                        println!("Would you like to try again?");
                        if confirm_dialog(ConfirmDialogResponse::Yes) == ConfirmDialogResponse::No {
                            break;
                        }
                        false
                    }
                    Err(GitScriptError::ComandFailed(e)) => {
                        println!("It seems that the command couldnt execute...");
                        println!("{e}");
                        println!("Would you like to try again?");
                        if confirm_dialog(ConfirmDialogResponse::Yes) == ConfirmDialogResponse::No {
                            break;
                        }
                        false
                    }
                    Err(GitScriptError::CouldNotStart(e)) => {
                        println!("It seems that the command couldnt start...");
                        println!("{e}");
                        println!("Would you like to try again?");
                        if confirm_dialog(ConfirmDialogResponse::Yes) == ConfirmDialogResponse::No {
                            break;
                        }
                        false
                    }
                    Ok(_) => {
                        println!("All clear!! Shh copyed!");
                        true
                    }
                }
            } else {
                false
            };

            is_github_configured = if is_shh_copyed {
                println!("\n\nThe shh key is on your clipboard!!!!!");
                println!(
                    "\nNow, go to https://github.com/settings/keys and configure your key!!!!"
                );
                println!("When done, please go back so we can test if the connection is right!");
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap_or_default();

                println!("Testing conection!!!");
                match test_github_ssh_connection() {
                    Err(ScriptError::ComandFailed(e)) => {
                        println!("It seems that the command couldnt execute...");
                        println!("{e}");
                        println!("Would you like to try again?");
                        if confirm_dialog(ConfirmDialogResponse::Yes) == ConfirmDialogResponse::No {
                            break;
                        }
                        false
                    }
                    Err(ScriptError::CouldNotStart(e)) => {
                        println!("It seems that the command couldnt start...");
                        println!("{e}");
                        println!("Would you like to try again?");
                        if confirm_dialog(ConfirmDialogResponse::Yes) == ConfirmDialogResponse::No {
                            break;
                        }
                        false
                    }
                    Ok(_) => true,
                }
            } else {
                false
            };

            if is_github_configured {
                println!("All done!!!!!!!!!");
                break;
            }
        }
    }

    if is_github_configured {
        println!("Wanna configure all related to github now?");
        if confirm_dialog(ConfirmDialogResponse::Yes) == ConfirmDialogResponse::Yes {
            loop {
                match configure_niri_with_ssh() {
                    Err(GitScriptError::HomeNotAvailable(e)) => {
                        println!(
                            "It seem that there is no home for me... Im kinda homelesss......"
                        );
                        println!("{e}");
                        println!("Would you like to try again?");
                        if confirm_dialog(ConfirmDialogResponse::Yes) == ConfirmDialogResponse::No {
                            break;
                        }
                    }
                    Err(GitScriptError::CouldNotStart(e)) => {
                        println!(
                            "An Error ocurred! It seems that git wasnt able to start.... Waaaaa?"
                        );
                        println!("{e}");
                        println!("Would you like to try again?");
                        if confirm_dialog(ConfirmDialogResponse::Yes) == ConfirmDialogResponse::No {
                            break;
                        }
                    }
                    Err(GitScriptError::ComandFailed(e)) => {
                        println!(
                            "An Error ocurred! It seems that git wasnt able to download... Maybe the repo doenst exist anymore or it has a typo?"
                        );
                        println!("{e}");
                        println!("Would you like to try again?");
                        if confirm_dialog(ConfirmDialogResponse::Yes) == ConfirmDialogResponse::No {
                            break;
                        }
                    }
                    Ok(_) => {
                        println!("All clear!! Niri configured!");
                        break;
                    }
                }
            }
            loop {
                match configure_fish_script_with_ssh() {
                    Err(GitScriptError::HomeNotAvailable(e)) => {
                        println!(
                            "It seem that there is no home for me... Im kinda homelesss......"
                        );
                        println!("{e}");
                        println!("Would you like to try again?");
                        if confirm_dialog(ConfirmDialogResponse::Yes) == ConfirmDialogResponse::No {
                            break;
                        }
                    }
                    Err(GitScriptError::CouldNotStart(e)) => {
                        println!(
                            "An Error ocurred! It seems that git wasnt able to start.... Waaaaa?"
                        );
                        println!("{e}");
                        println!("Would you like to try again?");
                        if confirm_dialog(ConfirmDialogResponse::Yes) == ConfirmDialogResponse::No {
                            break;
                        }
                    }
                    Err(GitScriptError::ComandFailed(e)) => {
                        println!(
                            "An Error ocurred! It seems that git wasnt able to download... Maybe the repo doenst exist anymore or it has a typo?"
                        );
                        println!("{e}");
                        println!("Would you like to try again?");
                        if confirm_dialog(ConfirmDialogResponse::Yes) == ConfirmDialogResponse::No {
                            break;
                        }
                    }
                    Ok(_) => {
                        println!("All clear!! Fish configured!");
                        break;
                    }
                }
            }
            loop {
                match configure_fastfatch_with_ssh() {
                    Err(GitScriptError::HomeNotAvailable(e)) => {
                        println!(
                            "It seem that there is no home for me... Im kinda homelesss......"
                        );
                        println!("{e}");
                        println!("Would you like to try again?");
                        if confirm_dialog(ConfirmDialogResponse::Yes) == ConfirmDialogResponse::No {
                            break;
                        }
                    }
                    Err(GitScriptError::CouldNotStart(e)) => {
                        println!(
                            "An Error ocurred! It seems that git wasnt able to start.... Waaaaa?"
                        );
                        println!("{e}");
                        println!("Would you like to try again?");
                        if confirm_dialog(ConfirmDialogResponse::Yes) == ConfirmDialogResponse::No {
                            break;
                        }
                    }
                    Err(GitScriptError::ComandFailed(e)) => {
                        println!(
                            "An Error ocurred! It seems that git wasnt able to download... Maybe the repo doenst exist anymore or it has a typo?"
                        );
                        println!("{e}");
                        println!("Would you like to try again?");
                        if confirm_dialog(ConfirmDialogResponse::Yes) == ConfirmDialogResponse::No {
                            break;
                        }
                    }
                    Ok(_) => {
                        println!("All clear!! Fastfatch configured!");
                        break;
                    }
                }
            }
            loop {
                match configure_nvim_with_ssh() {
                    Err(GitScriptError::HomeNotAvailable(e)) => {
                        println!(
                            "It seem that there is no home for me... Im kinda homelesss......"
                        );
                        println!("{e}");
                        println!("Would you like to try again?");
                        if confirm_dialog(ConfirmDialogResponse::Yes) == ConfirmDialogResponse::No {
                            break;
                        }
                    }
                    Err(GitScriptError::CouldNotStart(e)) => {
                        println!(
                            "An Error ocurred! It seems that git wasnt able to start.... Waaaaa?"
                        );
                        println!("{e}");
                        println!("Would you like to try again?");
                        if confirm_dialog(ConfirmDialogResponse::Yes) == ConfirmDialogResponse::No {
                            break;
                        }
                    }
                    Err(GitScriptError::ComandFailed(e)) => {
                        println!(
                            "An Error ocurred! It seems that git wasnt able to download... Maybe the repo doenst exist anymore or it has a typo?"
                        );
                        println!("{e}");
                        println!("Would you like to try again?");
                        if confirm_dialog(ConfirmDialogResponse::Yes) == ConfirmDialogResponse::No {
                            break;
                        }
                    }
                    Ok(_) => {
                        println!("All clear!! Nvim configured!");
                        break;
                    }
                }
            }
            loop {
                match configure_alacritty_with_shh() {
                    Err(GitScriptError::HomeNotAvailable(e)) => {
                        println!(
                            "It seem that there is no home for me... Im kinda homelesss......"
                        );
                        println!("{e}");
                        println!("Would you like to try again?");
                        if confirm_dialog(ConfirmDialogResponse::Yes) == ConfirmDialogResponse::No {
                            break;
                        }
                    }
                    Err(GitScriptError::CouldNotStart(e)) => {
                        println!(
                            "An Error ocurred! It seems that git wasnt able to start.... Waaaaa?"
                        );
                        println!("{e}");
                        println!("Would you like to try again?");
                        if confirm_dialog(ConfirmDialogResponse::Yes) == ConfirmDialogResponse::No {
                            break;
                        }
                    }
                    Err(GitScriptError::ComandFailed(e)) => {
                        println!(
                            "An Error ocurred! It seems that git wasnt able to download... Maybe the repo doenst exist anymore or it has a typo?"
                        );
                        println!("{e}");
                        println!("Would you like to try again?");
                        if confirm_dialog(ConfirmDialogResponse::Yes) == ConfirmDialogResponse::No {
                            break;
                        }
                    }
                    Ok(_) => {
                        println!("All clear!! Alacritty configured!");
                        break;
                    }
                }
            }
        }
    }
}
