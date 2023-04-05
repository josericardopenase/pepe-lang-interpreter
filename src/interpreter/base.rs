use std::io::{self, Read, Write};
use super::{parser::base::Parser, lexer::base::Lexer};
use colored::Colorize;


/**
 * Interpreter():
 * 
 * base class for interpretting user commands.
 * Then it will read a file and interpret every single command.
 * 
 */
pub struct Interpreter{
    lexer : Lexer,
    parser: Parser
}

impl Interpreter{
    pub fn new() -> Interpreter{
        return Interpreter{
            lexer: Lexer::new(),
            parser: Parser::new()
        };
    }

    fn read_command(&self, buffer : &mut String){
            //read user input
            print!(">>> ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(buffer).expect("Failed to read characters");
    }

    pub fn exec(&mut self){

        println!("Welcome to {} v{}", "pepescript".green().bold(), "0.0.1".bold());
        println!("Type {} for more information", ".help".bold());


        loop{
            let mut buffer = String::new();

            self.read_command(&mut buffer);

            self.lexer.get_tokens(buffer);
            self.lexer.to_string();
        }
    }

}