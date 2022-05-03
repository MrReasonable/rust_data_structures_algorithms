#[derive(thiserror::Error, Debug)]
pub enum BlobError {
    #[error("No Room")]
    NoRoom,
    #[error("Too Big.  Unable to store {0} bytes")]
    TooBig(u64),
    #[error("Not Found")]
    NotFound,
    #[error("Bincode Error: {0}")]
    Bincode(bincode::Error),
    #[error("IO Error: {0}")]
    IO(std::io::Error),
}

impl From<bincode::Error> for BlobError {
    fn from(e: bincode::Error) -> Self {
        Self::Bincode(e)
    }
}

impl From<std::io::Error> for BlobError {
    fn from(e: std::io::Error) -> Self {
        Self::IO(e)
    }
}
