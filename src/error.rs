#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("could not create game state")]
    InvalidCreate,
    #[error("invalid action")]
    InvalidAction,
    #[error("unknown error")]
    Unknown,
}
