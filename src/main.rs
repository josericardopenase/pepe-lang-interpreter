mod lexer;

use std::io::{self, Read, Write};
use lexer::Lexer;

fn main() {

    loop {
        //creating reading buffer
        let mut line : String = String::new();

        //printing > after user input
        print!(" > ");
        io::stdout().flush().unwrap();
        //reading user input
        io::stdin().read_line(&mut line).expect("Failed to read string form user");

        let mut lexer : Lexer = Lexer::new(line);
        lexer.eval();
        for x in lexer.elements.iter(){
            print!("{}\n", x.to_string());
        }
    }

}
