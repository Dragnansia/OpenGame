use std::io;
use tui::{
    backend::TermionBackend,
    widgets::{Block, Borders},
    Terminal,
};

pub fn run_display() -> Result<(), io::Error> {
    let stdout = io::stdout();
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    'tui: loop {
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default().title("OpenGame").borders(Borders::ALL);

            f.render_widget(block, size);
        })?;

        if true {
            break 'tui;
        }
    }

    Ok(())
}
