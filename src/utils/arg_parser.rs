use std::env::args;

///Parses input args 
pub fn parse_args() {
    let args = args().collect::<Vec<String>>();

    if args.iter().any(|f| f == "-f") {
        println!("{:#?}.", args);
    }
}
