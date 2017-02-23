pub type Row = [Player; 3];

pub type Field = [Row; 3];

pub type Coords = (usize, usize);

/// Must return `Coords` or `None` if a player chooses to surrender.
pub type PlayerController = fn(game: &Game) -> Option<Coords>;

#[derive(Copy, Clone, PartialEq)]
pub enum Player {
    Nobody,
    X,
    O,
}

pub struct Game {
    current_player: Player,
    field: Field,
    player1: PlayerController,
    player2: PlayerController,
}

impl Game {
    pub fn new(player1: PlayerController, player2: PlayerController) -> Game {
        Game {
            field: [[Player::Nobody; 3]; 3],
            current_player: Player::X,
            player1: player1,
            player2: player2,
        }
    }

    pub fn field(&self) -> &Field {
        &self.field
    }

    pub fn current_player(&self) -> Player {
        self.current_player
    }

    pub fn play(&mut self) -> Player {
        loop {
            match self.current_player_coords() {
                Some((row, col)) => {
                    if !self.cell_is_empty((row, col)) {
                        panic!("player tries to set occupied cell - cheating!")
                    }
                    self.field[row][col] = self.current_player;
                },
                _ => { self.switch_player(); return self.current_player },
            };

            match self.winner() {
                Some(winner) => return winner,
                _ => self.switch_player(),
            }
        }
    }

    pub fn cell_is_empty(&self, coords: Coords) -> bool {
        let (row, col) = coords;
        self.field[row][col] == Player::Nobody
    }

    fn current_player_coords(&self) -> Option<Coords> {
        match self.current_player {
            Player::X => (self.player1)(&self),
            Player::O => (self.player2)(&self),
            _ => panic!("current player can't be Nobody"),
        }
    }

    fn switch_player(&mut self) {
        if self.current_player == Player::X {
            self.current_player = Player::O
        } else if self.current_player == Player::O {
            self.current_player = Player::X
        }
    }

    /// Returns `Player` if it is already defined, otherwise `None`.
    fn winner(&self) -> Option<Player> {
        self.row_winner(self.row(0))
            .or( self.row_winner(self.row(1)) )
            .or( self.row_winner(self.row(2)) )
            .or( self.row_winner(self.col(0)) )
            .or( self.row_winner(self.col(1)) )
            .or( self.row_winner(self.col(2)) )
            .or( self.row_winner(self.diagonal()) )
            .or( self.row_winner(self.reverse_diagonal()) )
            .or( if self.has_empty_cell() { None } else { Some(Player::Nobody) } )
    }

    fn has_empty_cell(&self) -> bool {
        for row in &self.field {
            for col in row {
                if *col == Player::Nobody {
                    return true;
                }
            }
        }
        false
    }

    /// Returns `Player` if it is already defined, otherwise `None`.
    fn row_winner(&self, row: Vec<Player>) -> Option<Player> {
        if row[0] != Player::Nobody && row[0] == row[1] && row[1] == row[2] {
            Some(row[0])
        } else {
            None
        }
    }

    fn row(&self, number: usize) -> Vec<Player> {
        self.field[number].to_vec()
    }

    fn col(&self, number: usize) -> Vec<Player> {
        self.field.iter().map(|row| row[number]).collect()
    }

    fn diagonal(&self) -> Vec<Player> {
        self.field.iter().enumerate().map(|(i, row)| row[0 + i]).collect()
    }

    fn reverse_diagonal(&self) -> Vec<Player> {
        self.field.iter().enumerate().map(|(i, row)| row[2 - i]).collect()
    }

}
