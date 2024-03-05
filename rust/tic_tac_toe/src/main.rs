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
    FourByFour = 4,
}

impl BoardOpt {
    fn dim(self) -> usize {
        match self {
            BoardOpt::ThreeByThree => 3 * 3,
            BoardOpt::FourByFour => 4 * 4,
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
            16 => x * 4 + y,
            _ => unreachable!("other board options besides (3x3 and 4x4) should not be allowed"),
        }
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut divider = |f: &mut fmt::Formatter, count: usize| {
            writeln!(f).unwrap();
            for _i in 0..count {
                write!(f, " ---").unwrap();
            }
            writeln!(f).unwrap();
        };

        let chunk_size: usize = match self.state.len() {
            9 => 3,
            16 => 4,
            _ => unreachable!("other board options besides (3x3 and 4x4) should not be allowed"),
        };

        divider(f, chunk_size);

        self.state.chunks(chunk_size).for_each(|row| {
            row.iter().for_each(|cell| {
                let char = match cell {
                    Cell::Taken(player) => match player {
                        Player::Cross => " X ".to_string(),
                        Player::Circle => " O ".to_string(),
                    },
                    Cell::Free => "   ".to_string(),
                };

                write!(f, "|{}", char).unwrap();
            });
            write!(f, "|").unwrap();
            divider(f, chunk_size);
        });

        Ok(())
    }
}

fn main() {
    let mut game = Game::new(BoardOpt::FourByFour);

    let _ = game.apply(1, 1);
    println!("{game}");

    let _ = game.apply(0, 1);
    println!("{game}");
}
