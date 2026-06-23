use clap::error::Result;

use crate::{
    lexer::Token,
    process::{Process, run_cmd, run_cmd_pipe, run_cmd_redirect_input},
};

use std::env;
use std::process::ExitStatus;

#[derive(Debug, Clone)]
pub enum Operator {
    Pipe,                 // |
    RedirectInput,        // <
    RedirectOutput,       // >
    RedirectOutputAppend, // >>
    And,                  // &&
    Or,                   // ||
}

#[derive(Debug, Clone)]
pub enum Node {
    Command {
        cmd: String,
        args: Vec<String>,
    },

    Subshell {
        cmd: String,
    },

    InlineGroup {
        cmds: Vec<Node>,
    },

    Redirect {
        op: Operator,
        cmds: Vec<Node>,
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

pub fn build_ast(tokens: &[Token]) -> Result<Vec<Node>, String> {
    let mut i = 0;
    let len = tokens.len();
    let mut ast: Vec<Node> = Vec::new();

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

            Token::LeftCurlyBracket => {
                i = build_inline_node(tokens, i, &mut ast)?;
            }
            Token::RightCurlyBracket => {
                // error because LeftCurlyBracket should match with '}'
                // which moves the index to the token after its matching
                // RightCurlyBracket, meaning if a '}' is encountered, it's unmatched
                return Err("parse error near '}'".to_string());
            }

            Token::Pipe => {
                i = build_pipe_node(tokens, i, &mut ast)?;
            }

            Token::RedirectInput => {
                // not for when i come back to this:
                // this doesn't work with pipes, for example:
                // cat < file.txt | wc -l
                // or {cat < file.txt} | wc -l
                // (but does work with subshells)
                // consider refactoring?? but get it fixing somehow!
                i = build_redirect_input_node(tokens, i, &mut ast)?;
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
        cmd: cmd.to_string(),
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
                command.push_str(arg);
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

    ast.push(Node::Subshell { cmd: command });
    Ok(i)
}

fn build_inline_node(tokens: &[Token], i: usize, ast: &mut Vec<Node>) -> Result<usize, String> {
    build_inline_node_recurse(tokens, i, ast, &mut Vec::new())
}

fn build_inline_node_recurse(
    tokens: &[Token],
    mut i: usize,
    ast: &mut Vec<Node>,
    inline_nodes: &mut Vec<Node>,
) -> Result<usize, String> {
    let mut matching_curly_brackets = false;
    let len = tokens.len();

    i += 1;

    while !matching_curly_brackets && i < len {
        match &tokens[i] {
            Token::Id(cmd) => {
                i = build_command_node(cmd.to_string(), tokens, i, ast)?;
                let command = pop_command(ast)?;
                inline_nodes.push(command);
            }

            Token::Semicolon => {
                i += 1;
            }

            Token::LeftParen => {
                i = build_subshell_node(tokens, i, ast)?;
                let subshell = pop_subshell(ast)?;
                inline_nodes.push(subshell);
            }
            Token::RightParen => {
                return Err("parse error near ')'".to_string());
            }
            Token::LeftCurlyBracket => {
                i = build_inline_node_recurse(tokens, i, ast, &mut Vec::new())?;
                let inline_group = pop_inline(ast)?;
                inline_nodes.push(inline_group);
            }
            Token::RightCurlyBracket => {
                matching_curly_brackets = true;
                i += 1;
                break;
            }

            Token::Pipe => {
                ast.push(inline_nodes.pop().unwrap());
                i = build_pipe_node(tokens, i, ast)?;
                let pipe = pop_pipe(ast)?;
                inline_nodes.push(pipe);
            }

            Token::RedirectInput => {
                ast.push(inline_nodes.pop().unwrap());
                i = build_redirect_input_node(tokens, i, ast)?;
                let redirect = pop_redirect(ast)?;
                inline_nodes.push(redirect);
            }

            _ => {
                print!("TODO: build_inline_node");
                break;
            }
        }
    }

    if !matching_curly_brackets {
        return Err("unmatched '{'".to_string());
    }

    ast.push(Node::InlineGroup {
        cmds: inline_nodes.to_vec(),
    });
    Ok(i)
}

fn build_pipe_node(tokens: &[Token], mut i: usize, ast: &mut Vec<Node>) -> Result<usize, String> {
    let mut commands = Vec::new();
    let len = tokens.len();
    let mut num_pipes = 0;

    match ast.pop() {
        Some(command) => commands.push(command),
        _ => return Err("parse error, invalid pipe".to_string()),
    }

    while i < len {
        match &tokens[i] {
            Token::Id(cmd) => {
                i = build_command_node(cmd.to_string(), tokens, i, ast)?;

                let command = pop_command(ast)?;
                commands.push(command);
            }

            Token::LeftParen => {
                i = build_subshell_node(tokens, i, ast)?;

                let subshell = pop_subshell(ast)?;
                commands.push(subshell);
            }
            Token::RightParen => {
                return Err("parse error near ')'".to_string());
            }

            Token::LeftCurlyBracket => {
                i = build_inline_node(tokens, i, ast)?;

                let inline = pop_inline(ast)?;
                commands.push(inline);
            }
            Token::RightCurlyBracket => {
                // break instead of error because pipe could be inside of inline group
                break;
            }

            Token::Pipe => {
                i += 1;
                num_pipes += 1;
            }

            Token::RedirectInput => {
                i = build_redirect_input_node(tokens, i, ast)?;
                let redirect = pop_redirect(ast)?;
                commands.push(redirect);
            }

            _ => {
                println!("TODO: build_pipe_node");
                break;
            }
        }
    }

    if num_pipes != commands.len() - 1 {
        return Err("parse error, invalid pipe".to_string());
    }

    ast.push(Node::Redirect {
        op: Operator::Pipe,
        cmds: commands.to_vec(),
        file: String::new(),
    });

    Ok(i)
}

fn build_redirect_input_node(
    tokens: &[Token],
    mut i: usize,
    ast: &mut Vec<Node>,
) -> Result<usize, String> {
    let len = tokens.len();
    i += 1;
    if i > len {
        return Err("parse error near '<'".to_string());
    }

    let command = match ast.pop() {
        Some(cmd) => cmd,
        _ => return Err("parse error near '<'".to_string()),
    };

    match &tokens[i] {
        Token::Id(file) => {
            ast.push(Node::Redirect {
                op: Operator::RedirectInput,
                cmds: [command].to_vec(),
                file: file.to_string(),
            });

            i += 1;
        }

        _ => {
            return Err("parse error near '<'".to_string());
        }
    }

    Ok(i)
}

fn expr(ast: &[Node]) -> Result<ExitStatus, String> {
    let mut i = 0;
    let len = ast.len();
    let mut exit_code = ExitStatus::default();

    while i < len {
        match &ast[i] {
            Node::Command { cmd, args } => {
                let proc = Process {
                    cmd: cmd.to_string(),
                    args: args.to_vec(),
                };
                exit_code = run_cmd(&proc)?;
            }

            Node::Subshell { cmd } => {
                let proc = Process {
                    cmd: "./target/debug/rshell".to_string(),
                    args: ["-c".to_string(), (&cmd).to_string()].to_vec(),
                };
                exit_code = run_cmd(&proc)?;
            }

            Node::InlineGroup { cmds } => {
                expr(cmds)?;
            }

            Node::Redirect {
                op: Operator::Pipe,
                cmds,
                file: _file,
            } => {
                let mut procs = Vec::new();
                for c in cmds {
                    match c {
                        Node::Command { cmd: _, args: _ } | Node::Subshell { cmd: _ } => {
                            procs.push(command_to_proccess(c.clone())?);
                        }
                        Node::InlineGroup { cmds } => {
                            for cmd in cmds {
                                procs.push(command_to_proccess(cmd.clone())?);
                            }
                        }
                        Node::Redirect {
                            op: Operator::RedirectInput,
                            cmds,
                            file,
                        } => {}
                        _ => {
                            return Err("invalid pipe".to_string());
                        }
                    };
                }
                exit_code = run_cmd_pipe(&procs)?;
            }

            Node::Redirect {
                op: Operator::RedirectInput,
                cmds,
                file,
            } => {
                match &cmds[0] {
                    Node::Command { cmd: _, args: _ } | Node::Subshell { cmd: _ } => {
                        let proc = command_to_proccess(cmds[0].clone())?;
                        exit_code = run_cmd_redirect_input(&proc, file)?;
                    }
                    Node::InlineGroup { cmds } => {
                        for cmd in cmds {
                            let proc = command_to_proccess(cmd.clone())?;
                            exit_code = run_cmd_redirect_input(&proc, file)?;
                        }
                    }
                    _ => {
                        return Err("parse error near '<'".to_string());
                    }
                };
            }

            _ => println!("TODO: expr"),
        }

        i += 1;
    }

    Ok(exit_code)
}

fn pop_command(ast: &mut Vec<Node>) -> Result<Node, String> {
    match ast.pop() {
        Some(Node::Command { cmd, args }) => Ok(Node::Command { cmd, args }),
        _ => Err("parse error, invalid command".to_string()),
    }
}

fn pop_subshell(ast: &mut Vec<Node>) -> Result<Node, String> {
    match ast.pop() {
        Some(Node::Subshell { cmd }) => Ok(Node::Subshell { cmd }),
        _ => Err("parse error, invalid subshell".to_string()),
    }
}

fn pop_inline(ast: &mut Vec<Node>) -> Result<Node, String> {
    match ast.pop() {
        Some(Node::InlineGroup { cmds }) => Ok(Node::InlineGroup { cmds }),
        _ => Err("parse error, invalid inline group".to_string()),
    }
}

fn pop_pipe(ast: &mut Vec<Node>) -> Result<Node, String> {
    match ast.pop() {
        Some(Node::Redirect {
            op: Operator::Pipe,
            cmds,
            file,
        }) => Ok(Node::Redirect {
            op: Operator::Pipe,
            cmds,
            file,
        }),
        _ => Err("parse error, invalid pipe".to_string()),
    }
}

fn pop_redirect(ast: &mut Vec<Node>) -> Result<Node, String> {
    match ast.pop() {
        Some(Node::Redirect {
            op: Operator::RedirectInput,
            cmds,
            file,
        }) => Ok(Node::Redirect {
            op: Operator::RedirectInput,
            cmds,
            file,
        }),
        Some(Node::Redirect {
            op: Operator::RedirectOutput,
            cmds,
            file,
        }) => Ok(Node::Redirect {
            op: Operator::RedirectOutput,
            cmds,
            file,
        }),
        _ => Err("parse error, invalid pipe".to_string()),
    }
}

fn command_to_proccess(command: Node) -> Result<Process, String> {
    match command {
        Node::Command { cmd, args } => {
            return Ok(Process {
                cmd: cmd.to_string(),
                args: args.to_vec(),
            });
        }
        Node::Subshell { cmd } => {
            return Ok(Process {
                cmd: "./target/debug/rshell".to_string(),
                args: ["-c".to_string(), (&cmd).to_string()].to_vec(),
            });
        }
        _ => {
            return Err("cannot convert command to process".to_string());
        }
    };
}
