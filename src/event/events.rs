pub enum Event {
    KeyEvent(crossterm::Event::KeyEvent),
    CommandEntered,
    CommandExecuted,
    CommandSelected,
}