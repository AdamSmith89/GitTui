use crate::{panes::pane::*, eventing::{subscriber::Subscriber, events::Event}};
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

pub struct StatusModel {

}

impl StatusModel {
    pub fn new() -> Self {
        StatusModel {}
    }
}

impl Subscriber for StatusModel {
    fn callback(&mut self, event: &Event) {

    }
}
