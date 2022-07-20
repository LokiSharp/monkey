type TokenType = str;
struct Token {
    token_type: &'static TokenType,
    literal: &'static str,
}

const ILLEGAL: &str = "ILLEGAL";
const EOF: &str = "EOF";
// 标识符+字面量
const IDENT: &str = "IDENT"; // add, foobar, x, y, ...
const INT: &str = "INT"; // 1343456
// 运算符
const ASSIGN: &str = "=";
const PLUS: &str = "+";
// 分隔符
const COMMA: &str = ",";
const SEMICOLON: &str = ";";
const LPAREN: &str = "(";
const RPAREN: &str = ")";
const LBRACE: &str = "{";
const RBRACE: &str = "}";
// 关键字
const FUNCTION: &str = "FUNCTION";
const LET: &str = "LET";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input: &str = "=+(){},;";

        let tests: [Token; 9] = [
            Token { token_type: ASSIGN, literal: "=" },
            Token { token_type: PLUS, literal: "+" },
            Token { token_type: LPAREN, literal: "(" },
            Token { token_type: RPAREN, literal: ")" },
            Token { token_type: LBRACE, literal: "{" },
            Token { token_type: RBRACE, literal: "}" },
            Token { token_type: COMMA, literal: "," },
            Token { token_type: SEMICOLON, literal: ";" },
            Token { token_type: EOF, literal: "" },
        ];
    }
}