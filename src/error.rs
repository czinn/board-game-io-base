#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("lobby is empty")]
    EmptyLobby,
    #[error("user must be leader to perform operation")]
    UserNotLeader,
    #[error("game is already started")]
    GameAlreadyStarted,
    #[error("game has not started")]
    GameNotStarted,
    #[error("invalid player mapping")]
    InvalidPlayerMapping,
    #[error("game state has wrong number of players")]
    WrongPlayerCount,
    #[error("could not create game state")]
    InvalidCreate,
    #[error("user is spectating")]
    UserNotInGame,
    #[error("invalid action")]
    InvalidAction,
    #[error("unknown error")]
    Unknown,
}
