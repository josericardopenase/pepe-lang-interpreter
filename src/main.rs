mod collections;
mod interpreter;

use interpreter::base::Interpreter;

fn main() {

    let itr = Interpreter::new();

    itr.exec();



}
