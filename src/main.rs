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

        let lex_result = lex(&input.to_string());
        match lex_result {
            Err(e) => {
                eprintln!("{e}");
            }
            Ok(tokens) => {
                let _expr = parse(&tokens);
            }
        }
    }
}
