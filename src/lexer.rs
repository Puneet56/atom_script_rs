use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Illegal,
    Eof,
    Ident(String),
    Int(isize),
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
    LBracket,
    RBracket,
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
            Token::Ident(s) => write!(f, "Int({})", s),
            Token::Int(i) => write!(f, "Int({})", i),
            Token::String(s) => write!(f, "String({})", s),

            _ => write!(f, "{:?}", self),
        }
    }
}

impl From<char> for Token {
    fn from(c: char) -> Self {
        match c {
            '\0' => Self::Eof,
            '+' => Self::Plus,
            '-' => Self::Minus,
            '!' => Self::Bang,
            '*' => Self::Asterisk,
            '/' => Self::Slash,
            '<' => Self::Lt,
            '>' => Self::Gt,
            '=' => Self::Assign,
            ',' => Self::Comma,
            ';' => Self::Semicolon,
            '(' => Self::LParen,
            ')' => Self::RParen,
            '{' => Self::LBrace,
            '}' => Self::RBrace,
            ':' => Self::Colon,
            '[' => Self::LBracket,
            ']' => Self::RBracket,
            _ => Self::Illegal,
        }
    }
}

impl From<String> for Token {
    fn from(s: String) -> Self {
        match s.as_str() {
            "!=" => Self::NotEq,
            "==" => Self::Eq,
            "atom" => Self::Atom,
            "molecule" => Self::Molecule,
            "reaction" => Self::Reaction,
            "true" => Self::True,
            "false" => Self::False,
            "if" => Self::If,
            "else" => Self::Else,
            "produce" => Self::Produce,
            _ => {
                if s.chars().all(|c| c.is_numeric()) {
                    Self::Int(s.parse::<isize>().unwrap())
                } else {
                    Self::Ident(s)
                }
            }
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
    pub fn new(input: String) -> Self {
        let mut l = Self {
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

    pub fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input.chars().nth(self.read_position).unwrap()
        }
    }

    pub fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    pub fn read_identifer(&mut self) -> String {
        let mut ident = String::new();
        while self.ch.is_ascii_alphanumeric() || self.ch == '_' {
            ident.push(self.ch);
            self.read_char();
        }
        ident
    }

    pub fn read_number(&mut self) -> String {
        let mut number = String::new();
        while self.ch.is_ascii_digit() {
            number.push(self.ch);
            self.read_char();
        }
        number
    }

    pub fn read_string(&mut self) -> String {
        let mut string = String::new();
        self.read_char();
        while self.ch != '"' {
            string.push(self.ch);
            self.read_char();
        }

        self.read_char();
        string
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch {
            'a'..='z' | 'A'..='Z' | '_' => {
                return Token::from(self.read_identifer());
            }
            '0'..='9' => {
                return Token::Int(self.read_number().parse::<isize>().unwrap());
            }
            '"' => {
                return Token::String(self.read_string());
            }
            '=' | '!' => {
                if self.peek_char() == '=' {
                    let mut s = String::new();
                    s.push(self.ch);
                    self.read_char();
                    s.push(self.ch);
                    Token::from(s)
                } else {
                    Token::from(self.ch)
                }
            }
            _ => Token::from(self.ch),
        };

        self.read_char();
        token
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string() {
        let input = String::from(
            r#"
        "foobar"

        "foo bar"

        "1"
        "#,
        );

        let mut lexer = Lexer::new(input);

        let tests = vec![
            Token::String(String::from("foobar")),
            Token::String(String::from("foo bar")),
            Token::String(String::from("1")),
        ];

        for (i, t) in tests.iter().enumerate() {
            let tok = lexer.next_token();
            assert_eq!(tok, *t, "Test index: {i} | Expected: {t} | Got: {tok}");
        }
    }

    #[test]
    fn test_next_token() {
        let input = String::from(
            r#"
        atom five = 5;
        atom ten = 10;

        reaction add(x, y) {
            x + y;
        };

        molecule result = add(five, ten);

        !-/*5;
        5 < 10 > 5;

        if (5 < 10) {
            produce true;
        } else {
            produce false;
        }

        10 == 10;
        10 != 9;

        "foobar"
        "foo bar"
        [1, 2];
        {"foo": "bar"}
        "#,
        );

        let mut lexer = Lexer::new(input);

        let tests = vec![
            Token::Atom,
            Token::Ident(String::from("five")),
            Token::Assign,
            Token::Int(5),
            Token::Semicolon,
            Token::Atom,
            Token::Ident(String::from("ten")),
            Token::Assign,
            Token::Int(10),
            Token::Semicolon,
            Token::Reaction,
            Token::Ident(String::from("add")),
            Token::LParen,
            Token::Ident(String::from("x")),
            Token::Comma,
            Token::Ident(String::from("y")),
            Token::RParen,
            Token::LBrace,
            Token::Ident(String::from("x")),
            Token::Plus,
            Token::Ident(String::from("y")),
            Token::Semicolon,
            Token::RBrace,
            Token::Semicolon,
            Token::Molecule,
            Token::Ident(String::from("result")),
            Token::Assign,
            Token::Ident(String::from("add")),
            Token::LParen,
            Token::Ident(String::from("five")),
            Token::Comma,
            Token::Ident(String::from("ten")),
            Token::RParen,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int(5),
            Token::Semicolon,
            Token::Int(5),
            Token::Lt,
            Token::Int(10),
            Token::Gt,
            Token::Int(5),
            Token::Semicolon,
            Token::If,
            Token::LParen,
            Token::Int(5),
            Token::Lt,
            Token::Int(10),
            Token::RParen,
            Token::LBrace,
            Token::Produce,
            Token::True,
            Token::Semicolon,
            Token::RBrace,
            Token::Else,
            Token::LBrace,
            Token::Produce,
            Token::False,
            Token::Semicolon,
            Token::RBrace,
            Token::Int(10),
            Token::Eq,
            Token::Int(10),
            Token::Semicolon,
            Token::Int(10),
            Token::NotEq,
            Token::Int(9),
            Token::Semicolon,
            Token::String(String::from("foobar")),
            Token::String(String::from("foo bar")),
            Token::LBracket,
            Token::Int(1),
            Token::Comma,
            Token::Int(2),
            Token::RBracket,
            Token::Semicolon,
            Token::LBrace,
            Token::String(String::from("foo")),
            Token::Colon,
            Token::String(String::from("bar")),
            Token::RBrace,
            Token::Eof,
        ];

        for (i, t) in tests.iter().enumerate() {
            let tok = lexer.next_token();
            assert_eq!(tok, *t, "Test no. {}, Expected {}, got {}", i + 1, t, tok);
        }
    }
}
