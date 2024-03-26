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

#[derive(Clone)]
pub struct Game {
    pub state: Vec<Vec<Option<()>>>,
    pub cycles: usize,
    pub alive_cells: usize,
}

impl Game {
    pub fn new(rows: usize, cols: usize) -> Self {
        let mut game = Game {
            state: vec![vec![None; cols]; rows],
            cycles: 0,
            alive_cells: 0,
        };

        game.init_pattern();
        game
    }

    fn init_pattern(&mut self) {
        let row_mid = self.state.len() / 2;
        let col_mid = self.state[0].len() / 2;

        // initial game set
        // r-pentomino pattern
        // --xx--
        // -xx---
        // --x
        self.state[row_mid][col_mid] = Some(());
        self.state[row_mid][col_mid + 1] = Some(());
        self.state[row_mid + 1][col_mid - 1] = Some(());
        self.state[row_mid + 1][col_mid] = Some(());
        self.state[row_mid + 2][col_mid] = Some(());
    }

    pub fn reset(&mut self) {
        self.state = vec![vec![None; self.state.len()]; self.state[0].len()];
        self.init_pattern();
    }

    pub fn flip(&mut self, i: usize, j: usize) {
        match self.state[i][j] {
            Some(_) => {
                let _ = self.state[i][j].take();
                if self.alive_cells > 0 {
                    self.alive_cells -= 1;
                }
            }
            None => {
                self.state[i][j] = Some(());
                self.alive_cells += 1;
            }
        };
    }

    pub fn next_cycle(&self) -> Game {
        let mut next = Game::new(self.state.len(), self.state[0].len());

        next.cycles += self.cycles + 1;
        next.alive_cells = 0;

        for i in 0..self.state.len() {
            for j in 0..self.state[i].len() {
                // copy state from previous iteration to next
                next.state[i][j] = self.state[i][j];

                let count = self.count_neigbours(i, j);

                // Any live cell with fewer than two live neighbors dies, as if by underpopulation.
                if count < 2 {
                    next.state[i][j] = None;
                }
                // Any live cell with two or three live neighbors lives on to the next generation.
                if count >= 4 {
                    next.state[i][j] = None;
                }
                // Any live cell with more than three live neighbors dies, as if by overpopulation.
                if count == 3 {
                    next.state[i][j] = Some(());
                }

                if next.state[i][j].is_some() {
                    next.alive_cells += 1;
                }
            }
        }
        next
    }

    fn count_neigbours(&self, i: usize, j: usize) -> usize {
        let mut count = 0;

        let b = &self.state;

        let cols = b[0].len();
        let rows = b.len();
        for x in -1..2 {
            for y in -1..2 {
                let row = ((i as i32 + x + rows as i32) % rows as i32) as usize;
                let col = ((j as i32 + y + cols as i32) % cols as i32) as usize;

                if b[row][col].is_some() {
                    count += 1;
                };
            }
        }

        count
    }

    pub fn repaint(&self) {
        move_cursor_home();
        clear_screen();
        print!("{}", self);
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();

        self.state.iter().for_each(|row| {
            row.iter().for_each(|cell| match cell {
                None => out.push(' '),
                Some(_) => out.push_str(&random_foreground_color("X")),
                _ => unreachable!("cells can only be in the state of dead (=0) or alive (=1)"),
            });
            out.push('\n');
        });

        write!(f, "{}", out)
    }
}
