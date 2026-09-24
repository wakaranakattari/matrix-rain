use ratatui::{Frame, buffer::Buffer, layout::Rect, style::Style, widgets::Widget};

use crate::app::App;

pub fn draw(frame: &mut Frame, app: &App) {
    frame.render_widget(RainWidget { app }, frame.area());
}

struct RainWidget<'a> {
    app: &'a App,
}

impl Widget for RainWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.is_empty() {
            return;
        }

        clear_area(area, buf);

        let styles = self.app.theme().styles();
        let height = area.height as i32;

        for drop in self.app.drops() {
            let x = area.x.saturating_add(drop.col());
            if x >= area.right() {
                continue;
            }

            let head = drop.head_row();
            let (start, end) = drop.row_range(head);
            if end < 0 || start >= height {
                continue;
            }

            let from = start.max(0);
            let to = end.min(height - 1);
            let (mid_end, dim_end) = drop.thresholds();

            for row in from..=to {
                let y = area.y + row as u16;
                let (ch, brightness) = drop.cell(row, head, mid_end, dim_end);
                let cell = &mut buf[(x, y)];
                cell.set_char(ch);
                cell.set_style(styles[brightness as usize]);
            }
        }
    }
}

fn clear_area(area: Rect, buf: &mut Buffer) {
    let blank = Style::default();
    for y in area.top()..area.bottom() {
        for x in area.left()..area.right() {
            let cell = &mut buf[(x, y)];
            cell.set_char(' ');
            cell.set_style(blank);
        }
    }
}
