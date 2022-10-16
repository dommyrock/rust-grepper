mod utils;
mod cli;
use crate::utils::{arg_parser, file_parser};
use crate::cli::composition;
use std::{error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    arg_parser::parse_args();
    let _ = file_parser::parse_files();

    let _ =composition::setup_cli();
    Ok(())
}