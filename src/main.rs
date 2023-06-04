use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    io,
    process::Command,
    time::{Duration, Instant},
    fmt::{self, Display}
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame, Terminal,
    text::Spans,
    text::Span
};

#[derive(Debug)]
struct Utf8ConversionError {
    description: String,
}

impl Error for Utf8ConversionError {}

impl Display for Utf8ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

struct StatefulList<T> {
    state: ListState,
    items: Vec<Result<T, Box<dyn Error>>>,
    error_descriptions: Vec<String>,
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<Result<T, Box<dyn Error>>>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
            error_descriptions: Vec::new(),
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn add_item(&mut self, item: Result<T, Box<dyn Error>>) {
        let description = match &item {
            Ok(_) => String::new(),
            Err(err) => format!("{}", err),
        };

        self.items.push(item);
        self.error_descriptions.push(description);
    }
}

struct App {
    items: StatefulList<String>,
}

impl App {
    fn new() -> App {
        App {
            items: StatefulList::with_items(vec![]),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let output = Command::new("cargo")
        .args(&["build"])
        .output()
        .map_err(Box::new)?;

    let stdout = String::from_utf8(output.stdout)?;
    let stderr = String::from_utf8(output.stderr)?;
    // let stderr_bytes = stderr.as_bytes();

    let mut app = App::new();
    app.items.add_item(Ok(stdout));
    app.items.add_item(Err(Box::new(Utf8ConversionError {
        description: stderr.to_string(),
    })));

    // Initialize terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run application
    let tick_rate = Duration::from_millis(250);
    let res = run_app(&mut terminal, app, tick_rate);

    // Shutdown terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    let mut printed_message = false; // Flag to track if the message has been printed

    loop {
        terminal.draw(|f| ui(f, &mut app, &mut printed_message))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Down => app.items.next(),
                    KeyCode::Up => app.items.previous(),
                    _ => {}
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }

        // Check if the message has been printed and break the loop
        if printed_message {
            return Ok(());
        }
    }
}


fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App, printed_message: &mut bool) {
    // Filter out items that are not errors or warnings
    let filtered_items: Vec<_> = app
        .items
        .items
        .iter()
        .zip(app.items.error_descriptions.iter())
        .filter(|&(item, _)| {
            if let Ok(_) = item {
                false // Exclude items that are Ok (success)
            } else if let Err(err) = item {
                let err_str = err.to_string();
                err_str.contains("error") || err_str.contains("warning")
            } else {
                false
            }
        })
        .collect();

    // Check if there is at least one item with a non-empty description
    let should_render_tui = filtered_items.iter().any(|(_, description)| !description.is_empty());

    if should_render_tui {
        // Create list items from the filtered items
        let items: Vec<ListItem> = filtered_items
            .iter()
            .map(|(_, description)| {
                let lines = vec![Spans::from(description.as_str())];
                ListItem::new(lines)
                    .style(Style::default().fg(Color::Black).bg(Color::White))
            })
            .collect();

        // Create a List from the filtered list items
        let list_item_style = Style::default()
            .fg(Color::White) // Text color
            .bg(Color::Black); // Background color

        let highlight_style = Style::default().fg(Color::Black).bg(Color::LightGreen);

        let items = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Errors"))
            .highlight_style(highlight_style)
            .highlight_symbol("-> ")
            .style(list_item_style);

        // Create a Paragraph widget for the description
        let description_widget = if let Some(selected) = app.items.state.selected() {
            if let Some(description) = app.items.error_descriptions.get(selected) {
                Paragraph::new(description.as_str())
                    .block(Block::default().borders(Borders::ALL).title("Description"))
            } else {
                Paragraph::new("")
            }
        } else {
            Paragraph::new("")
        };

        // Render the List widget and description widget
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(f.size());

        f.render_stateful_widget(items, chunks[0], &mut app.items.state);
        f.render_widget(description_widget, chunks[1]);
    } else {
        // If there are no items with non-empty descriptions, print a message only once
        if !*printed_message {
            println!("No errors or warnings to display.");
            *printed_message = true; // Set the flag to true
        }
    }
}
