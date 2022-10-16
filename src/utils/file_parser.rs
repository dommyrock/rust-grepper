use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use crate::cli::cli_model::App;

///Read file line by line
pub fn _parse_files() -> io::Result<()> {
    let keyword: &str = "GQL.Candidate";
    let file = File::open("D:\\Me\\Git\\grepper\\TODOOOOOOOOO.txt")?;

    let reader = BufReader::new(file);
    let mut line_number = 0;

    for line in reader.lines() {
        line_number += 1;

        if let Some(position) = line?.to_lowercase().find(&keyword.to_lowercase()) {
            println!(
                "Matched:: '{}' on char:: {:?} at line:: {}",
                keyword, position, line_number
            )
        }
    }
    Ok(())
}

///Parses docs into current app
///TODO: RIGHT NOW THIS HARDCODES ALL FOUND POSITIONS TO LINE 45 ITEM
pub fn parse_into_app(app :&mut App) -> io::Result<()> {
    let keyword: &str = "GQL.Candidate";
    let file = File::open("D:\\Me\\Git\\grepper\\TODOOOOOOOOO.txt")?;

    let reader = BufReader::new(file);
    let mut line_number = 0;

    for line in reader.lines() {
        line_number += 1;

        if let Some(position) = line?.to_lowercase().find(&keyword.to_lowercase()) {
            // println!(
            //     "Matched:: '{}' on char:: {:?} at line:: {}",
            //     keyword, position, line_number
            // );
            //push found line to results
            app.items.push(vec!["D:\\Me\\Git\\grepper\\TODOOOOOOOOO.txt", "26","0"]);//lifetimes get in the way here
        }
    }
    Ok(())
}

//File Reader example :
//https://stackoverflow.com/questions/45882329/read-large-files-line-by-line-in-rust
//https://stackoverflow.com/questions/31192956/whats-the-de-facto-way-of-reading-and-writing-files-in-rust-1-x
//fin string https://doc.rust-lang.org/std/primitive.str.html#method.find and return index