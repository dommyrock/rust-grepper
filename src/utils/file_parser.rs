use std::fs::File;
use std::io::{self, prelude::*, BufReader};

///Read file line by line
pub fn parse_files() -> io::Result<()> {
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

//File Reader example :
//https://stackoverflow.com/questions/45882329/read-large-files-line-by-line-in-rust
//https://stackoverflow.com/questions/31192956/whats-the-de-facto-way-of-reading-and-writing-files-in-rust-1-x
//fin string https://doc.rust-lang.org/std/primitive.str.html#method.find and return index