use crossterm::{
    cursor,
    event::{self, read, Event, KeyCode, MouseEventKind},
    execute, terminal,
};

use std::{io, time::Duration};

mod grid;
use grid::Grid;

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();

    let mut grid = Grid::default();

    terminal::enable_raw_mode()?;

    execute!(
        stdout,
        terminal::EnterAlternateScreen,
        cursor::Hide,
        event::EnableMouseCapture
    )?;

    loop {
        if event::poll(Duration::from_millis(16))? {
            match read()? {
                Event::Key(event) => {
                    if event.code == KeyCode::Char('q') {
                        break;
                    }
                }
                Event::Mouse(event) => {
                    if event.kind == MouseEventKind::Down(event::MouseButton::Left)
                        || event.kind == MouseEventKind::Drag(event::MouseButton::Left)
                    {
                        let x = (event.column / 2) as usize;
                        let y = event.row as usize;
                        grid.toggle_cell(x, y);
                    }
                }
                _ => (),
            }
        }

        grid.update();
        grid.draw()?;
    }

    terminal::disable_raw_mode()?;

    execute!(
        stdout,
        terminal::LeaveAlternateScreen,
        cursor::Show,
        event::DisableMouseCapture
    )?;

    Ok(())
}
