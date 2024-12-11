mod cli;
mod commands;

use clap::Parser;
use cli::{App, Command};

fn main() {
    let app = App::parse();
    // ...other commands can be added here
    match app.command {
        Command::HashObject { path, stdin } => {
            commands::hash_object::execute(Command::HashObject { path, stdin })
        }
    }
}
