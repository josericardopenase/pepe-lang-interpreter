mod collections;
mod interpreter;

use std::vec;

use collections::tree::{Tree, Node};
use interpreter::base::Interpreter;

fn main() {

    let mut itr = Interpreter::new();
    itr.exec();
}
