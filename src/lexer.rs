use std::ops::Index;

enum TokenTypes{
    NumberToken,
    PlusToken,
    MinusToken,
    MulToken
}
pub struct Token{
    token_type: TokenTypes,
    text : String,
    index : usize
}

impl Token{
    pub fn to_string(&self) -> String{
        match self.token_type{
            TokenTypes::MinusToken => String::from("MinusToken"),
            TokenTypes::MulToken => String::from("MulToken"),
            TokenTypes::NumberToken => String::from("NumberToken"),
            TokenTypes::PlusToken => String::from("PlusToken")
        }
    }
}

pub struct Lexer{
    line : String,
    start: usize,
    pub elements : Vec<Token>
}

impl Lexer{
    pub fn new(line : String) -> Lexer{
        Lexer {
             line : line,
            start : 0,
            elements: Vec::new()
        }
    }

    fn next(&mut self){
        self.start += 1;
    }

    pub fn eval(&mut self){
        loop{
            if self.start >= self.line.len(){
                break;
            }

            let mut curr : char = self.line.chars().nth(self.start).unwrap();

            if curr.is_ascii_digit() {

                let n_start = self.start;
                while curr.is_ascii_digit(){
                    self.next();
                    curr = self.line.chars().nth(self.start).unwrap();
                }

                self.elements.push(Token { token_type: TokenTypes::NumberToken, text: String::from(&self.line[n_start..self.start]), index: n_start })
            }

            if curr == '+'{
                self.elements.push(Token{
                    token_type: TokenTypes::PlusToken ,
                    text: String::from("+"), index : self.start
                })
            }

            if curr == '-'{
                self.elements.push(Token{
                    token_type: TokenTypes::MinusToken ,
                    text: String::from("-"),
                    index : self.start
                })
            }

            if curr == '*' {
                self.elements.push(Token{
                    token_type: TokenTypes::MulToken ,
                    text: String::from("-"),
                    index : self.start
                })
            }

            self.next();
        }
    }
}