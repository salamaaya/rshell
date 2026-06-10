// use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Token {
    Id(String),

    Semicolon,
    Dollar,

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

pub fn lex(input: &str) -> Result<Vec<Token>, String> {
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

            'A'..='Z' | 'a'..='z' | '-' | '.' | '\\' | '/' => {
                let mut s = String::new();
                let mut ch = it.peek();
                while let Some(&i) = ch {
                    if !i.is_ascii_digit()
                        && !i.is_alphabetic()
                        && (i != '-')
                        && (i != '.')
                        && (i != '\\')
                        && (i != '/')
                    {
                        break;
                    } else if i == '\\' {
                        it.next();
                        ch = it.peek();
                        match ch {
                            Some(&c) => {
                                if is_special(c) {
                                    s.push(c);
                                    it.next();
                                    ch = it.peek();
                                }
                            }
                            _ => break,
                        }
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
                let mut s = String::new();

                it.next();
                let mut ch = it.peek();
                while let Some(&i) = ch {
                    match i {
                        '\'' => {
                            it.next();
                            break;
                        }
                        '\n' => return Err("unterminated single quote".to_string()),
                        _ => {
                            s.push(i);
                            it.next();
                            ch = it.peek();
                        }
                    }
                }
                result.push(Token::Id(s.clone()));
            }

            '\"' => {
                let mut s = String::new();

                it.next();
                let mut ch = it.peek();
                while let Some(&i) = ch {
                    match i {
                        '\"' => {
                            it.next();
                            break;
                        }
                        '\n' => return Err("unterminated double quote".to_string()),
                        _ => {
                            s.push(i);
                            it.next();
                            ch = it.peek();
                        }
                    }
                }
                result.push(Token::Id(s.clone()));
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

    Ok(result)
}

fn is_special(c: char) -> bool {
    match c {
        '\"' | '\'' | '\\' | '#' | '=' | '&' | '|' | '>' | '<' | ';' | '{' | '}' | '(' | ')'
        | '!' | '$' => true,
        _ => false,
    }
}
