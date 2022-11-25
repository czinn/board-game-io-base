use crate::ids::UserId;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("could not parse value")]
    ParseFailure,
    #[error("lobby is empty")]
    EmptyLobby,
    #[error("username is in use")]
    UsernameInUse,
    #[error("invalid reconnect token")]
    InvalidReconnectToken,
    #[error("user must be leader to perform operation")]
    UserNotLeader,
    #[error("user not found")]
    UserNotFound,
    #[error("user is in game and cannot be kicked")]
    UserIsPlayer,
    #[error("user {0} is not a player")]
    UserIsNotPlayer(UserId),
    #[error("user {0} is already a player")]
    UserIsAlreadyPlayer(UserId),
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
    #[error("invalid action: {0}")]
    InvalidAction(String),
    #[error("encountered tokio error: {0}")]
    TokioError(String),
    #[error("unknown error")]
    Unknown,
}
