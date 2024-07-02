use std::{error::Error, io::stdout};

use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Stylize,
    widgets::{
        block::{Position, Title},
        Block, BorderType, Borders, Padding, Paragraph,
    },
    Frame, Terminal, TerminalOptions, Viewport,
};

use crate::ysnp::YSNP;

pub struct Tui<'a> {
    ysnp: &'a YSNP,
}

impl Tui<'_> {
    pub fn new(ysnp: &YSNP) -> Tui {
        Tui { ysnp }
    }

    pub fn run_tui(&self) -> Result<(), Box<dyn Error>> {
        // stdout().execute(EnterAlternateScreen)?;
        enable_raw_mode()?;
        let mut terminal = Terminal::with_options(
            CrosstermBackend::new(stdout()),
            TerminalOptions {
                viewport: Viewport::Inline(48),
            },
        )?;
        // let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        // terminal.clear()?;

        self.run_ui_loop(&mut terminal);

        // stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }

    fn run_ui_loop<B: Backend>(&self, terminal: &mut Terminal<B>) -> Result<(), Box<dyn Error>> {
        loop {
            // TODO: Split this to two parts/fxns: draw and poll / handle_events
            terminal.draw(|frame| {
                let vertical_layout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(vec![Constraint::Min(0)])
                    .margin(3)
                    .spacing(1)
                    .split(frame.size());

                let second_layout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(vec![Constraint::Length(12), Constraint::Min(8)])
                    .margin(3)
                    .spacing(1)
                    .split(vertical_layout[0]);

                self.render_outer_block(frame, vertical_layout[0]);
                self.render_alerts(frame, second_layout[0]);
                self.render_paths(frame, second_layout[1]);
            })?;

            if event::poll(std::time::Duration::from_millis(16))? {
                if let event::Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                        break;
                    }
                }
            }
        }
        Ok(())
    }

    fn render_outer_block(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(
            Block::new()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::proportional(1))
                .title("YouShellNotPath")
                .title(
                    Title::from("q: quit")
                        .position(Position::Bottom)
                        .alignment(Alignment::Center),
                )
                .title_alignment(Alignment::Center),
            area,
        )
    }

    fn render_alerts(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(
            Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                .white()
                .block(
                    Block::new()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .padding(Padding::proportional(1))
                        .title("Alerts")
                        .title_alignment(Alignment::Center),
                ),
            area,
        );
    }

    fn render_paths(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(Paragraph::new("Paths will be rendered here").white(), area);
    }
}
