use typescript_definitions::TypeScriptifyTrait;

use board_game_io_base::protocol::*;
use board_game_io_base::user_id::UserId;

fn main() {
    #[cfg(any(debug_assertions, feature="export-typescript"))]
    {
        println!("{}", UserId::type_script_ify());
        println!("{}", ServerMessage::type_script_ify());
        println!("{}", ClientMessage::type_script_ify());
    }

    #[cfg(not(debug_assertions))]
    println!("ERROR: Must run debug build");
}
