use std::fmt;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum Token {
    ILLEGAL,
    EOF,
    // 标识符+字面量
    IDENT(String),
    INT(i64),
    STRING(String),
    // 运算符
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    LT,
    GT,
    EQ,
    NEQ,
    // 分隔符
    COMMA,
    SEMICOLON,
    COLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    LBRACKET,
    RBRACKET,
    // 关键字
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::IDENT(name) => write!(f, "{}", name),
            Token::INT(val) => write!(f, "{}", val),
            Token::MINUS => write!(f, "-"),
            Token::PLUS => write!(f, "+"),
            Token::BANG => write!(f, "!"),
            Token::ASTERISK => write!(f, "*"),
            Token::SLASH => write!(f, "/"),
            Token::GT => write!(f, ">"),
            Token::LT => write!(f, "<"),
            Token::EQ => write!(f, "=="),
            Token::NEQ => write!(f, "!="),
            Token::SEMICOLON => write!(f, ";"),
            Token::COLON => write!(f, ":"),
            Token::ASSIGN => write!(f, "="),
            Token::FUNCTION => write!(f, "fn"),
            Token::LPAREN => write!(f, "("),
            Token::RPAREN => write!(f, ")"),
            Token::LBRACE => write!(f, "{{"),
            Token::RBRACE => write!(f, "}}"),
            Token::COMMA => write!(f, ","),
            Token::LBRACKET => write!(f, "["),
            Token::RBRACKET => write!(f, "]"),
            tok => write!(f, "{:?}", tok),
        }
    }
}

pub fn lookup_ident(ident: String) -> Token {
    match ident.as_str() {
        "let" => Token::LET,
        "fn" => Token::FUNCTION,
        "true" => Token::TRUE,
        "false" => Token::FALSE,
        "if" => Token::IF,
        "else" => Token::ELSE,
        "return" => Token::RETURN,
        _ => Token::IDENT(ident),
    }
}

impl Token {
    pub fn is_ident(&self) -> bool {
        match self {
            Token::IDENT(_) => true,
            _ => false,
        }
    }

    pub fn is_int(&self) -> bool {
        match self {
            Token::INT(_) => true,
            _ => false,
        }
    }
}
