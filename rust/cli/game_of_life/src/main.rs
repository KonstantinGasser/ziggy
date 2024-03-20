use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Run,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Run => {
            clear_screen();
            let mut game = Game::new(40, 80);

            println!("{game}");

            std::thread::sleep(std::time::Duration::from_millis(200));

            loop {
                clear_screen();
                game = game.next_cycle();
                println!("{game}");
                std::thread::sleep(std::time::Duration::from_millis(200));
            }
        }
    }
}

fn clear_screen() {
    print!("\u{001b}c");
}

#[derive(Clone)]
struct Game(Vec<Vec<u8>>);

impl Game {
    fn new(rows: usize, cols: usize) -> Self {
        let mut game = Game(vec![vec![0_u8; cols]; rows]);

        // initial game set
        // r-pentomino pattern
        // 17: --xx--
        // 18: -xx---
        // 19: --x
        game.0[17][40] = 1;
        game.0[17][41] = 1;
        game.0[18][39] = 1;
        game.0[18][40] = 1;
        game.0[19][40] = 1;

        game
    }

    fn next_cycle(&self) -> Game {
        let mut next = Game::new(self.0.len(), self.0[0].len());

        for i in 0..self.0.len() {
            for j in 0..self.0[i].len() {
                // copy state from previous iteration to next
                next.0[i][j] = self.0[i][j];

                let count = self.count_neigbours(i, j);

                // Any live cell with fewer than two live neighbors dies, as if by underpopulation.
                if count < 2 {
                    next.0[i][j] = 0
                }
                // Any live cell with two or three live neighbors lives on to the next generation.
                if count >= 4 {
                    next.0[i][j] = 0
                }
                // Any live cell with more than three live neighbors dies, as if by overpopulation.
                if count == 3 {
                    next.0[i][j] = 1
                }
            }
        }
        next
    }

    fn count_neigbours(&self, i: usize, j: usize) -> usize {
        let mut count = 0;

        let b = &self.0;

        NEIGBOUR_DIRECTIONS.iter().for_each(|direction| {
            let row = i as i8 + direction[0];
            let col = j as i8 + direction[1];

            if row < 0 || row >= b.len() as i8 {
                return;
            }

            if col < 0 || col >= b[0].len() as i8 {
                return;
            }

            if b[row as usize][col as usize] == 1 {
                count += 1;
            }
        });

        count
    }
}

const NEIGBOUR_DIRECTIONS: [[i8; 2]; 8] = [
    [-1, -1],
    [0, -1],
    [1, -1],
    [1, 0],
    [1, 1],
    [0, 1],
    [-1, 1],
    [-1, 0],
];

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();

        self.0.iter().for_each(|row| {
            row.iter().for_each(|cell| match cell {
                0 => out.push(' '),
                1 => out.push('X'),
                _ => unreachable!("cells can only be in the state of dead (=0) or alive (=1)"),
            });
            out.push('\n');
        });

        write!(f, "{}", out)
    }
}
