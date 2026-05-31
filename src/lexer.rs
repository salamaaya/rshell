// use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Token {
    Num(f64),
    Id(String),

    SingleQuote(String),
    DoubleQuote(String),
    Semicolon(String),
    Dollar(String),
    LeftParen(String),
    RightParen(String),
    LeftCurlyBracket(String),
    RightCurlyBracket(String),

    Pipe(String),
    RedirectInput(String),
    RedirectOutput(String),
    RedirectOutputAppend(String),
    Background(String),
    And(String),
    Or(String),
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

            '0'..='9' => {
                let mut num = c
                    .to_string()
                    .parse()
                    .expect("lexer: character not a digit.");

                it.next();
                let mut digit_char = it.peek();

                while let Some(&i) = digit_char {
                    if !i.is_digit(10) {
                        if i == '.' {
                            let mut d = 10.0;
                            it.next();
                            digit_char = it.peek();

                            while let Some(&j) = digit_char {
                                if !j.is_digit(10) {
                                    digit_char = None;
                                } else {
                                    let f: f64 = j
                                        .to_string()
                                        .parse()
                                        .expect("lexer: character not a digit.");
                                    num = num + f / d;
                                    d = d * 10.0;
                                    it.next();
                                    digit_char = it.peek();
                                }
                            }
                        } else {
                            digit_char = None;
                        }
                    } else {
                        let digit: f64 = i
                            .to_string()
                            .parse()
                            .expect("lexer: character not a digit.");
                        num = num * 10.0 + digit;
                        it.next();
                        digit_char = it.peek();
                    }
                }
                result.push(Token::Num(num));
            }
            'A'..='Z' | 'a'..='z' => {
                let mut s = String::new();
                s.push(c);

                it.next();
                let mut ch = it.peek();
                while let Some(&i) = ch {
                    if !i.is_digit(10) && !i.is_alphabetic() {
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
                result.push(Token::SingleQuote("\'".to_string()));
                it.next();
            }
            '\"' => {
                result.push(Token::DoubleQuote("\"".to_string()));
                it.next();
            }
            ';' => {
                result.push(Token::Semicolon(";".to_string()));
                it.next();
            }
            '$' => {
                result.push(Token::Semicolon("$".to_string()));
                it.next();
            }
            '(' => {
                result.push(Token::LeftParen("(".to_string()));
                it.next();
            }
            ')' => {
                result.push(Token::RightParen(")".to_string()));
                it.next();
            }
            '{' => {
                result.push(Token::LeftCurlyBracket("{".to_string()));
                it.next();
            }
            '}' => {
                result.push(Token::RightCurlyBracket("}".to_string()));
                it.next();
            }

            '|' => {
                it.next();
                let ch = it.peek();
                if let Some('|') = ch {
                    result.push(Token::Or("||".to_string()));
                    it.next();
                } else {
                    result.push(Token::Pipe("|".to_string()));
                }
            }
            '<' => {
                result.push(Token::RedirectInput("<".to_string()));
                it.next();
            }
            '>' => {
                it.next();
                let ch = it.peek();
                if let Some('>') = ch {
                    result.push(Token::RedirectOutputAppend(">>".to_string()));
                    it.next();
                } else {
                    result.push(Token::RedirectOutput(">".to_string()));
                }
            }
            '&' => {
                it.next();
                let ch = it.peek();
                if let Some('&') = ch {
                    result.push(Token::And("&&".to_string()));
                    it.next();
                } else {
                    result.push(Token::Background("&".to_string()));
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
