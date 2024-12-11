use camino::Utf8PathBuf;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(name = "gitrust", version)]
pub struct App {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    HashObject {
        /// The directory containing files to hash
        path: Option<Utf8PathBuf>,
        #[clap(long)]
        stdin: bool,
    },
    CatFile {
        hash: String,
        #[clap(short = 't')]
        type_flag: bool,
        #[clap(short = 'p')]
        contents_flag: bool,
        #[clap(short = 's')]
        size_flag: bool,
    },
    // ...other commands can be added here
}
