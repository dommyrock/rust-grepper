mod utils;
use crate::utils::{arg_parser, cmd_executor, file_parser};

fn main() {
    arg_parser::parse_args();
    let _res = file_parser::parse_files();
    
    //GOTO file at specific line
    let _res = cmd_executor::exec_external_cmd();
    //TODO: Ui:
    //https://github.com/fdehau/tui-rs/blob/master/examples/table.rs
    //https://github.com/fdehau/tui-rs/blob/master/examples/user_input.rs
}
