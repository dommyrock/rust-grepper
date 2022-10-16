use tui::widgets::TableState;
use crate::utils::{cmd_executor};

pub enum WidgetMode {
    SearchResults,
    Search,
}

pub struct App<'a> {
    pub state: TableState,
    pub items: Vec<Vec<&'a str>>,
    pub view: WidgetMode,
    pub search: String,
}

impl<'a> App<'a> {
    pub fn new(itms: Vec<Vec<&'a str>>) -> App<'a> {
        App {
            state: TableState::default(),
            items: itms,
            search: String::new(),
            view: WidgetMode::SearchResults,
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
                let path = format!("{abs_file_path}:{line}:{at_char}");
                let _res = cmd_executor::exec_external_cmd(&path);
            }
            println!("CURRENT INTEX >> [{}]", row_idx);
        }
    }
}
