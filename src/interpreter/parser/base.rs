use std::fmt;

use crate::{collections::tree::Tree, interpreter::lexer::base::Token};

struct SyntaxToken{
}

pub struct Parser{
   ast : Tree<SyntaxToken> 
}

impl Parser{
    pub fn new(){
        /*return Parser{
            ast: Tree::<SyntaxToken>::new()
        };*/
    }

    pub fn parse(&self, tokens : Vec<Token>){
        
    }
}