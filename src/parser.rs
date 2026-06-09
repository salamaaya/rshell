use crate::{
    lexer::Token,
    process::{Process, run_cmd},
};

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

pub fn parse(tokens: &Vec<Token>) -> Result<i32, String> {
    let ast = build_ast(tokens)?;
    let expr = expr(&ast)?;
    Ok(expr)
}

fn build_ast(tokens: &Vec<Token>) -> Result<Vec<Node>, String> {
    let mut i = 0;
    let len = tokens.len();
    let mut ast = Vec::new();

    while i < len {
        let curr_tok = &tokens[i];
        match curr_tok {
            Token::Id(cmd) => {
                i = build_command_node(cmd.to_string(), tokens, i, &mut ast);
            }

            _ => print!("TODO"),
        }

        i += 1;
    }

    Ok(ast)
}

fn build_command_node(
    cmd: String,
    tokens: &Vec<Token>,
    mut i: usize,
    ast: &mut Vec<Node>,
) -> usize {
    let len = tokens.len();
    let mut args = vec![];

    i += 1;

    while i < len {
        let curr_arg = &tokens[i];

        match curr_arg {
            Token::Id(arg) => args.push(arg.to_string()),

            Token::Semicolon => {
                i += 1;
                break;
            }
            Token::Dollar => {
                println!("TODO: $");
                break;
            }

            Token::Escape(c) => {
                println!("TODO: \\{c}");
                break;
            }
            Token::Backslash => {
                println!("TODO: \\");
                break;
            }
            Token::Slash => {
                println!("TODO: /");
                break;
            }

            Token::LeftParen => {
                println!("TODO: (");
                break;
            }
            Token::RightParen => {
                println!("TODO: )");
                break;
            }
            Token::LeftCurlyBracket => {
                println!("TODO: {{");
                break;
            }
            Token::RightCurlyBracket => {
                println!("TODO: }}");
                break;
            }

            Token::Pipe => {
                println!("TODO: |");
                break;
            }
            Token::RedirectInput => {
                println!("TODO: <");
                break;
            }
            Token::RedirectOutput => {
                println!("TODO: >");
                break;
            }
            Token::RedirectOutputAppend => {
                println!("TODO: >>");
                break;
            }
            Token::Background => {
                println!("TODO: &");
                break;
            }

            Token::And => {
                println!("TODO: &&");
                break;
            }
            Token::Or => {
                println!("TODO: ||");
                break;
            }
        }
        i += 1;
    }

    ast.push(Node::Command {
        program: cmd.to_string(),
        args: args,
    });

    return i;
}

fn expr(ast: &Vec<Node>) -> Result<i32, String> {
    let mut i = 0;
    let len = ast.len();
    let mut exit_code = 0;

    while i < len {
        let curr_node = &ast[i];
        match curr_node {
            Node::Command { program, args } => {
                let proc = Process {
                    cmd: program.to_string(),
                    args: args.to_vec(),
                };
                exit_code = run_cmd(&proc);
            }

            _ => print!("TODO"),
        }

        i += 1;
    }

    Ok(exit_code)
}
