use std::cmp;

use rand::Rng;
use tokio::sync::broadcast;

use crate::conway::pattern;

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

#[derive(Clone, Copy, Default, Debug)]
pub struct Cell {
    pub state: Option<()>,
    pub cycles_alive: usize,
}

impl Cell {
    fn alive() -> Self {
        Cell {
            state: Some(()),
            cycles_alive: 0,
        }
    }

    fn revive(&mut self) {
        self.state = Some(());
    }

    fn declare_dead(&mut self) {
        self.cycles_alive = 0;
    }

    fn declear_survival(&mut self) {
        self.cycles_alive += 1;
    }

    // NOTE: colors in hsl are aligned on 360 degrees. The alive cycles
    // of a cell might be higher than 360. Two options:
    // 1) either we wrap the color back to the begining however then we lose
    // indication of age or
    // 2) we cap the color at its potential max of 360 which is the end state
    //
    // Option 2 will be implemeted
    pub fn color(&self) -> String {
        if self.state.is_none() {
            return format!("hsl(0, 0%, 100%)"); // color white
        }

        // QUESTION: what would be a good way to
        // color gradient the cell based on the cycles
        // it stayed alive?
        //
        // current issue is that cells start with (0,0,0).
        // each iteration the b in rgb evolves -> 255
        format!("hsl(210, 100%, {}%)", cmp::min(self.cycles_alive * 10, 100))
    }
}

#[derive(Debug)]
pub struct Game {
    pub state: Vec<Vec<Cell>>,
    pub cycles: usize,
    pub alive_cells: usize,
}

impl Game {
    pub fn empty(rows: usize, cols: usize) -> Self {
        Game {
            state: vec![vec![Cell::default(); cols]; rows],
            cycles: 0,
            alive_cells: 0,
        }
    }

    pub fn initialise_random(rows: usize, cols: usize) -> Self {
        let mut game = Game {
            state: vec![vec![Cell::default(); cols]; rows],
            cycles: 0,
            alive_cells: 0,
        };

        game.apply_pattern(
            pattern::copperhead(),
            game.state.len() / 2,
            game.state[0].len() / 2,
        );
        game
    }

    fn apply_pattern(&mut self, pattern: pattern::Pattern, start_x: usize, start_y: usize) {
        let rows = self.state.len();
        let cols = self.state[0].len();

        pattern.iter().for_each(|tuple| {
            self.state[wrap(start_x + tuple.0, rows)][wrap(start_y + tuple.1, cols)].revive();
        });
    }

    pub fn reset(&mut self) {
        self.state = vec![vec![Cell::default(); self.state.len()]; self.state[0].len()];
        self.apply_pattern(
            pattern::copperhead(),
            self.state.len() / 2,
            self.state[0].len() / 2,
        );
    }

    pub fn flip(&mut self, i: usize, j: usize) {
        match self.state[i][j].state {
            Some(_) => {
                self.state[i][j].state = None;
                if self.alive_cells > 0 {
                    self.alive_cells -= 1;
                }
            }
            None => {
                self.state[i][j].state = Some(());
                self.alive_cells += 1;
            }
        };
    }

    pub fn next_cycle(&self) -> Game {
        let mut next = Game::empty(self.state.len(), self.state[0].len());

        next.cycles += self.cycles + 1;
        next.alive_cells = 0;

        for i in 0..self.state.len() {
            for j in 0..self.state[i].len() {
                // copy state from previous iteration to next
                next.state[i][j] = self.state[i][j].clone();

                let count = self.count_neigbours(i, j);

                // Any live cell with fewer than two live neighbors dies, as if by underpopulation.
                if count < 2 {
                    next.state[i][j].state = None;
                    next.state[i][j].declare_dead();
                }
                // Any live cell with two or three live neighbors lives on to the next generation.
                if count >= 4 {
                    next.state[i][j].state = None;
                    next.state[i][j].declare_dead();
                }
                // Any live cell with more than three live neighbors dies, as if by overpopulation.
                if count == 3 {
                    next.state[i][j].state = Some(());
                    next.state[i][j].declear_survival();
                }

                if next.state[i][j].state.is_some() {
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

                if b[row][col].state.is_some() {
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
            row.iter().for_each(|cell| match cell.state {
                None => out.push(' '),
                Some(_) => out.push_str(&random_foreground_color("X")),
                _ => unreachable!("cells can only be in the state of dead (=0) or alive (=1)"),
            });
            out.push('\n');
        });

        write!(f, "{}", out)
    }
}

fn wrap(i: usize, dimension: usize) -> usize {
    (i + dimension) % dimension
}
