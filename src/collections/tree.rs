pub struct Node<T>{
    value : T,
    children : Vec<Node<T>>
}

impl<T> Node<T>{
    pub fn new(value : T) -> Node<T>
    {
        return Node{
            value: value, 
            children: vec![]
        }
    }

    pub fn add_node(&mut self, curr : Node<T>) -> () {
        self.children.push(curr);
    }

    pub fn remove_node(&mut self, index : usize) -> (){
        self.children.remove(index);
    }
}

pub struct Tree<T>{
    pub root : Node<T>
}

impl<T> Tree<T>{
    pub fn new(&mut self, root : Node<T>){
        self.root = root;
    }
}