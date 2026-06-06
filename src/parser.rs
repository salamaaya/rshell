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
    let mut ast = vec![];

    while i < len {
        let curr_tok = &tokens[i];
        match curr_tok {
            Token::Id(cmd) => {
                let mut args = vec![];
                i += 1;
                while i < len {
                    let curr_arg = &tokens[i];
                    match curr_arg {
                        Token::Id(arg) => args.push(arg.to_string()),
                        _ => break,
                    }
                    i += 1;
                }
                ast.push(Node::Command {
                    program: cmd.to_string(),
                    args: args,
                })
            }

            _ => print!("TODO"),
        }

        i += 1;
    }

    Ok(ast)
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
