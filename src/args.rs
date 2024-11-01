use clap::{Parser, Subcommand};

/// A batch rename tool for files and directories
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Renames files or directories
    Rn {
        /// rename many files that match a pattern
        #[arg(short, long)]
        file: bool,
        /// rename many directories that match a pattern
        #[arg(short, long)]
        direcory: bool,
    },
    /// Strips and renames files or directories
    SRn {
        /// Strips and renames many files that match a pattern
        #[arg(short, long)]
        file: bool,
        /// Strips and renames many directories that match a pattern
        #[arg(short, long)]
        direcory: bool,
    },

    /// Deletes files or directories
    Rm {
        /// delete many files that match a pattern
        #[arg(short, long)]
        file: bool,
        /// delete many directories that match a pattern
        #[arg(short, long)]
        direcory: bool,
    },
}
