//Native
use crate::cli::{
    cli_model::{App, WidgetMode},
    ui,
};
use crate::utils::file_parser;
use std::{error::Error, io};
//3rd party
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

///Sets up CLI custom tui_rs composition  
pub fn setup_cli() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let items = vec![];
    let app = App::new(items);
    let res = run_app(&mut terminal, app);

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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match app.view {
                WidgetMode::SearchResults => match key.code {
                    KeyCode::Tab => app.view = WidgetMode::Search,
                    KeyCode::Enter => app.open_file_location(),
                    KeyCode::Down => app.next(),
                    KeyCode::Up => app.previous(),
                    KeyCode::Char('q') => return Ok(()),
                    _ => {}
                },
                WidgetMode::Search => match key.code {
                    KeyCode::Enter => {
                        //reset previous app state
                        app.hits = 0;
                        app.items = vec![];
                        let _res = file_parser::parse_into_app(&mut app);
                    }
                    KeyCode::Char(c) => {
                        app.search.push(c);
                    }
                    KeyCode::Backspace => {
                        app.search.pop();
                    }
                    KeyCode::Tab => app.view = WidgetMode::SearchResults,
                    _ => {}
                },
            }
        }
    }
}
