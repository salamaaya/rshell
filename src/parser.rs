use crate::{
    lexer::Token,
    process::{Process, run_cmd},
};

use std::env;
use std::process::ExitStatus;

pub enum Operator {
    Pipe,                 // |
    RedirectInput,        // <
    RedirectOutput,       // >
    RedirectOutputAppend, // >>
    And,                  // &&
    Or,                   // ||
}

pub enum Node {
    Command {
        program: String,
        args: Vec<String>,
    },

    Subshell {
        command: String,
    },

    Redirect {
        op: Operator,
        command: Box<Node>,
        file: String,
    },

    Binary {
        op: Operator,
        left: Box<Node>,
        right: Box<Node>,
    },
}

pub fn parse(tokens: &[Token]) -> Result<ExitStatus, String> {
    let ast = build_ast(tokens)?;
    let exit_code = expr(&ast)?;
    Ok(exit_code)
}

fn build_ast(tokens: &[Token]) -> Result<Vec<Node>, String> {
    let mut i = 0;
    let len = tokens.len();
    let mut ast = Vec::new();

    while i < len {
        match &tokens[i] {
            Token::Id(cmd) => {
                i = build_command_node(cmd.to_string(), tokens, i, &mut ast)?;
            }

            Token::Semicolon => {
                i += 1;
            }

            Token::LeftParen => {
                i = build_subshell_node(tokens, i, &mut ast)?;
            }
            Token::RightParen => {
                // error because LeftParen should match with ')'
                // which moves the index to the token after its matching
                // RightParen, meaning if a ')' is encountered, it's unmatched
                return Err("parse error near ')'".to_string());
            }

            _ => {
                println!("TODO: build_ast");
                break;
            }
        }
    }

    Ok(ast)
}

fn build_command_node(
    cmd: String,
    tokens: &[Token],
    mut i: usize,
    ast: &mut Vec<Node>,
) -> Result<usize, String> {
    let len = tokens.len();
    let mut args = Vec::new();

    i += 1;

    while i < len {
        match &tokens[i] {
            Token::Id(arg) => args.push(arg.to_string()),

            Token::Dollar => {
                i += 1;
                if i >= len {
                    args.push("$".to_string());
                    break;
                }

                match &tokens[i] {
                    Token::Id(key) => match env::var(key) {
                        Ok(val) => args.push(val),
                        Err(e) => return Err(e.to_string()),
                    },
                    _ => return Err("invalid argument to $".to_string()),
                }
            }

            _ => break,
        }

        i += 1;
    }

    ast.push(Node::Command {
        program: cmd.to_string(),
        args,
    });

    Ok(i)
}

fn build_subshell_node(
    tokens: &[Token],
    mut i: usize,
    ast: &mut Vec<Node>,
) -> Result<usize, String> {
    let mut command = String::new();
    let mut matching_parens = Vec::new();
    let len = tokens.len();

    matching_parens.push(Token::LeftParen);
    i += 1;

    while !matching_parens.is_empty() && i < len {
        match &tokens[i] {
            Token::Id(arg) => {
                command.push_str(&arg);
                command.push(' ');
            }
            Token::Semicolon => command.push(';'),
            Token::Dollar => command.push('$'),
            Token::LeftParen => {
                command.push('(');
                matching_parens.push(Token::LeftParen);
            }
            Token::RightParen => {
                command.push(')');
                matching_parens.pop();
            }
            Token::LeftCurlyBracket => command.push('{'),
            Token::RightCurlyBracket => command.push('}'),
            Token::Pipe => command.push('|'),
            Token::RedirectInput => command.push('<'),
            Token::RedirectOutput => command.push('>'),
            Token::RedirectOutputAppend => {
                command.push('>');
                command.push('>');
            }
            Token::Background => command.push('&'),
            Token::And => {
                command.push('&');
                command.push('&');
            }
            Token::Or => {
                command.push('|');
                command.push('|');
            }
        }

        i += 1;
    }

    if i >= len && !matching_parens.is_empty() {
        return Err("unmatched '('".to_string());
    }

    // remove the trailing ')'
    command.pop();

    ast.push(Node::Subshell { command: command });
    Ok(i)
}

fn expr(ast: &[Node]) -> Result<ExitStatus, String> {
    let mut i = 0;
    let len = ast.len();
    let mut exit_code = ExitStatus::default();

    while i < len {
        match &ast[i] {
            Node::Command { program, args } => {
                let proc = Process {
                    cmd: program.to_string(),
                    args: args.to_vec(),
                };
                exit_code = run_cmd(&proc)?;
            }

            Node::Subshell { command } => {
                let mut args = Vec::new();
                args.push("-c");
                args.push(command);

                let proc = Process {
                    cmd: "./target/debug/rshell".to_string(),
                    args: ["-c".to_string(), (&command).to_string()].to_vec(),
                };
                exit_code = run_cmd(&proc)?;
            }

            _ => println!("TODO: expr"),
        }

        i += 1;
    }

    Ok(exit_code)
}
