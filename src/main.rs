mod process;
use crate::process::Process;
use crate::process::run_cmd;

use std::io;
use std::io::Write;

fn main() {
    loop {
        print!("rshell$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        let mut input: Vec<&str> = input.split_whitespace().collect();
        let args = input.split_off(1).into_iter().map(String::from).collect();

        let proc = Process {
            cmd: input[0].to_string(),
            args,
        };

        let exit = run_cmd(proc);
        if exit == 1 {
            break;
        }
    }
}
