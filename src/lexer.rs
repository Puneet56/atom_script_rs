use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Illegal,
    Eof,
    Ident(String),
    Int(i32),
    String(String),
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Lt,
    Gt,
    Eq,
    NotEq,
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Colon,
    Atom,
    Molecule,
    Reaction,
    True,
    False,
    If,
    Else,
    Produce,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Illegal => write!(f, "Illegal"),
            Token::Eof => write!(f, "EOF"),

            Token::Ident(s) => write!(f, "Int({})", s),
            Token::Int(i) => write!(f, "Int({})", i),
            Token::String(s) => write!(f, "String({})", s),

            Token::Assign => write!(f, "Assign"),
            Token::Plus => write!(f, "Plus"),
            Token::Minus => write!(f, "Minus"),
            Token::Bang => write!(f, "Bang"),
            Token::Asterisk => write!(f, "Asterisk"),
            Token::Slash => write!(f, "Slash"),
            Token::Lt => write!(f, "Lt"),
            Token::Gt => write!(f, "Gt"),
            Token::Eq => write!(f, "Eq"),
            Token::NotEq => write!(f, "NotEq"),
            Token::Comma => write!(f, "Comma"),
            Token::Semicolon => write!(f, "Semicolon"),
            Token::LParen => write!(f, "LParen"),
            Token::RParen => write!(f, "RParen"),
            Token::LBrace => write!(f, "LBrace"),
            Token::RBrace => write!(f, "RBrace"),
            Token::Colon => write!(f, "Colon"),
            Token::Atom => write!(f, "Atom"),
            Token::Molecule => write!(f, "Molecule"),
            Token::Reaction => write!(f, "Reaction"),
            Token::True => write!(f, "True"),
            Token::False => write!(f, "False"),
            Token::If => write!(f, "If"),
            Token::Else => write!(f, "Else"),
            Token::Produce => write!(f, "Produce"),
        }
    }
}

pub struct Lexer {
    pub input: String,
    pub position: usize,
    pub read_position: usize,
    pub ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        l.read_char();
        l
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        let token = match self.ch {
            '\0' => Token::Eof,
            '=' => Token::Assign,
            ';' => Token::Semicolon,
            '(' => Token::LParen,
            ')' => Token::RParen,
            ',' => Token::Comma,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '/' => Token::Slash,
            '*' => Token::Asterisk,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            _ => Token::Illegal,
        };

        self.read_char();
        token
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = String::from("=+(){},;");
        let mut lexer = Lexer::new(input);

        let tests = vec![
            Token::Assign,
            Token::Plus,
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::RBrace,
            Token::Comma,
            Token::Semicolon,
            Token::Eof,
        ];

        for t in tests {
            let tok = lexer.next_token();
            assert_eq!(tok, t, "Expected {}, got {}", t, tok);
        }
    }
}
