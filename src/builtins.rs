use crate::process::Process;

use std::env;
use std::path::Path;
use string_replace_all::string_replace_all;

static BUILTINS: [&str; 5] = ["cd", "exit", "pwd", "echo", "clear"];

pub fn is_builtin(cmd: &str) -> bool {
    if BUILTINS.contains(&cmd) {
        return true;
    }
    false
}

pub fn run_builtin(proc: &Process) -> i32 {
    let cmd = &proc.cmd;
    if cmd == "exit" {
        return 1;
    } else if cmd == "cd" {
        chdir(proc);
    } else if cmd == "pwd" {
        pwd(proc);
    } else if cmd == "echo" {
        println!("TODO!");
    } else if cmd == "clear" {
        clear();
    }

    0
}

fn get_home() -> Result<String, String> {
    match env::home_dir() {
        Some(home) => Ok(home
            .to_str()
            .expect("unable to convert home directory to str")
            .to_string()),
        None => Err("can't find home :/".to_string()),
    }
}

fn chdir(proc: &Process) {
    let home_dir = match get_home() {
        Ok(home) => home,
        Err(error) => {
            println!("{error}");
            return;
        }
    };

    if proc.args.is_empty() {
        let path = Path::new(&home_dir);
        assert!(env::set_current_dir(path).is_ok());
    } else if proc.args.len() == 1 {
        let dir = &proc.args[0];
        let expanded_path = string_replace_all(dir, "~", &home_dir);
        let path = Path::new(&expanded_path);
        if path.is_dir() {
            assert!(env::set_current_dir(path).is_ok());
        } else {
            println!("cd: invalid directory {dir}");
        }
    } else {
        println!("cd: incorrect args");
    }
}

fn pwd(proc: &Process) {
    if proc.args.is_empty() {
        let path = env::current_dir().expect("unable to find current directory");
        println!("{}", path.display());
    } else {
        println!("pwd: too many args");
    }
}

fn clear() {
    /*
     * https://stackoverflow.com/a/62101709
     * \x1B is the ESC character (ASCII 27).
     * [2J clears the entire terminal screen.
     * [1;1H moves the cursor to row 1, column 1 (top-left).
     */
    print!("\x1B[2J\x1B[1;1H");
}
