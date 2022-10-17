use std::process::Command;

///Executes external processes from current program
///
///See: https://stackoverflow.com/questions/62273768/couldnt-convert-the-error-to-stdioerror#:~:text=update%3A%20it%20was%20pointed%20out
pub fn exec_external_cmd(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let cmd = format!("code --goto {path}");
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args([
                "/C", &cmd, //open file on a specific FILE_PATH:Line:Character
            ])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("echo hello non WIN user")
            .output()
            .expect("failed to execute process")
    };

    String::from_utf8(output.stdout)?
        .lines()
        .for_each(|x| println!("{}", x));

    Ok(())
}
//Example 2 with git command that produces output
// Command::new("cmd")
// .args([
//     "/C",
//     "git status", //open file on a specific FILE_PATH:Line:Character
// ])
// .output()
// .expect("failed to execute process")

/*Also see:
    https://rust-lang-nursery.github.io/rust-cookbook/os/external.html#run-piped-external-commands
    https://stackoverflow.com/questions/7669963/how-can-i-get-a-side-by-side-diff-when-i-do-git-diff
    TLDR usefull command--> "git diff --word-diff=color"
*/
/*VS code CLI docs:
    https://code.visualstudio.com/docs/editor/command-line

 Rust lib to execute external commands from current proccess
    https://doc.rust-lang.org/std/process/struct.Command.html
    https://rust-lang-nursery.github.io/rust-cookbook/os/external.html
*/
