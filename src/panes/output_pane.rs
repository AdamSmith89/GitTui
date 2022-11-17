use crate::{panes::pane::*, eventing::{subscriber::Subscriber, events::Event}};
use tui::widgets::{Block, Borders, Paragraph};

pub struct OutputView {
    area: Rect,
}

impl OutputView {
    pub fn new(area: Rect) -> Self {
        OutputView { area }
    }
}

impl Pane for OutputView {
    fn draw(&self, frame: &mut Frame<CrosstermBackend<io::Stdout>>) {
        let widget = Paragraph::new("command output goes here")
            .block(Block::default().title("Output").borders(Borders::ALL));

        frame.render_widget(widget, self.area);
    }
}

pub struct OutputModel {

}

impl OutputModel {
    pub fn new() -> Self {
        OutputModel {}
    }
}

impl Subscriber for OutputModel {
    fn callback(&mut self, event: &Event) {

    }
}
