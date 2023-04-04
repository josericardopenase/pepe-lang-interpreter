use std::io::{self, Read, Write};
use super::{parser::base::Parser, lexer::base::Lexer};

/**
 * Interpreter():
 * 
 * base class for interpretting user commands.
 * Then it will read a file and interpret every single command.
 * 
 */
pub struct Interpreter{
    lexer : Lexer
}

impl Interpreter{
    pub fn new() -> Interpreter{
        return Interpreter{
            lexer: Lexer::new()
        };
    }

    fn read_command(&self, buffer : &mut String){
            //read user input
            print!("> ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(buffer).expect("Failed to read characters");
    }

    pub fn exec(&self){
        loop{
            let mut buffer = String::new();

            self.read_command(&mut buffer);

            self.lexer.getTokens(buffer);
        }
    }

}