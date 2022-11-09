use std::{io::{Stdout, self}, time::Duration};
use anyhow::Context;
use crossterm::{terminal, cursor, event::{self, Event, KeyEvent, KeyCode}};
use tui::{backend::CrosstermBackend, Terminal, layout::{Layout, Direction, Constraint, Rect}, terminal::CompletedFrame, widgets::{Paragraph, Block, Borders}};

pub struct App {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    panes: Vec<Rect>,
}

impl App {
    pub fn new() -> Self {
        App {
            terminal: Terminal::new(CrosstermBackend::new(io::stdout())).expect("Failed to initialize terminal backend"),
            panes: Vec::new(),
        }
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        self.setup_terminal().context("Failed to setup terminal")?;
        self.setup_panes();

        loop {
            if !self.tick() {
                break;
            }
        }

        self.teardown_terminal().context("Failed to teardown terminal")?;

        Ok(())
    }

    fn tick(&mut self) -> bool {
        if crossterm::event::poll(Duration::from_millis(1000)).is_ok() {
            match event::read().expect("Failed to read event") {
                Event::Key(KeyEvent { code: KeyCode::Esc, .. }) => return false,
                _ => (),
            }
            
            // Todo: how to propagate error from here?
            let _ = self.terminal.draw(|frame| {
                for pane in &self.panes {
                    let widget = Paragraph::new(pane.area().to_string())
                        .block(Block::default().title(pane.area().to_string()).borders(Borders::ALL));
    
                    frame.render_widget(widget, *pane);
                }
                // output_pane.draw(frame);
                // command_pane.draw(frame);
                // status_pane.draw(frame);
                // search_pane.draw(frame, &output_pane);
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
        ).context("Failed to setup terminal backend")?;
        
        self.terminal.clear().context("Failed to clear terminal")?;
        self.terminal.hide_cursor().context("Failed to hide cursor")?;

        Ok(())
    }

    fn setup_panes(&mut self) {
        let main_areas = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(self.terminal.get_frame().size());
        let left_area = main_areas[0];
        let right_area = main_areas[1];

        let mut left_areas = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(3), Constraint::Max(3)].as_ref())
            .split(left_area);

        let mut right_areas = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(right_area);

        self.panes.append(&mut left_areas);
        self.panes.append(&mut right_areas);

        // let mut output_pane = OutputPane::new(left_areas[0]);
        // let mut command_pane = CommandPane::new(left_areas[1]);
        // let mut status_pane = StatusPane::new(right_areas[0]);
        // let mut search_pane = SearchPane::new(right_areas[1]);
    }

    fn teardown_terminal(&mut self) -> anyhow::Result<()> {
        terminal::disable_raw_mode().context("Failed to disable raw-mode in terminal")?;
        crossterm::execute!(
            self.terminal.backend_mut(),
            terminal::LeaveAlternateScreen,
            event::DisableMouseCapture
        ).context("Failed to reset terminal backend")?;
        self.terminal.show_cursor().context("Failed to show cursor")?;

        Ok(())
    }
}