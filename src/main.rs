use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame, Terminal,
};

struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
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

    fn unselect(&mut self) {
        self.state.select(None);
    }
}

struct App<'a> {
    items: StatefulList<(&'a str, usize)>,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            items: StatefulList::with_items(vec![
                ("Item0", 1),
                ("Item1", 2),
                ("Item2", 1),
                ("Item3", 3),
                ("Item4", 1),
                ("Item5", 4),
            ]),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run application
    let tick_rate = Duration::from_millis(250);
    let app = App::new();
    let res = run_app(&mut terminal, app, tick_rate);

    // Shutdown terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    // Return errors (if exist)
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
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Left => app.items.unselect(),
                    KeyCode::Down => app.items.next(),
                    KeyCode::Up => app.items.previous(),
                    _ => {}
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    // Create two chunks with equal horizontal screen space
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());

    // Create a List from all list items and highlight the currently selected one
    let items = List::new(app.items.items)
        .block(Block::default().borders(Borders::ALL).title("List"))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    // We can now render the item list
    f.render_stateful_widget(items, chunks[0], &mut app.items.state);
}

//fn main() -> Result<(), io::Error> {
//    // Disables the default modes in a terminal
//    enable_raw_mode()?;

//    // Enable stdout to handle the standard output of the current process
//    let mut stdout = io::stdout();

//    //Immediately switch to alternate screen and read mouse
//    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

//    // Initialize tui application and clear drawings
//    let backend = CrosstermBackend::new(stdout);
//    let mut terminal = Terminal::new(backend)?;
//    terminal.clear().unwrap();

//    // Create app
//    let tick_rate = Duration::from_millis(250);
//    let app = App::new();
//    let res

//    // Start drawing
//    terminal.draw(|f| {
//      let chunks = Layout::default()
//            .direction(Direction::Horizontal)
//            .margin(1)
//            .constraints(
//                [
//                    Constraint::Percentage(50),
//                    Constraint::Percentage(50),
//                ].as_ref()
//            )
//            .split(f.size());

//        let block = Block::default()
//             .title("Results")
//             .borders(Borders::ALL);
//        f.render_widget(block, chunks[0]);

//        let block = Block::default()
//             .title("Preview")
//             .borders(Borders::ALL);
//        f.render_widget(block, chunks[1]);
//        }
//    )?;

//    thread::sleep(Duration::from_millis(5000));

//    // Restore terminal
//    disable_raw_mode()?;
//    execute!(
//        terminal.backend_mut(),
//        LeaveAlternateScreen,
//        DisableMouseCapture
//    )?;
//    terminal.show_cursor()?;
//    Ok(())
//}
