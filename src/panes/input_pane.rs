use std::{sync::Mutex, rc::Rc, borrow::BorrowMut, cell::RefCell};

use crate::{panes::pane::*, eventing::{subscriber::Subscriber, events::Event}};
use crossterm::event::{KeyEvent, KeyCode};
use tui::widgets::{Block, Borders, Paragraph};

pub struct InputData {
    input_buffer: Vec<char>
}

impl InputData {
    pub fn new() -> Self {
        InputData { input_buffer: Vec::new() }
    }
}

pub struct InputView {
    area: Rect,
    data: Mutex<Rc<RefCell<InputData>>>,
}

impl InputView {
    pub fn new(data: Rc<RefCell<InputData>>, area: Rect) -> Self {
        InputView { area, data: Mutex::new(data) }
    }
}

impl Pane for InputView {
    fn draw(&self, frame: &mut Frame<CrosstermBackend<io::Stdout>>) {
        let data = self.data.lock().expect("Failed to lock input data mutex for draw");
        let text = String::from("> ") + &data.borrow().input_buffer.iter().collect::<String>();

        let widget =
            Paragraph::new(text).block(Block::default().title("Input").borders(Borders::ALL));

        frame.render_widget(widget, self.area);
    }
}

pub struct InputModel {
    data: Mutex<Rc<RefCell<InputData>>>,
}

impl InputModel {
    pub fn new(data: Rc<RefCell<InputData>>) -> Self {
        InputModel { data: Mutex::new(data) }
    }
}

impl Subscriber for InputModel {
    fn callback(&mut self, event: &Event) {
        match event {
            Event::KeyEvent(KeyEvent {
                code: KeyCode::Char(c), ..}) => {
                let data = self.data.lock().expect("Failed to lock input data for callback");
                data.as_ref().borrow_mut().input_buffer.push(*c);
                
                // let x = data.as_ref();
                // let mut y = x.borrow_mut();//.input_buffer.push(*c);
                // y.input_buffer.push(*c);
            }
            _ => (),
        }
    }
}
