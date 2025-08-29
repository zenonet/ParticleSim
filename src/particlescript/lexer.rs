use std::{iter::{self}};
use itertools::{Itertools, MultiPeek, PeekingNext};


struct PositionTrackedChars<I>
where I: Iterator<Item = char>{
    inner: I,
    line: u32,
    column: u32
}
impl<I: Iterator<Item = char>> PositionTrackedChars<I>{
    fn new(inner: I) -> Self{
        Self { inner, line: 1, column: 0 }
    }

    fn position(&self) -> (u32, u32){
        (self.line, self.column)
    }
}

impl<I: Iterator<Item = char>> Iterator for PositionTrackedChars<I>{
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().inspect(|char|{
            match char{
                '\n' => {
                    self.line += 1;
                    self.column = 0;
                },
                _ => self.column += 1,
            }
        })
    }
}

impl<I: Iterator<Item = char>> PositionTrackedChars<MultiPeek<I>>{
    fn peek(&mut self) -> Option<&char>{
        self.inner.peek()
    }

    fn reset_peek(&mut self){
        self.inner.reset_peek()
    }
}

impl<I: Iterator<Item = char> + PeekingNext> PeekingNext for PositionTrackedChars<I>{
    fn peeking_next<F>(&mut self, accept: F) -> Option<Self::Item>
    where
        Self: Sized,
        F: FnOnce(&Self::Item) -> bool {

        self.inner.peeking_next(accept).inspect(|char| match char{
                '\n' => {
                    self.line += 1;
                    self.column = 0;
                },
                _ => self.column += 1,
            }
        )
    }
}


#[derive(Debug, Clone)]
pub struct LexerError{
    line: u32,
    column: u32,
    message: String
}

pub struct Lexer<I>
where I: Iterator<Item = char>{
    source: PositionTrackedChars<MultiPeek<I>>,
    pub error: Option<LexerError>,

    line: i32,
    column: i32,
}

impl<I: Iterator<Item = char>> Lexer<I>{
    pub fn new(source: I) -> Self{
        Self { source: PositionTrackedChars::new(source.multipeek()), error: None, line: 0, column: 0 }
    }
}

impl<I: Iterator<Item = char>> Iterator for Lexer<I>{
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(mut char) = self.source.next(){
            let position = self.source.position();

            const WHITESPACE_START: char = 0u8 as char;
            const WHITESPACE_END: char = 32u8 as char;

            let is_negative_number = if matches!(char, '-') && self.source.peek().is_some_and(|c| matches!(c, '0'..='9')){
                char = self.source.next().unwrap();
                true
            }else{
                false
            };

            self.source.reset_peek();

            let token_type = match char{
                WHITESPACE_START..=WHITESPACE_END => {return self.next()},
                '(' => TokenType::OpeningParenthesis,
                ')' => TokenType::ClosingParenthesis,
                '{' => TokenType::OpeningCurlyBrace,
                '}' => TokenType::ClosingCurlyBrace,
                '+' => TokenType::Plus,
                '-' => TokenType::Minus,
                '*' => TokenType::Asterisk,
                '/' => TokenType::Slash,
                ';' => TokenType::Semicolon,
                '.' => TokenType::Dot,
                c @ '0'..='9' => {
                    let mut value: f64 = 0.0;
                    for c in iter::once(c).chain(self.source.peeking_take_while(|c| matches!(c, '0'..='9'))){
                        let digit = c as u8 - 48u8;
                        println!("Adding digit: {digit}");
                        value = value * 10.0 + digit as f64;
                    }
                    println!("Dot");
                    self.source.reset_peek();
                    if matches!(self.source.peek(), Some('.')){
                        self.source.next(); // skip the dot
                        // Add digits after point
                        let mut digits_after_point = 0;
                        for c in self.source.peeking_take_while(|c| matches!(c, '0'..='9' )){
                            let digit = c as u8 - 48u8;
                            println!("Adding digit: {digit}");
                            value = value * 10.0 + digit as f64;
                            digits_after_point += 1;
                        }
                        value /= 10_i32.pow(digits_after_point) as f64;
                        TokenType::FloatLiteral(value as f32 * (if is_negative_number{-1.0} else {1.0} ))
                    }else{
                        TokenType::IntLiteral(value as i32 * (if is_negative_number{-1} else {1}))
                    }
                },
                '=' => TokenType::Equals,
                first @ ('A'..='Z' | 'a'..='z' | '_') => {
                    let word = iter::once(first).chain(self.source.peeking_take_while(|c| matches!(c, 'A'..='Z' | 'a'..='z' | '_' | '0'..='9'))).collect::<String>();

                    match word.as_str(){
                        "if" => TokenType::If,
                        "else" => TokenType::Else,
                        "let" => TokenType::Let,
                        "while" => TokenType::While,
                        "for" => TokenType::For,
                        _ => TokenType::Identifier(word)
                    }
                }
                c @ _ =>{
                    // TODO: Only show error source until the first whitespace character
                    self.error = Some(LexerError{
                        line: 0,
                        column: 0,
                        message: format!("Unknown token '{}'", iter::once(c).chain(self.source.by_ref()).collect::<String>())
                    });
                    return None;
                }
                    
            };

            Some(Token{
                token_type,
                line: position.0,
                column: position.1
            })
        }else{
            None
        }
    }
}

/// Performs lexical analysis on a &str and returns a vec of tokens.
/// This is for quick testing. Use `Lexer` instead of this for an iterative aproach
fn lex(source: &str) -> Result<Vec<Token>, LexerError>{
    let mut lexer = Lexer::new(source.chars());
    let tokens = lexer.by_ref().collect::<Vec<Token>>();
    if let Some(error) = &lexer.error{
        Err(error.clone())
    }else{
        Ok(tokens)
    }
}
#[derive(PartialEq, Clone, Debug)]
pub struct Token{
    pub line: u32,
    pub column: u32,
    pub token_type: TokenType
}

#[derive(PartialEq, Clone, Debug)]
pub enum TokenType{
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
    use crate::particlescript::lexer::{lex, LexerError, Token, TokenType};

    fn lex_to_types(source: &str) -> Result<Vec<TokenType>, LexerError>{
        lex(source).and_then(|tokens| Ok(tokens.into_iter().map(|token| token.token_type).collect()))
    }

    #[test]
    fn test1(){
        let tokens = lex_to_types("sean =()yay").unwrap();

        assert_eq!(tokens, vec![
            TokenType::Identifier(String::from("sean")),
            TokenType::Equals,
            TokenType::OpeningParenthesis,
            TokenType::ClosingParenthesis,
            TokenType::Identifier(String::from("yay"))
        ]);
    }

    #[test]
    fn float_literals_simple(){
        assert_eq!(
            lex_to_types("12.543.d").unwrap(),
            vec![
                TokenType::FloatLiteral(12.543),
                TokenType::Dot,
                TokenType::Identifier(String::from("d"))
            ]
        )
    }
    #[test]
    fn float_literals_negative(){
        assert_eq!(
            lex_to_types("-1.25- 1.25").unwrap(),
            vec![
                TokenType::FloatLiteral(-1.25),
                TokenType::Minus,
                TokenType::FloatLiteral(1.25)
            ]
        )
    }

    #[test]
    fn int_float_differentiation(){
        assert_eq!(
            lex_to_types("1.25 1.00 -1 125").unwrap(),
            vec![
                TokenType::FloatLiteral(1.25),
                TokenType::FloatLiteral(1.0),
                TokenType::IntLiteral(-1),
                TokenType::IntLiteral(125)
            ]
        )
    }
    #[test]
    fn some_keywords(){
        assert_eq!(
            lex_to_types("else yeet let say").unwrap(),
            vec![
                TokenType::Else,
                TokenType::Identifier(String::from("yeet")),
                TokenType::Let,
                TokenType::Identifier(String::from("say"))
            ]
        )
    }

    #[test]
    fn token_positions(){
        let tokens = lex("ln  4\n\n )").unwrap();
        assert_eq!(
            tokens[0],
            Token{
                token_type: TokenType::Identifier("ln".to_owned()),
                line: 1,
                column: 1
            }
        );

        assert_eq!(
            tokens[1],
            Token{
                token_type: TokenType::IntLiteral(4),
                line: 1,
                column: 5
            }
        );
        assert_eq!(
            tokens[2], 
            Token{
                token_type: TokenType::ClosingParenthesis,
                line: 3,
                column: 2
            }
        );
    }
}
