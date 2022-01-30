use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::process::Command;
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame, Terminal,
};

struct App {
    scroll: u16,
}

impl App {
    fn new() -> App {
        App { scroll: 0 }
    }

    fn on_tick(&mut self) {
        self.scroll += 1;
        self.scroll %= 10;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let tick_rate = Duration::from_millis(250);
    let app = App::new();
    let res = run_app(&mut terminal, app, tick_rate);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();

    // run a Command and retrieve stdout as bytes
    let c = Command::new("gls")
        .arg("-la")
        .arg("--color=always")
        .arg("/")
        .output()
        .unwrap();

    // render bytes to tui `Text`, converting ansi escape codes to tui `Style`s
    let text = ansi4tui::bytes_to_text(c.stdout);

    loop {
        terminal.draw(|f| ui(f, &app, text.clone()))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App, text: tui::text::Text) {
    let size = f.size();

    let block = Block::default().style(Style::default().bg(Color::Black).fg(Color::White));
    f.render_widget(block, size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ]
            .as_ref(),
        )
        .split(size);

    let create_block = |title| {
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Black).fg(Color::White))
            .title(Span::styled(
                title,
                Style::default().add_modifier(Modifier::BOLD),
            ))
    };
    let paragraph = Paragraph::new(text.clone())
        .style(Style::default().bg(Color::Black).fg(Color::White))
        .block(create_block("Left, no wrap"))
        .alignment(Alignment::Left);
    f.render_widget(paragraph, chunks[0]);
    let paragraph = Paragraph::new(text.clone())
        .style(Style::default().bg(Color::Black).fg(Color::White))
        .block(create_block("Left, wrap"))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, chunks[1]);
    let paragraph = Paragraph::new(text.clone())
        .style(Style::default().bg(Color::Black).fg(Color::White))
        .block(create_block("Center, wrap"))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
        .scroll((app.scroll, 0));
    f.render_widget(paragraph, chunks[2]);
    let paragraph = Paragraph::new(text)
        .style(Style::default().bg(Color::Black).fg(Color::White))
        .block(create_block("Right, wrap"))
        .alignment(Alignment::Right)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, chunks[3]);
}
