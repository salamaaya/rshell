use crate::process::Process;

static BUILTINS: [&str; 5] = ["cd", "exit", "pwd", "echo", "clear"];

pub fn is_builtin(cmd: &str) -> bool {
    if BUILTINS.contains(&cmd) {
        return true;
    }
    return false;
}

pub fn run_builtin(proc: Process) -> i32 {
    let cmd = proc.cmd;
    if cmd == "exit" {
        return 1;
    } else if cmd == "cd" {
        println!("TODO!");
    } else if cmd == "pwd" {
        println!("TODO!");
    } else if cmd == "echo" {
        println!("TODO!");
    } else if cmd == "clear" {
        println!("TODO!");
    }

    0
}
