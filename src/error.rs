use thiserror::Error;

pub type DnResult<T> = Result<T, DnError>;

#[derive(Error, Debug)]
pub enum DnError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    SqlError(#[from] rusqlite::Error)
}