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
            eprintln!("cd: invalid directory {dir}");
        }
    } else {
        eprintln!("cd: incorrect args");
    }
}

fn pwd(proc: &Process) {
    if proc.args.is_empty() {
        let path = env::current_dir().expect("unable to find current directory");
        println!("{}", path.display());
    } else {
        eprintln!("pwd: too many args");
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
    let mut output = String::new();

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

            output.push_str(arg);
            output.push(' ');
        }
    }

    if flag_e {
        interp_echo(&output);
    } else {
        print!("{output}");
    }

    if !flag_n {
        println!();
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
fn interp_echo(str: &str) {
    let mut output = String::new();
    let chars: Vec<(usize, char)> = str.char_indices().collect();
    let mut i = 0;
    let mut chars_to_delete = 0;
    let mut curr_line_len = 0;

    while i < chars.len() {
        let (_idx, c) = chars[i];

        if c == '\\' && i + 1 < chars.len() {
            let (_next_idx, next) = chars[i + 1];

            match next {
                '\\' => {
                    output.push('\\');
                }
                'b' => {
                    output.pop();
                    curr_line_len -= 1;
                }
                'c' => {
                    print!("{output}");
                    return;
                }
                'e' => chars_to_delete += 1,
                'f' => {
                    output.push('\n');
                    for _ in 0..curr_line_len {
                        output.push(' ');
                    }
                }
                'n' => print!("TODO!"),
                'r' => print!("TODO!"),
                'v' => print!("TODO!"),
                '0' => print!("TODO!"),
                'x' => print!("TODO!"),
                _ => eprintln!("echo: invalid special character {next}"),
            }

            i += 2;
        } else {
            if c != '\\' {
                output.push(c);
                curr_line_len += 1;
            }
            if chars_to_delete > 0 {
                output.pop();
                chars_to_delete -= 1;
                curr_line_len -= 1;
            }
            i += 1;
        }
    }

    print!("{output}");
}
