use crossterm::{
    cursor,
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{self, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    Result,
};
use std::io::{stdout, Write};
use std::time::Duration;

fn main() -> Result<()> {
    let mut stdout = stdout();

    execute!(stdout, EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    execute!(stdout, EnableMouseCapture)?;

    let mut x = 0;
    let mut y = 0;

    loop {
        execute!(stdout, terminal::Clear(ClearType::All))?;
        execute!(stdout, cursor::MoveTo(x, y))?;
        print!("@");
        stdout.flush()?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Up => {
                        if y > 0 {
                            y -= 1;
                        }
                    }
                    KeyCode::Down => {
                        y += 1;
                    }
                    KeyCode::Left => {
                        if x > 0 {
                            x -= 1;
                        }
                    }
                    KeyCode::Right => {
                        x += 1;
                    }
                    KeyCode::Esc => break,
                    _ => {}
                }
            }
        }
    }

    execute!(stdout, LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    execute!(stdout, DisableMouseCapture)?;

    Ok(())
}
