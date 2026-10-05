use std::io;

pub fn confirm_dialog(default: ConfirmDialogResponse) -> ConfirmDialogResponse {
    let question = match default {
        ConfirmDialogResponse::Yes => "[Y/n]",
        ConfirmDialogResponse::No => "[y/N]",
    };

    loop {
        println!("{question}");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Err(_) | Ok(0) => return default,
            Ok(_) => {}
        } // Ok(0) means the user pressed CTRL+D

        match input.trim() {
            "y" | "Y" => return ConfirmDialogResponse::Yes,
            "n" | "N" => return ConfirmDialogResponse::No,
            "" => return default,
            _ => println!("\nWrong answer, please try again!"),
        }
    }
}

#[derive(PartialEq, Clone, Copy, Eq)]
pub enum ConfirmDialogResponse {
    Yes,
    No,
}
