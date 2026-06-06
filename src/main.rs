use rshell::lexer::lex;
use rshell::parser::parse;

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

        let tokens = lex(&input.to_string()).expect("failed to lex");
        let expr = parse(&tokens);

        //let ret = run_cmd(&proc);
        //if ret == 1 {
        //    break;
        //}
    }
}
