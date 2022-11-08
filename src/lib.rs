pub mod error;
pub mod game;
pub mod ids;
pub mod protocol;
pub mod result;
pub mod room;
pub mod server;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
