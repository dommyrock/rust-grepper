//Native
mod utils;
use crate::utils::{arg_parser, cmd_executor, file_parser};
//3rd Party
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table, TableState},
    Frame, Terminal,
};
//TODO: Ui:
//https://github.com/fdehau/tui-rs/blob/master/examples/user_input.rs
// then extract UI logic into its own module /file
fn main() -> Result<(), Box<dyn Error>> {
    arg_parser::parse_args();
    let _res = file_parser::parse_files();

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
        vec![
            "D:\\Me\\Git\\grepper\\TODOOOOOOOOO.txt",
            "26",
            "0",
        ],
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

struct App<'a> {
    state: TableState,
    items: Vec<Vec<&'a str>>,
}

impl<'a> App<'a> {
    fn new(itms: Vec<Vec<&'a str>>) -> App<'a> {
        App {
            state: TableState::default(),
            items: itms,
        }
    }
    pub fn next(&mut self) {
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

    pub fn previous(&mut self) {
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

    ///GOTO file at specific line in Vs code
    pub fn open_file_location(&mut self) {
        let current_row = self.state.selected();
        if let Some(row_idx) = current_row {
            let row_data = self.items.get(row_idx);

            if let Some(data) = row_data {
                let abs_file_path = data[0];
                let line = data[1];
                let at_char = data[2];
                
                println!("Current <FILE_PATH> is: {}", abs_file_path);
                let path =format!("{abs_file_path}:{line}:{at_char}");
                let _res = cmd_executor::exec_external_cmd(&path);
            }
            println!("CURRENT INTEX >> [{}]", row_idx);
        }
    }
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Tab => app.open_file_location(),
                KeyCode::Down => app.next(),
                KeyCode::Up => app.previous(),
                _ => {}
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let rects = Layout::default()
        .constraints([Constraint::Percentage(100)].as_ref())
        .margin(5)
        .split(f.size());

    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let normal_style = Style::default().bg(Color::Blue);
    let header_cells = ["Path", "line", "char"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::White)));
    let header = Row::new(header_cells)
        .style(normal_style)
        .height(1)
        .bottom_margin(1);
    let rows = app.items.iter().map(|item| {
        let height = item
            .iter()
            .map(|content| content.chars().filter(|c| *c == '\n').count())
            .max()
            .unwrap_or(0)
            + 1;
        let cells = item.iter().map(|c| Cell::from(*c));
        Row::new(cells).height(height as u16).bottom_margin(1)
    });
    let t = Table::new(rows)
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Search results"),
        )
        .highlight_style(selected_style.fg(Color::LightGreen))
        .highlight_symbol(">> ")
        .widths(&[
            Constraint::Percentage(85),
            Constraint::Length(10),
            Constraint::Min(5),
        ]);
    f.render_stateful_widget(t, rects[0], &mut app.state);
}
