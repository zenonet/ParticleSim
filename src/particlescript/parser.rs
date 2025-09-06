use std::rc::Rc;

use crate::particlescript::{lexer::Token, types::Type};
use itertools::Itertools;



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

    let t = tokens.next().unwrap();
}

macro_rules! match_tokens {
    ( $tokens:expr, $( $x:pat_param ),* ) => {
        {
            (move ||{
        $( 
            let Some(t) = $tokens.peek() else {return false;};
            match t.token_type{
                $x => {},
                _ => {return false;}
            }
        )*
        return true;
    })()
    }
    };
}

#[cfg(test)]
mod test{
    use itertools::Itertools;

    use crate::particlescript::{lexer::{Token, TokenType}}

    #[test]
    fn defs(){
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
        let mut tokens = tokens.iter().multipeek();

        let res:bool = match_tokens!(
            tokens,
            TokenType::Identifier(_),
            TokenType::OpeningParenthesis,
            TokenType::ClosingParenthesis
        );

        assert!(res)
    }
}