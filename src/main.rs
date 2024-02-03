use crossterm::{cursor, execute, style::Color, terminal};
use std::{io::{self, Stdout, Write}, thread};

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();

    let grid = vec![
        vec![false, false, true],
        vec![false, true, false],
        vec![false, false, true],
    ];

    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    let _ = draw_grid(grid);

    thread::sleep_ms(3000);

    execute!(stdout, terminal::LeaveAlternateScreen, cursor::Show)?;


    Ok(())
}

fn draw_grid(grid: Vec<Vec<bool>>) -> io::Result<()> {
    let mut stdout = io::stdout();

    execute!(
        stdout,
        cursor::MoveTo(0, 0),
        terminal::Clear(terminal::ClearType::All)
    )?;

    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell {
                draw_particle(&mut stdout, x.try_into().unwrap(), y.try_into().unwrap())?;
            }
        }
    }

    Ok(())
}

fn draw_particle(stdout: &mut Stdout, x: u16, y: u16) -> io::Result<()> {
    execute!(
        stdout,
        cursor::MoveTo(x * 2, y),
        crossterm::style::SetBackgroundColor(Color::White),
        crossterm::style::Print("  "),
        crossterm::style::SetBackgroundColor(Color::Reset),
    )?;

    Ok(())
}
