use std::io::{stdout, Result};

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    layout::{Alignment, Constraint, Direction, Layout},
    style::Stylize,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

use crate::ysnp::YSNP;

pub fn run_tui(_: &YSNP) -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    // TODO main loop
    loop {
        terminal.draw(|frame| {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(frame.size());

            frame.render_widget(
                Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                    .white()
                    .block(
                        Block::new()
                            .borders(Borders::ALL)
                            .title("Status")
                            .title_alignment(Alignment::Center),
                    ),
                layout[0],
            );
        })?;
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
