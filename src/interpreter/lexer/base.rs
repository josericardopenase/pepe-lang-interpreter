
use colored::Colorize;
#[derive(Debug)]
#[derive(PartialEq)]
pub enum TokenType{
    NumericToken,
    PlusToken,
    MinusToken,
    LPar,
    RPar,
    LiteralToken,
    AssignToken,
}


#[derive(Debug)]
pub struct Token{
    pub token_type : TokenType,
    pub init : usize,
    pub end : usize,
    pub val : String
}

impl Token{
    pub fn new(token_type : TokenType, init : usize, end : usize, val : String) -> Token{
        return Token { token_type: token_type, init: init,  end: end, val: val };
    }
}

pub struct Lexer{
    index : usize,
    pub tokens : Vec<Token>
}

impl Lexer{
    pub fn new() -> Lexer{
        return Lexer{index: 0, tokens : vec![]};
    }

    pub fn to_string(&self){
        for x in self.tokens.iter(){
            println!("Type: {:?}, Value: {}", x.token_type, x.val);
        }
    }

    fn next(&mut self){
        self.index += 1;
    }

    fn reset(&mut self){
        self.tokens.clear();
        self.index=0;
    }

    pub fn get_tokens(&mut self, line : String){

        self.reset();

        //FIXME: ESTO NECESITA UN REFACTOR IMPORTANTE usando expresiones regulares
        if line.trim() == ".exit"{
            println!("{}", "Bye, bye :(".red());
            std::process::exit(0);
        }

        while self.index  < line.len() {
            let mut curr : char = line.chars().nth(self.index).unwrap();

            if(curr == '+'){
                self.tokens.push(Token::new(TokenType::PlusToken, self.index, self.index, String::from("+")));
                self.next();
            }else if(curr == '-'){
                self.tokens.push(Token::new(TokenType::MinusToken, self.index, self.index, String::from("-")));
                self.next();
            }else if(curr == '='){
                self.tokens.push(Token::new(TokenType::AssignToken, self.index, self.index, String::from("=")));
                self.next();
            }else if(curr == '('){
                self.tokens.push(Token::new(TokenType::LPar, self.index, self.index, String::from("(")));
                self.next();
            }else if(curr == ')'){
                self.tokens.push(Token::new(TokenType::RPar, self.index, self.index, String::from(")")));
                self.next();
            }else if(curr.is_ascii_digit()){
                let init = self.index;
                while curr.is_ascii_digit(){
                    self.next();
                    curr = line.chars().nth(self.index).unwrap();
                }
                self.tokens.push(Token::new(TokenType::NumericToken, init, self.index, String::from(&line[init..self.index])));
            }else if(curr.is_alphabetic()){
                let init = self.index;
                while curr.is_alphabetic(){
                    self.next();
                    curr = line.chars().nth(self.index).unwrap();
                }
                self.tokens.push(Token::new(TokenType::LiteralToken, init, self.index, String::from(&line[init..self.index])));
            }else{
                self.next()
            }

        }
            
    }
}