use rshell::lexer::lex;
use rshell::parser::parse;

use std::io;
use std::io::Write;

fn handle_input() {
    let mut input = String::new();

    loop {
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let mut input_cloned = input.clone();
        input_cloned = input_cloned.trim().to_string();
        if input_cloned.is_empty() {
            return;
        }

        let last_char = input_cloned.chars().nth(input_cloned.len() - 1).unwrap();
        if last_char == '\\' {
            let second_to_last_char = input_cloned.chars().nth(input_cloned.len() - 2).unwrap();
            if second_to_last_char == '\\' {
                break;
            }

            input_cloned.pop();
            input = input_cloned;
            print!("> ");
            io::stdout().flush().unwrap();
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

fn main() {
    loop {
        print!("rshell$ ");
        io::stdout().flush().unwrap();

        handle_input();
    }
}
