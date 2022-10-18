use crate::utils::cmd_executor;
use tui::widgets::TableState;

pub enum WidgetMode {
    SearchResults,
    Search,
}

pub struct App<'a> {
    pub state: TableState,
    pub items: Vec<Vec<String>>,
    pub hits: u32,
    pub view: WidgetMode,
    pub search: String,
    pub not_used: Vec<Vec<&'a String>>, //to ssatisfy cotract
}

impl<'a> App<'a> {
    pub fn new(itms: Vec<Vec<String>>) -> App<'a> {
        App {
            state: TableState::default(),
            items: itms,
            hits: 0,
            search: String::new(),
            view: WidgetMode::Search,
            not_used: vec![],
        }
    }
    ///Navigate to next row
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

    ///Navigate to previous row
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
                if let (Some(pth), Some(ln), Some(char)) = (data.get(0), data.get(1), data.get(2)) {
                    let path = format!("{pth}:{ln}:{char}");
                    let _res = cmd_executor::exec_external_cmd(path);
                }
            }
        }
    }
}
