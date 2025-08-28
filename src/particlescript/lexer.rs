use std::{iter::{self}};
use itertools::Itertools;

pub fn lex(code: &str) -> Result<Vec<Token>, String>{
    let iter = code.chars();
    let iter = &mut iter.multipeek();

    let mut tokens = Vec::<Token>::new();
    while let Some(mut char) = iter.next(){

        const WHITESPACE_START: char = 0u8 as char;
        const WHITESPACE_END: char = 32u8 as char;

        let is_negative_number = if matches!(char, '-') && iter.peek().is_some_and(|c| matches!(c, '0'..='9')){
            char = iter.next().unwrap();
            true
        }else{
            false
        };
        iter.reset_peek();

        let token = match char{
            WHITESPACE_START..=WHITESPACE_END => { continue; },
            '(' => Token::OpeningParenthesis,
            ')' => Token::ClosingParenthesis,
            '{' => Token::OpeningCurlyBrace,
            '}' => Token::ClosingCurlyBrace,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Asterisk,
            '/' => Token::Slash,
            ';' => Token::Semicolon,
            '.' => Token::Dot,
            c @ '0'..='9' => {
                let mut value: f64 = 0.0;
                for c in iter::once(c).chain(iter.peeking_take_while(|c| matches!(c, '0'..='9'))){
                    let digit = c as u8 - 48u8;
                    println!("Adding digit: {digit}");
                    value = value * 10.0 + digit as f64;
                }
                println!("Dot");
                iter.reset_peek();
                if matches!(iter.peek(), Some('.')){
                    iter.next(); // skip the dot
                    // Add digits after point
                    let mut digits_after_point = 0;
                    for c in iter.peeking_take_while(|c| matches!(c, '0'..='9' )){
                        let digit = c as u8 - 48u8;
                        println!("Adding digit: {digit}");
                        value = value * 10.0 + digit as f64;
                        digits_after_point += 1;
                    }
                    value /= 10_i32.pow(digits_after_point) as f64;
                    Token::FloatLiteral(value as f32 * (if is_negative_number{-1.0} else {1.0} ))
                }else{
                    Token::IntLiteral(value as i32 * (if is_negative_number{-1} else {1}))
                }
            },
            '=' => Token::Equals,
            first @ ('A'..='Z' | 'a'..='z' | '_') => {
                let word = iter::once(first).chain(iter.take_while(|c| matches!(c, 'A'..='Z' | 'a'..='z' | '_' | '0'..='9'))).collect::<String>();

                match word.as_str(){
                    "if" => Token::If,
                    "else" => Token::Else,
                    "let" => Token::Let,
                    "while" => Token::While,
                    "for" => Token::For,
                    _ => Token::Identifier(word)
                }
            }
            c @ _ =>{
                return Err(format!("Unknown token '{}'", iter::once(c).chain(iter).collect::<String>()));
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
    Plus,
    Minus,
    Asterisk,
    Slash,

    // Keywords:
    If,
    Else,
    While,
    Let,
    For,
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
    #[test]
    fn float_literals_negative(){
        assert_eq!(
            lex("-1.25- 1.25").unwrap(),
            vec![
                Token::FloatLiteral(-1.25),
                Token::Minus,
                Token::FloatLiteral(1.25)
            ]
        )
    }

    #[test]
    fn int_float_differentiation(){
        assert_eq!(
            lex("1.25 1.00 -1 125").unwrap(),
            vec![
                Token::FloatLiteral(1.25),
                Token::FloatLiteral(1.0),
                Token::IntLiteral(-1),
                Token::IntLiteral(125)
            ]
        )
    }
    #[test]
    fn some_keywords(){
        assert_eq!(
            lex("else yeet let say").unwrap(),
            vec![
                Token::Else,
                Token::Identifier(String::from("yeet")),
                Token::Let,
                Token::Identifier(String::from("say"))
            ]
        )
    }
}
