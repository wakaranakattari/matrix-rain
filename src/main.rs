mod app;
mod raindrop;
mod theme;
mod ui;

use std::time::{Duration, Instant};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;

use app::App;

const FRAME_TIME: Duration = Duration::from_millis(16);
const MAX_DT: f32 = 0.02;

fn main() -> anyhow::Result<()> {
    let mut terminal = ratatui::init();
    let result = run(&mut terminal);
    ratatui::restore();
    result
}

fn run(terminal: &mut DefaultTerminal) -> anyhow::Result<()> {
    let size = terminal.size()?;
    let mut app = App::new(size.width, size.height);
    let mut last_frame = Instant::now();

    loop {
        let frame_start = Instant::now();
        let dt = frame_start
            .duration_since(last_frame)
            .as_secs_f32()
            .min(MAX_DT);
        last_frame = frame_start;

        while event::poll(Duration::ZERO)? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Tab => app.next_theme(),
                    _ => {}
                },
                Event::Resize(width, height) => app.resize(width, height),
                _ => {}
            }
        }

        app.tick(dt);

        terminal.draw(|frame| {
            ui::draw(frame, &app);
        })?;

        if let Some(remaining) = FRAME_TIME.checked_sub(frame_start.elapsed())
            && event::poll(remaining)?
        {
            continue;
        }
    }
}
