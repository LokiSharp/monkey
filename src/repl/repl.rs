use std::cell::RefCell;
use std::io;
use std::rc::Rc;
use crate::lexer::lexer::Lexer;
use crate::lexer::token::Token;

const PROMPT: &str = ">> ";

pub fn start<R: io::BufRead, W: io::Write>(mut reader: R, mut writer: W) -> io::Result<()> {
    loop {
        writer.write(PROMPT.as_ref());
        writer.flush();
        let mut line = String::new();
        reader.read_line(&mut line)?;

        let mut l = Lexer::new(&line);

        loop {
            let tok = l.next_token();
            println!("{}", tok);
            if tok == Token::EOF {
                break;
            }
        }
    }
}