//Native
use crate::cli::{
    cli_model::{App, WidgetMode},
    ui,
};
use std::{error::Error, io};
//3rd party
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal
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
    let items = vec![
        vec!["Row11", "Row12", "Row13"],
        vec!["Row21", "Row22", "Row23"],
        vec!["Row31", "Row32", "Row33"],
        vec!["D:\\Me\\Git\\grepper\\TODOOOOOOOOO.txt", "26", "0"],
        vec!["Row51", "Row52", "Row53"],
        vec!["Row61", "Row62\nTest", "Row63"],
        vec!["Row71", "Row72", "Row73"],
        vec!["Row81", "Row82", "Row83"],
        vec!["Row91", "Row92", "Row93"],
        vec!["Row101", "Row102", "Row103"],
        vec!["Row111", "Row112", "Row113"],
        vec!["Row121", "Row122", "Row123"],
        vec!["Row131", "Row132", "Row133"],
        vec!["Row141", "Row142", "Row143"],
        vec!["Row151", "Row152", "Row153"],
        vec!["Row161", "Row162", "Row163"],
        vec!["Row171", "Row172", "Row173"],
        vec!["Row181", "Row182", "Row183"],
        vec!["Row191", "Row192", "Row193"],
    ];
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
                    // KeyCode::Enter => { v1
                    //     app.items.push(app.input.drain(..).collect());
                    // }
                    // KeyCode::Enter => {
                    //     app.items.push(vec!["D:\\Me\\Git\\grepper","27","0"]);
                    // }
                    //TODO: actual logic needs to run kw serch on some input location we got from cmd args
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