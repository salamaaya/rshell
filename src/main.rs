use rshell::lexer::lex;
use rshell::process::Process;
use rshell::process::run_cmd;

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

        let mut input_vec: Vec<&str> = input.split_whitespace().collect();
        let args = input_vec
            .split_off(1)
            .into_iter()
            .map(String::from)
            .collect();

        let proc = Process {
            cmd: input_vec[0].to_string(),
            args,
        };

        /* TESTING LEXER ONLY!!! */
        let tokens = lex(&input.to_string()).expect("failed to lex");
        print!("{:?}", tokens);

        let ret = run_cmd(&proc);
        if ret == 1 {
            break;
        }
    }
}
