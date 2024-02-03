use crossterm::{
    cursor, execute,
    style::{Color, Print, SetBackgroundColor},
    terminal,
};
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

    /// Draws the grid to stdout
    pub fn draw(&self) -> io::Result<()> {
        let mut stdout = io::stdout();

        execute!(
            stdout,
            cursor::MoveTo(0, 0),
            terminal::Clear(terminal::ClearType::All)
        )?;

        for (y, row) in self.grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell {
                    draw_particle(&mut stdout, x.try_into().unwrap(), y.try_into().unwrap())?;
                }
            }
        }

        stdout.flush()?;

        Ok(())
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

    /// Updates the state of the grid, making particles fall
    pub fn update(&mut self) {
        for y in 0..(self.height - 1) {
            // Skipping the bottom row as it will never need updating
            for x in 0..self.width {
                let cell = self.grid[y][x];

                let cell_below = self.grid[y + 1][x];

                if cell {
                    if !cell_below {
                        self.grid[y][x] = false;
                        self.grid[y + 1][x] = true;
                    } else if x > 0 && !self.grid[y + 1][x - 1] {
                        self.grid[y][x] = false;
                        self.grid[y + 1][x - 1] = true;
                    } else if x < (self.width - 1) && !self.grid[y + 1][x + 1] {
                        self.grid[y][x] = false;
                        self.grid[y + 1][x + 1] = true;
                    }
                }
            }
        }
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
