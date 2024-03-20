use rand::Rng;

fn clear_screen() {
    print!("\x1B[2J");
}

fn move_cursor_home() {
    print!("\x1B[H");
}

fn random_foreground_color(placeholder: &str) -> String {
    let colors = [
        "31", "32", "33", "34", "35", "36", "91", "92", "93", "94", "95", "96",
    ];
    let mut rng = rand::thread_rng();
    let color_code = colors[rng.gen_range(0..colors.len())];
    format!("\x1B[{}m{}", color_code, placeholder)
}
pub struct Game(Vec<Vec<u8>>);

impl Game {
    pub fn new(rows: usize, cols: usize) -> Self {
        let mut game = Game(vec![vec![0_u8; cols]; rows]);

        let row_mid = game.0.len() / 2;
        let col_mid = game.0[0].len() / 2;

        // initial game set
        // r-pentomino pattern
        // --xx--
        // -xx---
        // --x
        game.0[row_mid][col_mid] = 1;
        game.0[row_mid][col_mid + 1] = 1;
        game.0[row_mid + 1][col_mid - 1] = 1;
        game.0[row_mid + 1][col_mid] = 1;
        game.0[row_mid + 2][col_mid] = 1;

        game
    }

    pub fn next_cycle(&self) -> Game {
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
            let row = i as i16 + direction[0];
            let col = j as i16 + direction[1];

            if row < 0 || row >= b.len() as i16 {
                return;
            }

            if col < 0 || col >= b[0].len() as i16 {
                return;
            }

            if b[row as usize][col as usize] == 1 {
                count += 1;
            }
        });

        count
    }

    pub fn repaint(&self) {
        move_cursor_home();
        clear_screen();
        print!("{}", self);
    }
}

const NEIGBOUR_DIRECTIONS: [[i16; 2]; 8] = [
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
                1 => out.push_str(&random_foreground_color("X")),
                _ => unreachable!("cells can only be in the state of dead (=0) or alive (=1)"),
            });
            out.push('\n');
        });

        write!(f, "{}", out)
    }
}