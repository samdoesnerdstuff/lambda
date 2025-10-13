//! Lambda Lexer Module
//!

#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum Token {
    // Type literals
    Integer(i64),
    Float(f64),
    Bool(bool),
    String(String),

    // Identifiers
    Identifier(String),

    // Keywords
    Fn,
    End,
    If,
    Else,
    While,
    For,
    Return,
    True,
    False,
    Null,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    DoubleEqual,
    NotEqual,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    Arrow,   // ->

    // Delimiters
    LParen,
    RParen,
    LBracket,
    RBracket,
    Comma,
    Colon,
    Semicolon,

    // Other
    Comment(String),
    EOF,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

pub mod lexer {
    use super::{Token, Span};

    pub fn lex(source: &str) -> Vec<(Token, Span)> {
        let mut tokens = Vec::new();
        let mut chars = source.char_indices().peekable();

        while let Some((i, c)) = chars.next() {
            match c {
                // Whitespace
                //
                c if c.is_whitespace() => continue,

                // Single character tokens
                //
                '(' => tokens.push((Token::LParen, Span { start: i, end: i + 1 })),
                ')' => tokens.push((Token::RParen, Span { start: i, end: i + 1 })),
                ',' => tokens.push((Token::Comma, Span { start: i, end: i + 1 })),
                ':' => tokens.push((Token::Colon, Span { start: i, end: i + 1 })),
                ';' => tokens.push((Token::Semicolon, Span { start: i, end: i + 1 })),

                // Operators
                //
                '+' => tokens.push((Token::Plus, Span { start: i, end: i + 1 })),
                '-' => {
                    if let Some((_, '>')) = chars.peek() {
                        chars.next();
                        tokens.push((Token::Arrow, Span { start: i, end: i + 2 }));
                    } else {
                        tokens.push((Token::Minus, Span { start: i, end: i + 1 }));
                    }
                }
                '*' => tokens.push((Token::Star, Span { start: i, end: i + 1 })),

                // This could be both division or a comment so it gets special treatment
                //
                '/' => {
                    if let Some((_, '!')) = chars.peek() {
                        chars.next(); // consume '!'
                        let start = i;
                        let mut end = i + 2; // account for /!
                        let mut content = String::new();
                        while let Some((j, ch)) = chars.next() {
                            if ch == '\n' {
                                end = j;
                                break;
                            } else {
                                content.push(ch);
                                end = j;
                            }
                        }
                        tokens.push((Token::Comment(content), Span { start, end }));
                    } else {
                        tokens.push((Token::Slash, Span { start: i, end: i + 1 }));
                    }
                }

                // Numbers
                //
                c if c.is_ascii_digit() => {
                    let start = i;
                    let mut end = i;
                    let mut num_str = c.to_string();
                    let mut is_float = false;

                    while let Some(&(_, next)) = chars.peek() {
                        if next.is_ascii_digit() {
                            let (j, d) = chars.next().unwrap();
                            num_str.push(d);
                            end = j;
                        } else if next == '.' && !is_float {
                            is_float = true;
                            let (j, dot) = chars.next().unwrap();
                            num_str.push(dot);
                            end = j;
                        } else {
                            break;
                        }
                    }

                    let token = if is_float {
                        Token::Float(num_str.parse::<f64>().unwrap())
                    } else {
                        Token::Integer(num_str.parse::<i64>().unwrap())
                    };

                    tokens.push((token, Span { start, end: end + 1 }));
                }

                // Strings
                //
                '"' => {
                    let start = i;
                    let mut end = i;
                    let mut content = String::new();

                    while let Some((j, ch)) = chars.next() {
                        if ch == '"' {
                            end = j;
                            break;
                        } else {
                            content.push(ch);
                        }
                    }

                    tokens.push((Token::String(content), Span { start, end: end + 1 }));
                }

                // Identifiers & keywords
                //
                c if c.is_ascii_alphabetic() || c == '_' => {
                    let start = i;
                    let mut end = i;
                    let mut ident = c.to_string();

                    while let Some(&(_, ch)) = chars.peek() {
                        if ch.is_ascii_alphanumeric() || ch == '_' {
                            let (j, c) = chars.next().unwrap();
                            ident.push(c);
                            end = j;
                        } else {
                            break;
                        }
                    }

                    let token = match ident.as_str() {
                        "fn" => Token::Fn,
                        "end" => Token::End,
                        "if" => Token::If,
                        "else" => Token::Else,
                        "while" => Token::While,
                        "for" => Token::For,
                        "return" => Token::Return,
                        "true" => Token::Bool(true),
                        "false" => Token::Bool(false),
                        "null" => Token::Null,
                        _ => Token::Identifier(ident),
                    };

                    tokens.push((token, Span { start, end: end + 1 }));
                }

                // Unknown chars
                //
                _ => {
                    eprintln!("Unexpected character '{}' @ {}", c, i)
                }
            }
        }

        tokens.push((Token::EOF, Span { start: source.len(), end: source.len() }));
        tokens
    }
}

// --------------------------- !! LEXER TESTS !! --------------------------- //
#[cfg(test)]
mod tests {
    use super::lexer::*;

    fn lex_to_strings(src: &str) -> Vec<String> {
        lex(src)
            .into_iter()
            .map(|(tok, _)| format!("{tok:?}"))
            .collect()
    }

    #[test]
    fn test_simple_fn() {
        let src = "fn hello() end";
        let expected = vec![
            "Fn", 
            "Identifier(\"hello\")", 
            "LParen", 
            "RParen", 
            "End", 
            "EOF"
        ];
        assert_eq!(lex_to_strings(src), expected);
    }

    #[test]
    fn test_numbers() {
        let src = "42 3.14";
        let expected = vec![
            "Integer(42)", 
            "Float(3.14)", 
            "EOF"
        ];
        assert_eq!(lex_to_strings(src), expected);
    }

    #[test]
    fn test_strings() {
        let src = "\"hello\" \"world\"";
        let expected = vec![
            "String(\"hello\")", 
            "String(\"world\")", 
            "EOF"
        ];
        assert_eq!(lex_to_strings(src), expected);
    }

    #[test]
    fn test_operators() {
        let src = "+ - * / -> ==";
        let expected = vec![
            "Plus", "Minus", "Star", "Slash", "Arrow", "EOF"
        ];
        assert_eq!(lex_to_strings(src), expected);
    }

    #[test]
    fn test_keywords_and_identifiers() {
        let src = "if else while for return true false null my_var";
        let expected = vec![
            "If", "Else", "While", "For", "Return", "Bool(true)", "Bool(false)", "Null", "Identifier(\"my_var\")", "EOF"
        ];
        assert_eq!(lex_to_strings(src), expected);
    }

    #[test]
    fn test_complex_fn_call() {
        let src = "fn greet(name) write(\"hi \" + name) end";
        let expected = vec![
            "Fn", "Identifier(\"greet\")", "LParen", "Identifier(\"name\")", "RParen",
            "Identifier(\"write\")", "LParen", "String(\"hi \")", "Plus", "Identifier(\"name\")", "RParen",
            "End", "EOF"
        ];
        assert_eq!(lex_to_strings(src), expected);
    }

    #[test]
    fn test_comments() {
        let src = "/! this is a comment in Lambda";
        let expected = vec!["Comment(\" this is a comment in Lambda\")", "EOF" ];
        assert_eq!(lex_to_strings(src), expected);
    }
}
