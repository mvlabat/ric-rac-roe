mod draw_field;
mod console_player;

use ::game::*;

use std::io;
use std::error::Error;

pub fn run() {
    loop {
        println!("{}" ,instructions());

        match (first_player(), second_player()) {
            (Some(first_player), Some(second_player)) => play(first_player, second_player),
            _ => break,
        }
    }
}

fn play(first_player: PlayerController, second_player: PlayerController) {
    let mut game: Game = Game::new(first_player, second_player);
    match game.play() {
        Player::Nobody => println!("\nWell, that's a draw. ¯\\_(ツ)_/¯"),
        winner @ _ => println!("\nPlayer {0} wins! (ツ)_/\\_({0})", player_to_str(winner)),
    }
    println!("");
    print_game_field(&game.field());
}

fn player_to_str(player: Player) -> String {
    match player {
        Player::X => "X",
        Player::O => "O",
        Player::Nobody => "Nobody",
    }.into()
}

fn print_game_field(field: &Field) {
    println!("{}", draw_field::draw(&field));
}

fn instructions() -> String {
    r#"
    Hey! That's a new game.

    You can use the following keys:
    - 1 - Human
    - 2 - Perfect AI
    - 3 - Ultimate AI
    "#.into()
}

fn first_player() -> Option<PlayerController> {
    print!("Choose the first player (or print 'q' to quit): ");
    choose_player()
}

fn second_player() -> Option<PlayerController> {
    print!("Choose the second player (or print 'q' to quit): ");
    choose_player()
}

fn choose_player() -> Option<PlayerController> {
    loop {
        io::Write::flush(&mut io::stdout()).unwrap();
        let mut text: String = String::new();
        io::stdin().read_line(&mut text).unwrap();
        let text = text.trim_right();

        if text == String::from("q") {
            return None;
        }

        match parse_key(text) {
            Ok(1) => return Some(console_player::player_controller),
            Ok(2) => return Some(visualized_perfect_ai),
            Ok(3) => return Some(visualized_ultimate_ai),
            Err(error) => print!("Error: {}\nRepeat: ", error),
            _ => panic!("unexpected key {}", text),
        }
    }
}

fn parse_key(key: &str) -> Result<usize, String> {
    let key: usize = key.parse().map_err(show_error)?;
    match key {
        1 ... 3 => Ok(key),
        _ => Err(parse_error())
    }
}

fn parse_error() -> String {
    "possible keys are 1, 2 or 3".into()
}

fn show_error<T: Error>(err: T) -> String {
    err.description().to_string()
}

fn visualized_perfect_ai(game: &Game) -> Option<Coords> {
    println!("");
    print_game_field(&game.field());
    let (row, col) = ::ai::perfect::player_controller(&game).unwrap();
    println!("Player {0} controlled by Perfect AI chooses ({1}, {2})",
             player_to_str(game.current_player()), row, col);
    Some((row, col))
}

fn visualized_ultimate_ai(game: &Game) -> Option<Coords> {
    println!("");
    print_game_field(&game.field());
    let (row, col) = ::ai::ultimate::player_controller(&game).unwrap();
    println!("Player {0} controlled by Ultimate AI chooses ({1}, {2})",
             player_to_str(game.current_player()), row, col);
    Some((row, col))
}
