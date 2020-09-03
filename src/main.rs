use colored::*;
use exitfailure::ExitFailure;
use failure::ResultExt;
use std::io::{self, Read, Write};
use structopt::StructOpt;

const PIKACHU: &str = "⣿⣿⣿⣿⣿⡏⠉⠛⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⣿
⣿⣿⣿⣿⣿⣿⠀⠀⠀⠈⠛⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠿⠛⠉⠁⠀⣿
⣿⣿⣿⣿⣿⣿⣧⡀⠀⠀⠀⠀⠙⠿⠿⠿⠻⠿⠿⠟⠿⠛⠉⠀⠀⠀⠀⠀⣸⣿
⣿⣿⣿⣿⣿⣿⣿⣷⣄⠀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣴⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⠏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠠⣴⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⡟⠀⠀⢰⣹⡆⠀⠀⠀⠀⠀⢰⣹⡆⠀⠀⠀⠸⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⠃⠀⠀⠈⠉⠀⠀⠤⠄⠀⠀⠀⠉⠁⠀⠀⠀⠀⢿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⢾⣿⣷⠀⠀⠀⠀⡠⠤⢄⠀⠀⠀⠠⣿⣿⣷⠀⢸⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⡀⠉⠀⠀⠀⠀⠀⢄⠀⢀⠀⠀⠀⠀⠉⠉⠁⠀⠀⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣧⠀⠀⠀⠀⠀⠀⠀⠈⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢹⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⣿";

const POKEBALL: &str = "────────▄███████████▄────────
─────▄███▓▓▓▓▓▓▓▓▓▓▓███▄─────
────███▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓███────
───██▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓██───
──██▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓██──
─██▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓██─
██▓▓▓▓▓▓▓▓▓███████▓▓▓▓▓▓▓▓▓██
██▓▓▓▓▓▓▓▓██░░░░░██▓▓▓▓▓▓▓▓██
██▓▓▓▓▓▓▓██░░███░░██▓▓▓▓▓▓▓██
███████████░░███░░███████████
██░░░░░░░██░░███░░██░░░░░░░██
██░░░░░░░░██░░░░░██░░░░░░░░██
██░░░░░░░░░███████░░░░░░░░░██
─██░░░░░░░░░░░░░░░░░░░░░░░██─
──██░░░░░░░░░░░░░░░░░░░░░██──
───██░░░░░░░░░░░░░░░░░░░██───
────███░░░░░░░░░░░░░░░███────
─────▀███░░░░░░░░░░░███▀─────
────────▀███████████▀────────";

#[derive(StructOpt)]
struct Options {
    #[structopt(default_value = "Created by manishsingh10895")]
    message: String,

    #[structopt(short = "p", long = "pokeball")]
    /// To show POKEBALL
    pokeball: bool,

    #[structopt(short = "f", long = "file", parse(from_os_str))]
    ///Use your own ASCII pikachu from a file
    file: Option<std::path::PathBuf>,

    #[structopt(short = "i", long = "stdin")]
    /// Read the message from stdin
    stdin: bool,
}

fn main() -> Result<(), ExitFailure> {
    let options = Options::from_args();

    let mut message = "".to_string();

    if options.stdin {
        io::stdin()
            .read_to_string(&mut message)
            .expect("Error reading from stdin");
        message.pop();
    } else {
        message = options.message;
    }

    if options.pokeball {
        println!("{}", POKEBALL.to_string().red());
    } else {
        match &options.file {
            Some(path) => {
                let alt = std::fs::read_to_string(path)
                    .with_context(|_| format!("could not find file {:?}", path))?;
                print_message(message);
                println!("{}", alt.to_string().bright_yellow());
            }
            None => {
                print_message(message);
                println!("{}", PIKACHU.to_string().bright_yellow())
            }
        }
    }

    return Ok(());
}

fn print_message(message: String) {
    if message.to_lowercase().contains("raichu") {
        eprintln!("Pikachu hasn't evolved yet");
    } else {
        println!(" {}", "-".repeat(message.chars().count() + 2));
        println!("| {} |", message);
        println!(" {}", "-".repeat(message.chars().count() + 2));
        println!("  \\  /");
        println!("   \\/");
    }
}
