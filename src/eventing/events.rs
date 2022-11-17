pub enum Event {
    KeyEvent(crossterm::event::KeyEvent),
    CommandEntered,
    CommandExecuted,
    CommandSelected,
}