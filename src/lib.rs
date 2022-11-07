pub mod error;
pub mod game;
pub mod protocol;
pub mod reconnect_token;
pub mod result;
pub mod room;
pub mod room_id;
pub mod user_id;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
