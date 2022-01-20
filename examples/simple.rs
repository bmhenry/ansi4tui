mod util;

use std::process::Command;

use util::event::{Event, Events};
use std::{error::Error, io};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    style::Style,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

fn main() -> Result<(), Box<dyn Error>> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();

    loop {
        terminal.draw(|f| {
            let size = f.size();

            // run a Command and retrieve stdout as bytes
            let c = Command::new("bat")
                .arg("--color=always")
                .arg("/Users/uesugi/dotfiles/bin/rot.py")
                .output()
                .unwrap();

            // render bytes to tui `Text`, converting ansi escape codes to tui `Style`s
            let text = ansi4tui::bytes_to_text(c.stdout);

            // display text
            let paragraph = Paragraph::new(text)
                .style(Style::default())
                .block(Block::default().title("Preview").borders(Borders::ALL));

            f.render_widget(paragraph, size);
        })?;

        if let Event::Input(key) = events.next()? {
            if key == Key::Char('q') {
                break;
            }
        }
    }
    Ok(())
}
