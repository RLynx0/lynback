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

fn init() {
    todo!()
}

fn save(name: Option<String>, message: Option<String>) {
    todo!()
}

fn load(query: String) {
    todo!()
}

fn log() {
    todo!()
}
