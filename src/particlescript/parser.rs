use std::rc::Rc;

use crate::particlescript::{lexer::Token, lexer::TokenType::*, types::Type};
use itertools::Itertools;


macro_rules! match_tokens {
    ( $tokens:expr, $( $x:pat_param ),* ) => {
        {
            //let tokens: itertools::MultiPeek<std::slice::IntoIter<'_, Token>> = $tokens;
            (move ||{
                let mut cnt = 0;
        $( 
            let Some(t) = $tokens.peek() else {return None;};
            match t.token_type{
                $x => {cnt += 1},
                _ => {return None;}
            }
        )*

        // Let's collect the tokens
        let mut res = Vec::<Token>::with_capacity(cnt);

        $(
            // We gotta use $x somewhere, let's just hope the compiler optimizes this away
            matches!(TokenType::Semicolon, $x);
            res.push($tokens.next().unwrap());
        )*
        
        return Some(res);
    })()
    }
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

    if let Some(tok) = match_tokens!(tokens,
        Identifier(name),
        Equals
    ){
        println!("{}", name);
    }
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

        let res = match_tokens!(
            tokens,
            TokenType::Identifier(_),
            TokenType::OpeningParenthesis,
            TokenType::ClosingParenthesis
        );

        assert!(res.is_some())
    }
}