use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Git not found on this system")]
    NoGit,
}

pub type Result<T> = std::result::Result<T, Error>;
