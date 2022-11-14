use anyhow::Context;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    terminal,
};
use std::{
    io::{self, Stdout},
    time::Duration,
};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    Terminal,
};

use super::panes::pane::Pane;
use crate::panes::{
    input_pane::InputView, library_pane::LibraryView, output_pane::OutputView,
    status_pane::StatusView,
};

use crate::eventing::event_hub::*;

pub struct App<'a> {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    event_hub: EventHub<'a>,
    panes: Vec<Box<dyn Pane>>,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        App {
            terminal: Terminal::new(CrosstermBackend::new(io::stdout()))
                .expect("Failed to initialize terminal backend"),
            event_hub: EventHub::new(),
            panes: Vec::new(),
        }
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        self.setup_terminal().context("Failed to setup terminal")?;
        self.build_layout();

        loop {
            if !self.tick() {
                break;
            }
        }

        self.teardown_terminal()
            .context("Failed to teardown terminal")?;

        Ok(())
    }

    fn tick(&mut self) -> bool {
        if crossterm::event::poll(Duration::from_millis(1000)).is_ok() {
            match event::read().expect("Failed to read event") {
                Event::Key(KeyEvent {
                    code: KeyCode::Esc, ..
                }) => return false,
                _ => (),
            }

            // Todo: how to propagate error from here?
            let _ = self.terminal.draw(|frame| {
                for pane in &self.panes {
                    pane.draw(frame);
                }
            });
        }

        true
    }

    fn setup_terminal(&mut self) -> anyhow::Result<()> {
        terminal::enable_raw_mode().context("Failed to enable raw-mode in terminal")?;
        crossterm::execute!(
            io::stdout(),
            terminal::EnterAlternateScreen,
            event::EnableMouseCapture,
            cursor::EnableBlinking
        )
        .context("Failed to setup terminal backend")?;

        self.terminal.clear().context("Failed to clear terminal")?;
        self.terminal
            .hide_cursor()
            .context("Failed to hide cursor")?;

        Ok(())
    }

    fn build_layout(&mut self) {
        let main_areas = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(self.terminal.get_frame().size());
        let left_area = main_areas[0];
        let right_area = main_areas[1];

        let left_areas = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(3), Constraint::Max(3)].as_ref())
            .split(left_area);

        let right_areas = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(right_area);

        self.panes.push(Box::new(OutputView::new(left_areas[0])));
        self.panes.push(Box::new(InputView::new(left_areas[1])));
        self.panes.push(Box::new(StatusView::new(right_areas[0])));
        self.panes.push(Box::new(LibraryView::new(right_areas[1])));
    }

    fn teardown_terminal(&mut self) -> anyhow::Result<()> {
        terminal::disable_raw_mode().context("Failed to disable raw-mode in terminal")?;
        crossterm::execute!(
            self.terminal.backend_mut(),
            terminal::LeaveAlternateScreen,
            event::DisableMouseCapture
        )
        .context("Failed to reset terminal backend")?;
        self.terminal
            .show_cursor()
            .context("Failed to show cursor")?;

        Ok(())
    }
}
