use crate::lexer::Token;

#[derive(Debug, Clone)]
enum Expr {
    Num(f64),
    Command(String, Box<Expr>),
    Pipe(i32, i32),            /* new stdin, new stdout */
    RedirectInput(i32),        /* new stdin */
    RedirectOutput(i32),       /* new stdout */
    RedirectOutputAppend(i32), /* new stdout */
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
}

fn parse(tokens: &Vec<Token>) -> Result<Expr, String> {
    let expr = expr()?;
    Ok(expr)
}

fn expr() -> Result<Expr, String> {
    print!("TODO");
}

fn term() -> Result<Expr, String> {
    print!("TODO");
}
