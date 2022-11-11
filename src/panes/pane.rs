pub use std::io;
pub use tui::{Frame, backend::CrosstermBackend, layout::Rect};

pub trait Pane {
    fn draw(&self, frame: &mut Frame<CrosstermBackend<io::Stdout>>);
}
