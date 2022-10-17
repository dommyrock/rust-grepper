use crate::cli::cli_model::App;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

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
pub fn parse_into_app(app: &mut App) -> io::Result<()> {
    let keyword: &str = "GQL.Candidate";
    let file = File::open("D:\\Me\\Git\\grepper\\TODOOOOOOOOO.txt")?;

    let reader = BufReader::new(file);
    let mut line_number = 0;

    for line in reader.lines() {
        line_number += 1;

        if let Some(position) = line?.to_lowercase().find(&keyword.to_lowercase()) {
            //push found line to results
            let p = String::from("D:\\Me\\Git\\grepper\\TODOOOOOOOOO.txt");
            app.items
                .push(vec![p, line_number.to_string(), position.to_string()]);
        }
    }
    Ok(())
}
/*lifetime docs
    https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime/fn.html
    https://stackoverflow.com/questions/54488127/getting-temporary-value-dropped-while-borrowed-when-trying-to-update-an-option
    https://stackoverflow.com/questions/54056268/temporary-value-is-freed-at-the-end-of-this-statement
    https://stackoverflow.com/questions/59540064/insert-constructed-string-into-vec-in-rust
    https://stackoverflow.com/questions/30154541/how-do-i-concatenate-strings
*/

//File Reader example :
//https://stackoverflow.com/questions/45882329/read-large-files-line-by-line-in-rust
//https://stackoverflow.com/questions/31192956/whats-the-de-facto-way-of-reading-and-writing-files-in-rust-1-x
//fin string https://doc.rust-lang.org/std/primitive.str.html#method.find and return index
