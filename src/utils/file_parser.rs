use crate::cli::cli_model::App;
use std::io::{self, prelude::*, BufReader};
use std::path::Path;
use std::{fs, fs::File};

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
    //1st arg is always path
    let args = std::env::args().collect::<Vec<String>>();
    let path = &String::from(&args[1]);

    let should_recurse = if args.len() > 2 { true } else { false };
    if should_recurse {
        //Recurse directory search
        let _ = visit_dirs_recurs(Path::new(path), app);
    } else {
        //Search
        let _rs = run_search(path, app);
    }

    //Clean up inputs
    app.search.clear();
    Ok(())
}

///Execute search on specified file path and update App state
fn run_search(path: &String, app: &mut App) -> io::Result<()> {
    if let Ok(file) = File::open(path) {
        let reader = BufReader::new(file);
        let mut line_number = 0;

        for line in reader.lines() {
            line_number += 1;

            if let Some(position) = line?
                .to_lowercase()
                .find(&app.search.as_str().to_lowercase())
            {
                //push found line to results
                app.items.push(vec![
                    path.clone(),
                    line_number.to_string(),
                    (position + 1).to_string(),
                ]);
                //increment total hits
                app.hits = app.hits + 1;
            }
        }
    }
    Ok(())
}

///see https://doc.rust-lang.org/std/fs/fn.read_dir.html
/// call example
///```
///let _res = visit_dirs_recurs(Path::new("D:\\Me\\Git\\grepper\\src"))
///```
fn visit_dirs_recurs(dir: &Path, app: &mut App) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let path = entry?.path();
            //recurse nested dirs
            if path.is_dir() {
                visit_dirs_recurs(&path, app)?;
            } else if path.is_file() {
                // println!("{:?} >> is FILE!\n", &path);
                let _search = run_search(&path.into_os_string().into_string().unwrap(), app);
            }
        }
    }
    Ok(())
}

///Gets extension from filename
fn _get_extension(filename: &str) -> Option<&str> {
    Path::new(filename)
        .extension()
        .and_then(std::ffi::OsStr::to_str)
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
