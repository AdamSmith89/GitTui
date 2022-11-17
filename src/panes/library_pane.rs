use crate::{panes::pane::*, eventing::{subscriber::Subscriber, events::Event}};
use tui::widgets::{Block, Borders, Paragraph};

pub struct LibraryView {
    area: Rect,
}

impl LibraryView {
    pub fn new(area: Rect) -> Self {
        LibraryView { area }
    }
}

impl Pane for LibraryView {
    fn draw(&self, frame: &mut Frame<CrosstermBackend<io::Stdout>>) {
        let widget = Paragraph::new("command library goes here").block(
            Block::default()
                .title("Command Library")
                .borders(Borders::ALL),
        );

        frame.render_widget(widget, self.area);
    }
}

pub struct LibraryModel {

}

impl LibraryModel {
    pub fn new() -> Self {
        LibraryModel {}
    }
}

impl Subscriber for LibraryModel {
    fn callback(&mut self, event: &Event) {

    }
}
