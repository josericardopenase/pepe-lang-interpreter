use std::{fmt, ops::Index};

use crate::{collections::tree::{Tree, Node}, interpreter::lexer::base::{Token, TokenType}};


#[derive(Debug)]
enum SyntaxTokenTypes{
    BinaryToken,
    UnaryToken 
}

#[derive(Debug)]
struct SyntaxToken{
    pub token_type : SyntaxTokenTypes,
    pub value : Token
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

    pub fn parse(&self, tokens : &mut Vec<Token>){

        let binary_tokens = [TokenType::MinusToken, TokenType::PlusToken, TokenType::AssignToken];

        let mut stokens : Vec<Node<SyntaxToken>> = vec![];

        for x in 0..tokens.len() {
            if binary_tokens.contains(&tokens[x].token_type) {
                stokens[x] = Node::<SyntaxToken>::new(
                    SyntaxToken { token_type:  SyntaxTokenTypes::BinaryToken, value: tokens[x]}
                );
            }else {
                    stokens[x] = Node::<SyntaxToken>::new( SyntaxToken { token_type:  SyntaxTokenTypes::UnaryToken , value: tokens[x]});
            }
        }


        for i in 0..stokens.len(){
            match stokens[i].value.token_type {
                SyntaxTokenTypes::BinaryToken  => print!("hello world"),
                SyntaxTokenTypes::UnaryToken  => print!("hello world")

            }
        }
    }
        
}