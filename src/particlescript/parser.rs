use std::{rc::Rc, sync::atomic::AtomicUsize};

use crate::particlescript::{lexer::{Token, TokenType}, types::Type};
use bevy::{render::render_resource::encase::private::Length, tasks::futures_lite::StreamExt, text::cosmic_text::ttf_parser::morx::Chain};
use itertools::{Itertools, MultiPeek};


// pub struct Parser<T>
// where T: Iterator<Item = Token>{
//    tokens: T
// }

// impl<T:Iterator<Item = Token>> Parser<T>{
    
// }


// trait PeekSlice<T, I>
// where T: Iterator<Item = I>{
//     fn peek<'a>(&'a mut self, length: usize) -> Vec<&'a I>;    
// }

// impl<T: Iterator<Item = I>, I> PeekSlice<T, I> for MultiPeek<T>{
//     fn peek<'a>(&'a mut self, length: usize) -> Vec<&'a I> {

//         let mut v = Vec::<&I>::with_capacity(length);

//         self
//                     v.push(self.peek().unwrap());
//                                 v.push(self.peek().unwrap());


//         for i in 0..length{
//         }

//         v
//     }
// }


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



trait ChainElement{
    fn append<const LENGTH:usize>(self, new: StmtDef<LENGTH>) -> Definition<LENGTH, Self>
    where Self: Sized;
}
struct StmtDef<const LENGTH: usize>{
    pub pattern: [TokenType; LENGTH],
}
impl<const LENGTH:usize> StmtDef<LENGTH>{
    pub fn new(pattern: [TokenType; LENGTH]) -> Self{
        Self { pattern }
    }
}


struct StmtDefinitionChain;

impl ChainElement for StmtDefinitionChain{
    fn append<const LENGTH: usize>(self, new: StmtDef<LENGTH>) -> Definition<LENGTH, Self>
    where Self: Sized {
        Definition { def: new, next: Self }
    }
}

/// The end of the chain
struct Definition<const LENGTH:usize, Next>
where Next: ChainElement{
    pub def: StmtDef<LENGTH>,
    pub next: Next
}
impl<const LENGTH:usize, Next: ChainElement> ChainElement for Definition<LENGTH, Next>{
    fn append<const n:usize>(self, new: StmtDef<{n}>) -> Definition<n, Self>{
        Definition { 
            def: new,
            next: self 
        }
    }
}

fn parse<T>(
    tokens: T,
    scope: &mut Scope
) 
where T: Iterator<Item = Token>{
    let mut tokens = tokens.multipeek();

    let t = tokens.next().unwrap();
}

#[cfg(test)]
mod test{
    use crate::particlescript::{lexer::TokenType, parser::{ChainElement, Definition, StmtDef, StmtDefinitionChain}};

    #[test]
    fn defs(){
        let defs = StmtDefinitionChain{}
            .append(StmtDef::new([TokenType::Let]))
            .append(StmtDef::new([TokenType::For, TokenType::OpeningParenthesis]));
    }
}