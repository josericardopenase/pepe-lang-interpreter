use std::fmt;

use crate::{collections::tree::Tree, interpreter::lexer::base::Token};

struct SyntaxToken{
}

impl fmt::Display for SyntaxToken{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Your implementation here
        // Implement how SyntaxToken should be printed
        write!(f, "<Token>") // Example implementation
    }
}

pub struct Parser{
   ast : Option<Tree<SyntaxToken>>
}

impl Parser{
    pub fn new() -> Parser{
        return Parser{
            ast: Some(Tree::<SyntaxToken>::new())
        }
    }

    pub fn parse(&self, tokens : Vec<Token>){
        
    }
}