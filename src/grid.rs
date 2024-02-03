use crossterm::{
    cursor, execute,
    style::{Color, Print, SetBackgroundColor},
    terminal,
};
use rand::Rng;
use std::io::{self, Stdout, Write};

pub struct Grid {
    grid: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

impl Grid {
    /// Creates a new Grid
    ///
    /// # Panics
    ///
    /// Panics if the width or the height is less than 1
    pub fn new(width: usize, height: usize) -> Self {
        assert!(width > 0);
        assert!(height > 0);

        let grid = vec![vec![false; width]; height];

        Grid {
            grid,
            width,
            height,
        }
    }

    /// Sets the given cell to true if the cell is in range
    ///
    /// # Parameters
    ///
    /// - `x` The column of the cell to toggle, with 0 being the leftmost cell
    /// - `y` The row of the cell to toggle, with 0 being the top of the screen
    pub fn toggle_cell(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            self.grid[y][x] = true;
        }
    }

    /// Updates and draws the state of the grid, making particles fall
    pub fn update(&mut self) -> io::Result<()> {
        let mut stdout = io::stdout();

        // Couple of notes
        // - Skipping the bottom row as it will never need updating
        // - Reversing the ranges to go bottom up so that a particle doesn't get moved more than
        //   once
        for y in (0..(self.height - 1)).rev() {
            for x in (0..self.width).rev() {
                let cell = self.grid[y][x];

                let cell_below = self.grid[y + 1][x];

                if cell {
                    if !cell_below {
                        self.grid[y][x] = false;
                        erase_particle(&mut stdout, x as u16, y as u16)?;

                        self.grid[y + 1][x] = true;
                        draw_particle(&mut stdout, x as u16, (y + 1) as u16)?;
                    } else {
                        let left_chance = rand::thread_rng().gen_range(1..=100) > 50;

                        if left_chance && x > 0 && !self.grid[y + 1][x - 1] {
                            self.grid[y][x] = false;
                            erase_particle(&mut stdout, x as u16, y as u16)?;

                            self.grid[y + 1][x - 1] = true;
                            draw_particle(&mut stdout, (x - 1) as u16, (y + 1) as u16)?;
                        }

                        if x < (self.width - 1) && !self.grid[y + 1][x + 1] {
                            self.grid[y][x] = false;
                            erase_particle(&mut stdout, x as u16, y as u16)?;

                            self.grid[y + 1][x + 1] = true;
                            draw_particle(&mut stdout, (x + 1) as u16, (y + 1) as u16)?;
                        }
                    }
                }
            }
        }

        stdout.flush()
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new(80, 50)
    }
}

/// Draws a particle at the given coordinates
///
/// **NOTE** A particle is actually two chars wide, to make a square.
///          This is accounted for in the function.
///
/// # Parameters
///
/// - `x` The column of the cell to draw, with 0 being the leftmost cell
/// - `y` The row of the cell to draw, with 0 being the top of the screen
fn draw_particle(stdout: &mut Stdout, x: u16, y: u16) -> io::Result<()> {
    execute!(
        stdout,
        cursor::MoveTo(x * 2, y),
        SetBackgroundColor(Color::White),
        Print("  "),
        SetBackgroundColor(Color::Reset),
    )?;

    stdout.flush()
}

/// Erases a particle at the given coordinates
///
/// **NOTE** A particle is actually two chars wide, to make a square.
///          This is accounted for in the function.
///
/// # Parameters
///
/// - `x` The column of the cell to draw, with 0 being the leftmost cell
/// - `y` The row of the cell to draw, with 0 being the top of the screen
fn erase_particle(stdout: &mut Stdout, x: u16, y: u16) -> io::Result<()> {
    execute!(
        stdout,
        cursor::MoveTo(x * 2, y),
        SetBackgroundColor(Color::Reset),
        Print("  "),
    )?;

    stdout.flush()
}

#[cfg(test)]
mod tests {
    use super::Grid;

    #[test]
    fn grid_new_works() {
        let grid = Grid::new(3, 2);

        let expected = vec![vec![false, false, false], vec![false, false, false]];

        assert_eq!(grid.grid, expected);
    }

    #[test]
    fn toggle_cell_works() {
        let mut grid = Grid::new(3, 3);

        grid.toggle_cell(1, 2);

        let expected = vec![
            vec![false, false, false],
            vec![false, false, false],
            vec![false, true, false],
        ];

        assert_eq!(grid.grid, expected);
    }
}
