mod args;
mod operations;
mod visual;
use crate::args::Commands;

use clap::Parser;

fn main() {
    let cli = args::Cli::parse();
    // dbg!(cli);
    match &cli.command {
        Some(Commands::Rn { .. }) => {
            let dir = operations::user_input("a direcotry");
            let rn_to = operations::user_input("rename to");
            operations::rename_many_files(&dir, &rn_to);
        }
        Some(Commands::SRn { .. }) => {
            let dir = operations::user_input("a direcotry");
            operations::strip_rename_many_files(&dir);
        }
        Some(Commands::Rm { file, .. }) => {
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
