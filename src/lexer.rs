// use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Token {
    Id(String),

    SingleQuote,
    DoubleQuote,
    Semicolon,
    Dollar,

    Escape(String),
    Backslash,
    Slash,

    LeftParen,
    RightParen,
    LeftCurlyBracket,
    RightCurlyBracket,

    Pipe,
    RedirectInput,
    RedirectOutput,
    RedirectOutputAppend,
    Background,

    And,
    Or,
}

pub fn lex(input: &String) -> Result<Vec<Token>, String> {
    let mut result = Vec::new();
    let mut it = input.chars().peekable();
    let mut _lineno = 1;
    /* let mut words = HashMap::from([
        ("true".to_string(), Token::True("true".to_string())),
        ("false".to_string(), Token::False("false".to_string())),
        ("if".to_string(), Token::If("if".to_string())),
        ("else".to_string(), Token::Else("else".to_string())),
        ("while".to_string(), Token::While("while".to_string())),
    ]);
    TODO implement branching */

    while let Some(&c) = it.peek() {
        match c {
            ' ' | '\t' => {
                it.next();
            }
            '\n' => {
                _lineno += 1;
                it.next();
            }

            'A'..='Z' | 'a'..='z' | '-' | '.' => {
                let mut s = String::new();
                s.push(c);

                it.next();
                let mut ch = it.peek();
                while let Some(&i) = ch {
                    if !i.is_digit(10) && !i.is_alphabetic() && !(i == '.') {
                        ch = None;
                    } else {
                        s.push(i);
                        it.next();
                        ch = it.peek();
                    }
                }
                result.push(Token::Id(s.clone()));
                /* match words.get(&s) {
                    Some(t) => result.push(Token::clone(t)),
                    None => {
                        result.push(Token::Id(s.clone()));
                        words.insert(s.clone(), Token::Id(s.clone()));
                    }
                } will need for the TODO for branching */
            }

            '\'' => {
                result.push(Token::SingleQuote);
                it.next();
            }
            '\"' => {
                result.push(Token::DoubleQuote);
                it.next();
            }
            ';' => {
                result.push(Token::Semicolon);
                it.next();
            }
            '$' => {
                result.push(Token::Dollar);
                it.next();
            }

            '(' => {
                result.push(Token::LeftParen);
                it.next();
            }
            ')' => {
                result.push(Token::RightParen);
                it.next();
            }
            '{' => {
                result.push(Token::LeftCurlyBracket);
                it.next();
            }
            '}' => {
                result.push(Token::RightCurlyBracket);
                it.next();
            }

            '\\' => {
                it.next();
                let ch = it.peek();
                if let Some(escape_char) = ch {
                    result.push(Token::Escape(escape_char.to_string()));
                    it.next();
                } else {
                    result.push(Token::Backslash);
                }
            }
            '/' => {
                result.push(Token::Slash);
                it.next();
            }

            '|' => {
                it.next();
                let ch = it.peek();
                if let Some('|') = ch {
                    result.push(Token::Or);
                    it.next();
                } else {
                    result.push(Token::Pipe);
                }
            }
            '<' => {
                result.push(Token::RedirectInput);
                it.next();
            }
            '>' => {
                it.next();
                let ch = it.peek();
                if let Some('>') = ch {
                    result.push(Token::RedirectOutputAppend);
                    it.next();
                } else {
                    result.push(Token::RedirectOutput);
                }
            }
            '&' => {
                it.next();
                let ch = it.peek();
                if let Some('&') = ch {
                    result.push(Token::And);
                    it.next();
                } else {
                    result.push(Token::Background);
                }
            }

            _ => {
                result.push(Token::Id(c.to_string()));
                it.next();
            }
        }
    }

    return Ok(result);
}
