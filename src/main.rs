use std::process::{exit, Command, Output};

use clap::Parser;
use error::{Error, Result};
use opt::{Command::*, Opt};

mod error;
mod opt;

fn main() {
    git_version().unwrap_or_else(|e| {
        eprintln!("{}", e);
        exit(1);
    });

    match Opt::parse().command {
        Init => init(),
        Save { name, message } => save(name, message),
        Load { query } => load(query),
        Log => log(),
    }
}

fn git_version() -> Result<Output> {
    Command::new("git")
        .arg("--version")
        .output()
        .or(Err(Error::NoGit))
}

fn init() {}

fn save(name: Option<String>, message: Option<String>) {
    let git_output = Command::new("git").arg("add").arg(".").output();
}

fn load(query: String) {
    println!("Loading '{}'", query)
}

fn log() {
    println!("Logging the backup tree")
}
