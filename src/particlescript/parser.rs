use std::{rc::Rc};

use crate::particlescript::{lexer::{Token, TokenType::*}, types::{base_types, Type, Value, ValueData}};
use itertools::{Itertools, MultiPeek};


#[derive(Debug)]
pub enum Stmt{
    Assignment{
        variable: Rc<Variable>,
        value: Box<Stmt>,
    },
    Literal(Value)
}

impl Stmt{
    fn return_type(&self, scope: &Scope) -> Rc<Type>{
        match self{
            Stmt::Assignment{variable: _, value: _} => scope.find_type("void").unwrap(),
            Stmt::Literal(val) => val.typ.clone()
        }
    }
}

macro_rules! match_tokens {
    ( $tokens:expr, $( $x:pat_param ),* ) => {

        $( 
            let Some(t) = $tokens.peek() else { return Option::<Stmt>::None};
            let $x = t.token_type.clone() else {return Option::<Stmt>::None};
        )*


        $(
            // We gotta use $x somewhere, let's just hope the compiler optimizes this away
            matches!(Semicolon, $x);
            $tokens.next().unwrap();
        )*
    };
}

#[derive(Debug)]
struct Variable{
    name: String,
    typ: Rc<Type>,
}


pub struct Scope{
    variables: Vec<Rc<Variable>>,
    types: Vec<Rc<Type>>,
    parent_scope: Option<Box<Scope>>,
}

impl Scope{
    pub fn root() -> Self{
        Self { variables: vec![], types: base_types().into_iter().map(|t| Rc::new(t)).collect(), parent_scope: None }
    }

    fn find_variable(&self, name: &str) -> Option<Rc<Variable>>{
        self.variables.iter().find(|v| v.name == name).map(Clone::clone).or_else(||{
            self.parent_scope.as_ref().and_then(|p| p.find_variable(name))
        })
    }

    fn declare_variable(&mut self, name: String, typ: Rc<Type>) -> Rc<Variable>{
        let variable = Rc::new(Variable{
            name,
            typ
        });
        self.variables.push(variable.clone());
        variable
    }

    fn find_type(&self, name: &str) -> Option<Rc<Type>>{
        self.types.iter().find(|t| t.name == name).map(Clone::clone).or_else(|| self.parent_scope.as_ref().and_then(|p| p.find_type(name)))
    }
}


type ParserFn = fn() -> bool;
pub fn parse<T>(
    tokens: &mut MultiPeek<T>,
    scope: &mut Scope
) -> Option<Stmt>
where T: Iterator<Item = Token>{
    

    let parsers: [Box<dyn Fn(&mut Scope, &mut MultiPeek<T>) -> Option<Stmt>>; _] = [Box::new(|scope, tokens|{
        match_tokens!(tokens,
            Let,
            Identifier(var_name),
            Equals
        );

        let expr = parse(tokens, scope).expect("No expression after variable declaration");

        let variable = scope.declare_variable(var_name, scope.find_type("int").unwrap());
        Some(Stmt::Assignment { variable, value: Box::new(expr)})
    }),
    Box::new(|scope, tokens|{
        match_tokens!(tokens,
            IntLiteral(v)
        );

        Some(Stmt::Literal(Value{
            typ: scope.find_type("int").unwrap(),
            data: ValueData::Int(v)
        }))
    })];

    for parser in parsers{
        if let Some(stmt) = parser(scope, tokens){
            println!("Parsed stmt: {:#?}", stmt);
            return Some(stmt);
        }else{
            tokens.reset_peek();
        }
    }

    if let Some(last_tok) = tokens.peek(){
        panic!("Failed to parse statement at {}:{}", last_tok.line, last_tok.column);
    }else{
        panic!("Unexpected end of file")
    }
}


#[cfg(test)]
mod test{
    use std::rc::Rc;

    use crate::particlescript::{lexer::{Token, TokenType}, parser::{parse, Scope, Stmt}, types::base_types};

/*     #[test]
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

            
        })();

        assert!(res)
    } */

    #[test]
    fn parse_variable_declaration(){
        let tokens = [
            TokenType::Let,
            TokenType::Identifier(String::from("a")),
            TokenType::Equals,
            TokenType::IntLiteral(5)
        ].map(|t| Token{ line: 0, column: 0, token_type: t});

        let mut scope = Scope{
            variables: vec![],
            types: base_types().into_iter().map(Rc::new).collect(),
            parent_scope: None,
        };
        let Some(stmt) = parse(tokens.into_iter(), &mut scope) else { panic!("Parser failed") };

        let Some(v) = scope.variables.get(0) else { panic!("Variable was not declared") };

        match stmt {
            Stmt::Assignment { variable, value } => {
                assert!(Rc::ptr_eq(v, &variable));
            }
            _ => panic!("Stmt is not an assignment"),
        }
        
    }
}