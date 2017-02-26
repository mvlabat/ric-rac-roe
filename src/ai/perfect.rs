use ::game::*;
use std;

pub fn player_controller(game: &Game) -> Option<Coords> {
    let coords = optimal_move(game.current_player(), game.field());
    Some(coords)
}

fn optimal_move(ai_player: Player, field: &Field) -> Coords {
    let (coords, _) = best_outcome(ai_player, ai_player, field, 1);
    coords
}

/// Returns value of an outcome.
fn minimax(ai_player: Player, field: &Field, current_player: Player, depth: i32) -> i32 {
    match Game::winner(&field) {
        Some(Player::Nobody) => 0,
        Some(player) => {
            if player == ai_player { 10 } else { -10 }
        },
        None => {
            let (_, value) = best_outcome(ai_player, opposite_player(current_player), &field, depth + 1);
            value
        }
    }
}

fn best_outcome(ai_player: Player, current_player: Player, field: &Field, depth: i32) -> (Coords, i32) {
    let possible_moves: Vec<(Coords, i32)> = possible_outcomes(field, current_player).iter()
        .map(|&(coords, outcome)| {
            (coords, minimax(ai_player, &outcome, current_player, depth))
        }).collect();

    fn comparator(&&(_, x_value): &&(Coords, i32), &&(_, y_value): &&(Coords, i32)) -> std::cmp::Ordering {
        x_value.cmp(&y_value)
    }

    if ai_player == current_player {
        *possible_moves.iter().max_by(comparator).unwrap()
    } else {
        *possible_moves.iter().min_by(comparator).unwrap()
    }
}

fn opposite_player(player: Player) -> Player {
    match player {
        Player::X => Player::O,
        Player::O => Player::X,
        _ => panic!("player is neither X, nor O")
    }
}

fn possible_outcomes(field: &Field, player: Player) -> Vec<(Coords, Field)> {
    let mut outcomes: Vec<(Coords, Field)> = Vec::new();

    for (i, &row) in field.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col == Player::Nobody {
                let mut outcome = field.clone();
                outcome[i][j] = player;
                outcomes.push( ((i, j), outcome) );
            }
        }
    }

    outcomes
}
