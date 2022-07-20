use std::iter::Peekable;
use std::str::Chars;
use crate::lexer::token;
use crate::lexer::token::Token;

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer {
        Lexer {
            input: input.chars().peekable()
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.eat_whitespace();
        match self.read_char() {
            Some('=') => {
                if let Some('=') = self.peek_char() {
                    self.read_char();
                    Token::EQ
                } else {
                    Token::ASSIGN
                }
            }
            Some('+') => Token::PLUS,
            Some('(') => Token::LPAREN,
            Some(')') => Token::RPAREN,
            Some('{') => Token::LBRACE,
            Some('}') => Token::RBRACE,
            Some('[') => Token::LBRACKET,
            Some(']') => Token::RBRACKET,
            Some(',') => Token::COMMA,
            Some(';') => Token::SEMICOLON,
            Some(':') => Token::COLON,
            Some('-') => Token::MINUS,
            Some('!') => {
                if let Some('=') = self.peek_char() {
                    self.read_char();
                    Token::NEQ
                } else {
                    Token::BANG
                }
            },
            Some('*') => Token::ASTERISK,
            Some('/') => Token::SLASH,
            Some('<') => Token::LT,
            Some('>') => Token::GT,
            Some(ch) => {
                if is_letter(ch) {
                    let ident = self.read_identifier(ch);
                    let tok = token::lookup_ident(ident);
                    tok
                } else if ch.is_digit(10) {
                    Token::INT(self.read_int(ch))
                } else if ch == '"' {
                    Token::STRING(self.read_string())
                } else {
                    Token::ILLEGAL
                }
            },
            None => Token::EOF,
        }
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn read_char(&mut self) -> Option<char> {
        self.input.next()
    }

    fn eat_whitespace(&mut self) {
        while let Some(&ch) = self.input.peek() {
            if ch.is_whitespace() {
                self.read_char();
            } else {
                break;
            }
        }
    }

    fn read_string(&mut self) -> String {
        let mut str = String::new();
        while let Some(ch) = self.read_char() {
            if ch == '"' {
                return str;
            }
            str.push(ch);
        }
        str
    }

    fn read_int(&mut self, ch: char) -> i64 {
        let mut s = String::new();
        s.push(ch);

        while let Some(&ch) = self.peek_char() {
            if ch.is_digit(10) {
                s.push(self.read_char().unwrap());
            } else {
                break;
            }
        }
        s.parse().unwrap()
    }

    fn read_identifier(&mut self, ch: char) -> String {
        let mut ident = String::new();
        ident.push(ch);

        while let Some(&ch) = self.peek_char() {
            if is_letter(ch) {
                ident.push(self.read_char().unwrap());
            } else {
                break;
            }
        }
        ident
    }
}

fn is_letter(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peek_char() {
        let input: &str = "a b c";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.peek_char(), Some(&'a'));
        lexer.read_char();
        assert_eq!(lexer.peek_char(), Some(&' '));
        lexer.read_char();
        assert_eq!(lexer.peek_char(), Some(&'b'));
        lexer.read_char();
        assert_eq!(lexer.peek_char(), Some(&' '));
        lexer.read_char();
        assert_eq!(lexer.peek_char(), Some(&'c'));
    }

    #[test]
    fn test_read_char() {
        let input: &str = "a b c";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.read_char(), Some('a'));
        assert_eq!(lexer.read_char(), Some(' '));
        assert_eq!(lexer.read_char(), Some('b'));
        assert_eq!(lexer.read_char(), Some(' '));
        assert_eq!(lexer.read_char(), Some('c'));
    }

    #[test]
    fn test_eat_whitespace() {
        let input: &str = "a b c";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.read_char(), Some('a'));
        lexer.eat_whitespace();
        assert_eq!(lexer.read_char(), Some('b'));
        lexer.eat_whitespace();
        assert_eq!(lexer.read_char(), Some('c'));
    }

    #[test]
    fn test_read_string() {
        let input: &str = "Hello";
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.read_string(), "Hello");
    }

    #[test]
    fn test_read_int() {
        let input: &str = "100";
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.read_int('1'), 1100);
    }

    #[test]
    fn test_read_identifier() {
        let input: &str = "et";
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.read_identifier('l'), "let");
    }

    #[test]
    fn test_next_token() {
        let input = r#"let five = 5;
let ten = 10;
let add = fn(x, y) {
  x + y;
};
let result = add(five, ten);
!-/*5;
5 < 10 > 5;
if (5 < 10) {
    return true;
} else {
    return false;
}
10 == 10;
10 != 9;
"foobar"
"foo bar"
[1, 2];
{"foo": "bar"}"#;

        let tests = vec![
            Token::LET,
            Token::IDENT("five".to_string()),
            Token::ASSIGN,
            Token::INT(5),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("ten".to_string()),
            Token::ASSIGN,
            Token::INT(10),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("add".to_string()),
            Token::ASSIGN,
            Token::FUNCTION,
            Token::LPAREN,
            Token::IDENT("x".to_string()),
            Token::COMMA,
            Token::IDENT("y".to_string()),
            Token::RPAREN,
            Token::LBRACE,
            Token::IDENT("x".to_string()),
            Token::PLUS,
            Token::IDENT("y".to_string()),
            Token::SEMICOLON,
            Token::RBRACE,
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("result".to_string()),
            Token::ASSIGN,
            Token::IDENT("add".to_string()),
            Token::LPAREN,
            Token::IDENT("five".to_string()),
            Token::COMMA,
            Token::IDENT("ten".to_string()),
            Token::RPAREN,
            Token::SEMICOLON,
            Token::BANG,
            Token::MINUS,
            Token::SLASH,
            Token::ASTERISK,
            Token::INT(5),
            Token::SEMICOLON,
            Token::INT(5),
            Token::LT,
            Token::INT(10),
            Token::GT,
            Token::INT(5),
            Token::SEMICOLON,
            Token::IF,
            Token::LPAREN,
            Token::INT(5),
            Token::LT,
            Token::INT(10),
            Token::RPAREN,
            Token::LBRACE,
            Token::RETURN,
            Token::TRUE,
            Token::SEMICOLON,
            Token::RBRACE,
            Token::ELSE,
            Token::LBRACE,
            Token::RETURN,
            Token::FALSE,
            Token::SEMICOLON,
            Token::RBRACE,
            Token::INT(10),
            Token::EQ,
            Token::INT(10),
            Token::SEMICOLON,
            Token::INT(10),
            Token::NEQ,
            Token::INT(9),
            Token::SEMICOLON,
            Token::STRING("foobar".to_string()),
            Token::STRING("foo bar".to_string()),
            Token::LBRACKET,
            Token::INT(1),
            Token::COMMA,
            Token::INT(2),
            Token::RBRACKET,
            Token::SEMICOLON,
            Token::LBRACE,
            Token::STRING("foo".to_string()),
            Token::COLON,
            Token::STRING("bar".to_string()),
            Token::RBRACE,
            Token::EOF,
        ];

        let mut l = Lexer::new(input);

        for t in tests.iter() {
            let tok = l.next_token();

            assert_eq!(*t, tok, "expected {} token but got {}", t, tok)
        }
    }
}
