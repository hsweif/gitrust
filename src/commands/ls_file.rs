use crate::cli::Command;
use crate::index;

pub fn execute(command: Command) {
    if let Command::LsFile { stage } = command {
        match stage {
            true => match index::load_index() {
                Ok(entries) => {
                    for entry in entries {
                        println!("{}", entry);
                    }
                }
                Err(e) => eprintln!("Error: {}", e),
            },
            false => eprintln!("Unimplemented"),
        }
    }
}
