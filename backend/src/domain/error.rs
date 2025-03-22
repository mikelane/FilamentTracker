use std::fmt;

#[derive(Debug)]
pub enum FilamentError {
    NotFound(String),
    InvalidData(String),
    RepositoryError(String),
}

impl std::error::Error for FilamentError {}

impl fmt::Display for FilamentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FilamentError::NotFound(id) => write!(f, "Filament with id '{}' not found", id),
            FilamentError::InvalidData(msg) => write!(f, "Invalid filament data: {}", msg),
            FilamentError::RepositoryError(msg) => write!(f, "Repository error: {}", msg),
        }
    }
}
