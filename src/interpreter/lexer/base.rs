#[derive(Debug)]

pub struct Token{

}

pub struct Lexer{
    index : usize,
    tokens : Vec<Token>
}

impl Lexer{
    pub fn new() -> Lexer{
        return Lexer{index: 0, tokens : vec![]};
    }

    pub fn toString(&self){
        print!("{:?}", self.tokens);
    }

    fn next(&mut self){
        self.index += 1;
    }

    pub fn getTokens(&self, line : String){
        
    }
}