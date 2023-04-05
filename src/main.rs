mod collections;
mod interpreter;

use interpreter::base::Interpreter;

fn main() {

    let mut itr = Interpreter::new();
    itr.exec();

}
