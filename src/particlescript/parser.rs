use std::rc::Rc;

use crate::particlescript::{lexer::Token, lexer::TokenType::*, types::Type};
use itertools::{Itertools, MultiPeek};


macro_rules! match_tokens {
    ( $tokens:expr, $( $x:pat_param ),* ) => {

        $( 
            let Some(t) = $tokens.peek() else { return false};
            let $x = t.token_type.clone() else {return false};
        )*
    };
}

#[derive(Debug)]
struct Variable{
    name: String,
    typ: Rc<Type>,
}


struct Scope{
    variables: Vec<Rc<Variable>>,
    types: Vec<Type>,
    parent_scope: Box<Scope>,
}

impl Scope{
    fn find_variable(&self, name: &str) -> Option<Rc<Variable>>{
        self.variables.iter().find(|v| v.name == name).map(Clone::clone).or_else(||{
            self.parent_scope.find_variable(name)
        })
    }
}

enum Stmt{
    Assignment{
        variable: Rc<Variable>,
        value: Box<Stmt>,
    },
}


fn parse<T>(
    tokens: T,
    scope: &mut Scope
) 
where T: Iterator<Item = Token>{
    let mut tokens = tokens.multipeek();

    

}


#[cfg(test)]
mod test{
    use itertools::Itertools;

    use crate::particlescript::{lexer::{Token, TokenType}};

    #[test]
    fn match_tokens(){
        let tokens = [            
            Token{
                token_type: TokenType::Identifier("ln".to_owned()),
                line: 1,
                column: 1
            },
            Token{
                token_type: TokenType::OpeningParenthesis,
                line: 1,
                column: 5
            },
            Token{
                token_type: TokenType::ClosingParenthesis,
                line: 3,
                column: 2
            }
        ];
        let mut tokens = tokens.into_iter().multipeek();

        let res = (||{
            match_tokens!(
                tokens,
                TokenType::Identifier(k),
                TokenType::OpeningParenthesis,
                TokenType::ClosingParenthesis
            );

            assert_eq!(k, String::from("ln"));

            true
        })();

        assert!(res)
    }
}