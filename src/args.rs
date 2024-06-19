use clap::{Parser, Subcommand};

/// A files and direcories rename tool
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Renames files or direcories
    Rename {
        /// rename many file that match a pattern
        #[arg(short, long)]
        file: bool,
        /// rename many direcories that match a pattern
        #[arg(short, long)]
        direcory: bool,
    },

    /// Deletes files
    Delete {
        #[arg(short, long)]
        file: bool,
        #[arg(short, long)]
        direcory: bool,
    },
}
