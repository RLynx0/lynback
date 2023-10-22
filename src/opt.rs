use clap::{Parser, Subcommand};

/// Utility for tree-style configuration backups
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Opt {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Clone, Debug, Subcommand)]
pub enum Command {
    /// Initialise backup repository
    Init,

    /// Create new backup node
    Save {
        /// Optional name for the backup
        name: Option<String>,

        /// Optional description for backup
        #[arg(short, long)]
        message: Option<String>,
    },

    /// Load a backup
    Load {
        /// Id or unix-style path to backup
        query: String,
    },

    /// Display backup tree
    Log,
}
