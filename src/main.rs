use crossterm::{cursor, execute, terminal};

use std::io;
use std::thread;

mod grid;
use grid::Grid;

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();

    let mut grid = Grid::default();

    grid.toggle_cell(3, 8);

    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    let _ = grid.draw();

    thread::sleep_ms(3000);

    execute!(stdout, terminal::LeaveAlternateScreen, cursor::Show)?;

    Ok(())
}
