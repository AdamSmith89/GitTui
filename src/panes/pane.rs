pub use std::io;
pub use tui::{backend::CrosstermBackend, layout::Rect, Frame};

pub trait Pane {
    fn draw(&self, frame: &mut Frame<CrosstermBackend<io::Stdout>>);
}
