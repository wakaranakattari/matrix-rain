use ratatui::{Frame, buffer::Buffer, layout::Rect, widgets::Widget};

use crate::app::App;

pub fn draw(frame: &mut Frame, app: &App) {
    frame.render_widget(RainWidget { app }, frame.area());
}

struct RainWidget<'a> {
    app: &'a App,
}

impl Widget for RainWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let theme = self.app.theme();

        for drop in &self.app.drops {
            let col = drop.col as usize;

            let (start, end) = drop.row_range();

            for row in start..=end {
                if row < 0 || row >= self.app.height as i32 {
                    continue;
                }

                let (ch, level) = drop.cell(row);

                let x = area.x + col as u16;
                let y = area.y + row as u16;

                if x < area.right() && y < area.bottom() {
                    let cell = &mut buf[(x, y)];
                    cell.set_char(ch);
                    cell.set_style(theme.style(level));
                }
            }
        }
    }
}
