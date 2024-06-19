mod args;
mod operations;
mod visual;
use crate::args::Commands;

use clap::Parser;

fn main() {
    let cli = args::Cli::parse();
    // dbg!(cli);
    match &cli.command {
        Some(Commands::Rename { .. }) => {
            let dir = operations::user_input("a direcotry");
            operations::rename_many_files(&dir);
        }
        Some(Commands::Delete { file, .. }) => {
            let dir = operations::user_input("a direcotry");
            if *file {
                operations::delete_many_files(&dir);
                return;
            }
            operations::delete_many_dirs(&dir);
        }
        None => {}
    }
}
