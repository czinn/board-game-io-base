pub mod error;
pub mod result;
pub mod game;
pub mod user_id;
pub mod game_room;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
