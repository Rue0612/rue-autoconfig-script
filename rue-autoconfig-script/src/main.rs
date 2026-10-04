mod scripts;
mod util;

use crate::{
    scripts::{
        comand_error_enum::CommandError,
        installers::{
            install_aur_apps_script::install_aur_apps,
            install_flatpak_apps_script::install_flatpack_apps,
            install_pacman_apps_script::install_pacman_apps,
        },
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
                Err(CommandError::CouldNotStart(e)) => {
                    println!(
                        "An Error ocurred! It seems that pacman wasnt able to start.... Maybe an SUDO error?"
                    );
                    println!("{e}");
                    println!("Would you like to try again?");
                    if confirm_dialog(ConfirmDialogResponse::Yes) == ConfirmDialogResponse::No {
                        break;
                    }
                }
                Err(CommandError::ComandFailed(e)) => {
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
                Err(CommandError::CouldNotStart(e)) => {
                    println!(
                        "An Error ocurred! It seems that flatpack wasnt able to start.... Waaaaa?"
                    );
                    println!("{e}");
                    println!("Would you like to try again?");
                    if confirm_dialog(ConfirmDialogResponse::Yes) == ConfirmDialogResponse::No {
                        break;
                    }
                }
                Err(CommandError::ComandFailed(e)) => {
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
                Err(CommandError::CouldNotStart(e)) => {
                    println!(
                        "An Error ocurred! It seems that paru wasnt able to start.... Waaaaa?"
                    );
                    println!("{e}");
                    println!("Would you like to try again?");
                    if confirm_dialog(ConfirmDialogResponse::Yes) == ConfirmDialogResponse::No {
                        break;
                    }
                }
                Err(CommandError::ComandFailed(e)) => {
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
}
