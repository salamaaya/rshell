use crate::process::Process;

use lazy_regex::regex_replace_all;
use std::env;
use std::io;
use std::path::Path;
use string_replace_all::string_replace_all;
use unicode_segmentation::UnicodeSegmentation;

static BUILTINS: [&str; 5] = ["cd", "exit", "pwd", "echo", "clear"];

pub fn is_builtin(cmd: &str) -> bool {
    if BUILTINS.contains(&cmd) {
        return true;
    }
    false
}

pub fn run_builtin(out: &mut dyn io::Write, proc: &Process) {
    let cmd = &proc.cmd;
    if cmd == "exit" {
    } else if cmd == "cd" {
        chdir(proc);
    } else if cmd == "pwd" {
        pwd(out, proc);
    } else if cmd == "echo" {
        echo(out, proc);
    } else if cmd == "clear" {
        clear();
    }
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
            eprintln!("{error}");
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

fn pwd(out: &mut dyn io::Write, proc: &Process) {
    if proc.args.is_empty() {
        let path = env::current_dir().expect("unable to find current directory");
        assert!(writeln!(out, "{}", path.display()).is_ok());
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
fn echo(out: &mut dyn io::Write, proc: &Process) {
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
        interp_echo(out, &output);
    } else {
        assert!(write!(out, "{output}").is_ok());
    }

    if !flag_n {
        assert!(writeln!(out).is_ok());
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
fn interp_echo(out: &mut dyn io::Write, str: &str) {
    let output = str.replace("\\\\", "\\");
    let output = output.replace("\\a", "\u{07}");
    let output = output.replace("\\b", "\u{08}");
    let output = output.replace("\\e", "\u{1B}");
    let output = output.replace("\\f", "\u{0C}");
    let output = output.replace("\\n", "\n");
    let output = output.replace("\\r", "\r");
    let output = output.replace("\\t", "\t");
    let output = output.replace("\\v", "\u{0B}");

    // https://stackoverflow.com/a/68337748
    let output = regex_replace_all!(r#"\\0(\d{1,3})"#, &output, |_, num: &str| {
        let num: u32 = u32::from_str_radix(num, 8).unwrap();
        let c: char = std::char::from_u32(num).unwrap();
        c.to_string()
    });
    let output = regex_replace_all!(r#"\\x(\d{1,2})"#, &output, |_, num: &str| {
        let num: u32 = u32::from_str_radix(num, 16).unwrap();
        let c: char = std::char::from_u32(num).unwrap();
        c.to_string()
    });

    let output_vec: Vec<&str> = output.graphemes(true).collect();
    for i in 0..output_vec.len() {
        if i + 1 < output.len() && output_vec[i] == "\\" && output_vec[i + 1] == "c" {
            return;
        } else {
            assert!(write!(out, "{:}", output_vec[i]).is_ok());
        }
    }
}
