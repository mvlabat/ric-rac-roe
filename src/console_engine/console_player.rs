use ::game::*;
use super::{print_game_field, player_to_str};

use std::io;
use std::error::Error;

pub fn player_controller(game: &Game) -> Option<Coords> {
    print_game_field(&game.field());
    prompt(&game)
}

fn prompt(game: &Game) -> Option<Coords> {
    loop {
        print!(
            "Player {}, enter row and column (1..3) separated with whitespace (or print 's' to surrender): ",
            player_to_str(game.current_player()));
        io::Write::flush(&mut io::stdout()).unwrap();

        match read_coords() {
            Ok(Some(coords)) if game.cell_is_empty(coords) => return Some(coords),
            Ok(Some(_)) => println!("Error: you really should not take another player's cell!"),
            Ok(None) => return None,
            Err(error) => println!("Error: {}", error)
        }
    }
}

fn read_coords() -> Result<Option<Coords>, String> {
    fn parse_num(num: &str) -> Result<usize, String> {
        let num: usize = num.parse().map_err(show_error)?;
        match num {
            1 ... 3 => Ok(num),
            _ => Err(parse_error())
        }
    }

    let mut text: String = String::new();
    io::stdin().read_line(&mut text).map_err(show_error).and_then(|_| {
        let text = text.trim_right();
        if text == String::from("s") {
            return Ok(None)
        }

        let coords =
            text.split_whitespace()
                .map(parse_num)
                .collect::<Result<Vec<usize>, _>>()?;

        if coords.len() != 2 {
            return Err(parse_error());
        }

        Ok(Some( (coords[0] - 1, coords[1] - 1) ))
    })
}

fn show_error<T: Error>(err: T) -> String {
    err.description().to_string()
}

fn parse_error() -> String {
    "you have to enter two integers (1..3)".into()
}
