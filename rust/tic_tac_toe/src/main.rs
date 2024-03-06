use std::{error::Error, fmt, io, usize};

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

#[derive(Debug)]
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

    fn apply(&mut self, placement: Placement) -> Result<(), GameError> {
        let index = self.to_1d_index(placement.0, placement.1);

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

struct Placement(usize, usize);

fn read_promot(stdin: &mut io::Stdin, buffer: &mut String) -> Result<Placement, GameError> {
    match stdin.read_line(buffer) {
        Ok(_) => (),
        Err(err) => panic!("unable to read promopt from io::Stdin: {err}"),
    };

    // convert input to a Placement
    let coordinates = buffer.split(',').collect::<Vec<&str>>();

    let x = match coordinates[0].parse::<usize>() {
        Ok(x) => x,
        Err(err) => panic!("X coordinate is not a usize: {err}"),
    };

    let y = match coordinates[0].parse::<usize>() {
        Ok(y) => y,
        Err(err) => panic!("Y coordinate is not a usize: {err}"),
    };

    Ok(Placement(x, y))
}

fn main() {
    let mut stdin = io::stdin();

    let mut game = Game::new(BoardOpt::ThreeByThree);

    let mut buffer = String::new();
    loop {
        let placement = read_promot(&mut stdin, &mut buffer).unwrap();
        buffer.clear(); // is there a safer way of cleaning it up? while resuing the same
                        // underlying buffer?

        let _ = game.apply(placement);
        println!("{game}");
    }
    // let _ = game.apply(1, 1);

    // let _ = game.apply(0, 1);
    // println!("{game}");
}
