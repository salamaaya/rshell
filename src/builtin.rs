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
        echo(proc);
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

/*
* the possible options supported:
*   -n     do not output the trailing newline
*   -e     enable interpretation of backslash escapes
*   -E     disable interpretation of backslash escapes (default)
*/
fn echo(proc: &Process) {
    let mut flag_n = false;
    let mut flag_e = false;
    let mut found_flags = false;

    for arg in &proc.args {
        // first, find the flags
        // note: E is default anyways so we don't need to check for that
        if !found_flags && arg.starts_with('-') {
            if arg.contains("n") {
                flag_n = true;
            }
            if arg.contains("e") {
                flag_e = true;
            }
        } else {
            found_flags = true;

            if flag_e {
                interp_echo(arg);
            } else {
                print!("{arg} ");
            }
        }
    }

    if !flag_n {
        println!("");
    }
}

/*
* \\     backslash
* \a     alert (BEL)
* \b     backspace
* \c     produce no further output
* \e     escape
* \f     form feed
* \n     new line
* \r     carriage return
* \t     horizontal tab
* \v     vertical tab
* \0NNN  byte with octal value NNN (1 to 3 digits)
* \xHH   byte with hexadecimal value HH (1 to 2 digits)
*/
fn interp_echo(str: &String) {
    let mut prev = ' ';
    let mut output = String::from("");

    for (i, c) in str.char_indices() {
        if prev == '\\' && i > 0 {
            match c {
                '\\' => output.push(c),
                'b' => {
                    output.pop();
                    ()
                }
                'c' => print!("TODO!"),
                'e' => print!("TODO!"),
                'f' => print!("TODO!"),
                'n' => print!("TODO!"),
                'r' => print!("TODO!"),
                'v' => print!("TODO!"),
                '0' => print!("TODO!"),
                'x' => print!("TODO!"),
                _ => println!("echo: invalid special character {c}"),
            }
        } else if c != '\\' {
            output.push(c);
        }

        prev = c;
    }

    print!("{output}");
}
