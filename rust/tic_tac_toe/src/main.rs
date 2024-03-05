use std::{error::Error, fmt, usize};

#[derive(Debug, Clone)]
enum Cell {
    Taken(Player),
    Free,
}

#[derive(Debug, Clone)]
enum Player {
    Cross,
    Circle,
}

enum GameError {
    InvalidSize(String),
}

enum BoardOpt {
    ThreeByThree = 3,
    NineByNine = 9,
}

impl BoardOpt {
    fn dim(self) -> usize {
        match self {
            BoardOpt::ThreeByThree => 3 * 3,
            BoardOpt::NineByNine => 9 * 9,
        }
    }
}

struct Game {
    state: Vec<Cell>, // game board
    turn: Player,
}

impl Game {
    fn new(size: BoardOpt) -> Game {
        Game {
            state: vec![Cell::Free; size.dim()],
            turn: Player::Cross,
        }
    }

    fn apply(&mut self, row: usize, col: usize) -> Result<(), GameError> {
        let index = self.to_1d_index(row, col);

        self.state[index] = Cell::Taken(self.turn.clone());
        self.turn = match self.turn {
            Player::Circle => Player::Cross,
            Player::Cross => Player::Circle,
        };

        Ok(())
    }

    fn to_1d_index(&self, x: usize, y: usize) -> usize {
        match self.state.len() {
            9 => x * 3 + y,
            27 => x * 9 + y,
            _ => unreachable!("other board options besides (3x3 and 9x9) should not be allowed"),
        }
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, " - - -").unwrap();
        self.state
            .chunks(BoardOpt::ThreeByThree as usize)
            .for_each(|row| {
                row.iter().for_each(|cell| {
                    let char = match cell {
                        Cell::Taken(player) => match player {
                            Player::Cross => "X".to_string(),
                            Player::Circle => "O".to_string(),
                        },
                        Cell::Free => " ".to_string(),
                    };

                    write!(f, "|{}", char).unwrap();
                });
                writeln!(f, "|\n - - -").unwrap();
            });

        Ok(())
    }
}

fn main() {
    let mut game = Game::new(BoardOpt::ThreeByThree);

    let _ = game.apply(1, 1);
    println!("{game}");
}
