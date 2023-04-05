use std::fmt;


#[derive(Clone)]
pub struct Node<T>{
    pub value : T,
    pub children : Vec<Node<T>>
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

impl<T: fmt::Display> Node<T>{
    fn get_tree_string(&self, prefix: &str, is_last: bool) -> String {
        let mut tree_str = String::new();
        
        if is_last {
            tree_str += &format!("{}{}", prefix, "└── ");
        } else {
            tree_str += &format!("{}{}", prefix, "├── ");
        }
        
        tree_str += &format!("{}\n", self.value);

        let num_children = self.children.len();
        for (i, child) in self.children.iter().enumerate() {
            let child_prefix = if is_last {
                format!("{}{}", prefix, "    ")
            } else {
                format!("{}{}", prefix, "│   ")
            };
            
            if i == num_children - 1 {
                tree_str += &child.get_tree_string(&child_prefix, true);
            } else {
                tree_str += &child.get_tree_string(&child_prefix, false);
            }
        }

        return tree_str;
    }

    pub fn to_string(&self) -> String {
        return self.get_tree_string("", true);
    }

}


pub struct Tree<T>{
    pub root : Option<Node<T>>
}

impl<T: fmt::Display> Tree<T>{
    pub fn new() -> Tree<T>{
        return Tree::<T>{root : None};
    }

    pub fn to_string(&self) -> String {
        match &self.root {
            Some(node) => return node.to_string(),
            None => return String::new(),
        }
    }
}
