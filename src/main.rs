mod app;
mod raindrop;
mod theme;
mod ui;

use std::time::Instant;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};

use app::App;

const FRAME_TIME: std::time::Duration = std::time::Duration::from_millis(16);

fn main() -> anyhow::Result<()> {
    let mut terminal = ratatui::init();

    let size = terminal.size()?;
    let mut app = App::new(size.width, size.height);

    let mut last_frame = Instant::now();

    loop {
        let frame_start = Instant::now();

        while event::poll(std::time::Duration::ZERO)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => {
                            ratatui::restore();
                            return Ok(());
                        }
                        KeyCode::Tab => app.next_theme(),
                        _ => {}
                    }
                }
            }
        }

        let dt = (frame_start - last_frame).as_secs_f32().min(0.02);

        last_frame = frame_start;

        let size = terminal.size()?;
        app.resize(size.width, size.height);

        app.tick(dt);

        terminal.draw(|frame| {
            ui::draw(frame, &app);
        })?;

        let elapsed = frame_start.elapsed();

        if elapsed < FRAME_TIME {
            std::thread::sleep(FRAME_TIME - elapsed);
        }
    }
}
