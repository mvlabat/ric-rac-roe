mod draw_field;
mod console_player;

use ::game::*;

pub fn run() {
    let mut game: Game = Game::new(console_player::player_controller, console_player::player_controller);
    match game.play() {
        Player::Nobody => println!("Well, that's a draw. ¯\\_(ツ)_/¯"),
        winner @ _ => println!("Player {0} wins! (ツ)_/\\_({0})", player_to_str(winner)),
    }
    print_game_field(&game.field());
}

fn print_game_field(field: &Field) {
    println!("{}", draw_field::draw(&field));
}

fn player_to_str(player: Player) -> String {
    match player {
        Player::X => "X",
        Player::O => "O",
        Player::Nobody => "Nobody",
    }.into()
}
