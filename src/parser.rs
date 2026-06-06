use crate::lexer::Token;

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

fn parse(tokens: &Vec<Token>) -> Result<Node, String> {
    let expr = expr(tokens)?;

    Ok(expr)
}

fn expr(tokens: &Vec<Token>) -> Result<Node, String> {}
