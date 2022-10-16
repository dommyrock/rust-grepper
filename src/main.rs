mod utils;
mod cli;
use crate::cli::composition;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let _ = composition::setup_cli();
    Ok(())
}
