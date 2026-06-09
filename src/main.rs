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

        loop {
            input = input.trim().to_string();
            let last_char = input.chars().nth(input.len() - 1).unwrap();
            if last_char == '\\' {
                let second_to_last_char = input.chars().nth(input.len() - 2).unwrap();
                if second_to_last_char == '\\' {
                    break;
                }

                input.pop();
                print!("> ");
                io::stdout().flush().unwrap();

                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
            } else {
                break;
            }
        }

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
