use std::{iter::{self}};
use peeking_take_while::PeekableExt;

pub fn lex(code: &str) -> Result<Vec<Token>, String>{
    let iter = code.chars();
    let iter = &mut iter.peekable();

    let mut tokens = Vec::<Token>::new();
    while let Some(char) = iter.next(){

        const WHITESPACE_START: char = 0u8 as char;
        const WHITESPACE_END: char = 31u8 as char;
        let token = match char{
            WHITESPACE_START..=WHITESPACE_END => { continue; },
            '(' => Token::OpeningParenthesis,
            ')' => Token::ClosingParenthesis,
            '{' => Token::OpeningCurlyBrace,
            '}' => Token::ClosingCurlyBrace,
            ';' => Token::Semicolon,
            '.' => Token::Dot,
            c @ '0'..='9' => {
                let mut value: f64 = 0.0;
                for c in iter::once(c).chain(iter.by_ref().peeking_take_while(|c| matches!(c, '0'..='9' ))){
                    let digit = c as u8 - 48u8;
                    println!("Adding digit: {digit}");
                    value = value * 10.0 + digit as f64;
                }
                println!("val: {value}");
                if matches!(iter.peek(), Some('.')){
                    iter.next(); // skip the dot
                    // Add digits after point
                    let mut digits_after_point = 0;
                    for c in iter.by_ref().peeking_take_while(|c| matches!(c, '0'..='9' )){
                        let digit = c as u8 - 48u8;
                        value = value * 10.0 + digit as f64;
                        digits_after_point += 1;
                    }
                    value /= 10_i32.pow(digits_after_point) as f64;
                }

                Token::FloatLiteral(value as f32)
            },
            '=' => Token::Equals,
            first @ ('A'..='Z' | 'a'..='z' | '_') => {
                Token::Identifier(iter::once(first).chain(iter.by_ref().take_while(|c| matches!(c, 'A'..='Z' | 'a'..='z' | '_' | '0'..='9'))).collect::<String>())
            }
            _ =>{
                return Err(format!("Unknown token {}", iter.collect::<String>()));
            }
                
        };

        tokens.push(token);
    }

    Ok(tokens)
}

#[derive(PartialEq, Clone, Debug)]
pub enum Token{
    Identifier(String),
    IntLiteral(i32),
    FloatLiteral(f32),
    OpeningParenthesis,
    ClosingParenthesis,
    OpeningCurlyBrace,
    ClosingCurlyBrace,
    Semicolon,
    Equals,
    Dot,
}

#[cfg(test)]
mod lexer_tests{
    use crate::particlescript::lexer::{lex, Token};

    #[test]
    fn test1(){
        let tokens = lex("sean =()yay").unwrap();

        assert_eq!(tokens, vec![
            Token::Identifier(String::from("sean")),
            Token::Equals,
            Token::OpeningParenthesis,
            Token::ClosingParenthesis,
            Token::Identifier(String::from("yay"))
        ]);
    }

    #[test]
    fn float_literals_simple(){
        assert_eq!(
            lex("12.543.d").unwrap(),
            vec![
                Token::FloatLiteral(12.543),
                Token::Dot,
                Token::Identifier(String::from("d"))
            ]
        )
    }
}
