mod cli;
mod commands;
mod objects;

use clap::Parser;
use cli::{App, Command};

fn main() {
    let app = App::parse();
    // ...other commands can be added here
    match app.command {
        Command::HashObject {
            path,
            stdin,
            write_flag,
        } => commands::hash_object::execute(Command::HashObject {
            path,
            stdin,
            write_flag,
        }),
        Command::CatFile {
            hash,
            type_flag,
            contents_flag,
            size_flag,
        } => commands::cat_file::execute(Command::CatFile {
            hash,
            type_flag,
            contents_flag,
            size_flag,
        }),
        Command::LsFile { stage } => commands::ls_file::execute(Command::LsFile { stage }),
    }
}
