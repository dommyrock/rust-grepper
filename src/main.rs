mod utils;
use crate::utils::{arg_parser,file_parser};

fn main() {
    arg_parser::parse_args();
    let _res = file_parser::parse_files();
}