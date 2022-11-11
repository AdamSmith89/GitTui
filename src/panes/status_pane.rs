use crate::panes::pane::*;
use tui::widgets::{Block, Borders, Paragraph};

pub struct StatusView {
    area: Rect,
}

impl StatusView {
    pub fn new(area: Rect) -> Self {
        StatusView { area }
    }
}

impl Pane for StatusView {
    fn draw(&self, frame: &mut Frame<CrosstermBackend<io::Stdout>>) {
        let widget = Paragraph::new("status info goes here")
            .block(Block::default().title("Status").borders(Borders::ALL));

        frame.render_widget(widget, self.area);
    }
}
