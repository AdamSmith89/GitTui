use crate::panes::pane::*;
use tui::widgets::{Block, Borders, Paragraph};

pub struct InputView {
    area: Rect,
}

impl InputView {
    pub fn new(area: Rect) -> Self {
        InputView { area }
    }
}

impl Pane for InputView {
    fn draw(&self, frame: &mut Frame<CrosstermBackend<io::Stdout>>) {
        let widget =
            Paragraph::new("> ").block(Block::default().title("Input").borders(Borders::ALL));

        frame.render_widget(widget, self.area);
    }
}
