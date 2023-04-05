use std::fmt;

pub struct Node<T>{
    value : T,
    children : Vec<Node<T>>
}

impl<T: fmt::Display> Node<T>{
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

    fn get_tree_string(&self, prefix: &str) -> String {
        let mut tree_str = format!("{}{}\n", prefix, self.value);
        let child_prefix = format!("{}{}", prefix, "│   ");
        let last_child_prefix = format!("{}{}", prefix, "└── ");

        let num_children = self.children.len();
        for (i, child) in self.children.iter().enumerate() {
            let child_prefix = if i == num_children - 1 {
                last_child_prefix.clone()
            } else {
                child_prefix.clone()
            };
            tree_str += &format!("{}{}", child_prefix, child.get_tree_string(&prefix.replace("└──", "    ")));
        }

        return tree_str;
    }

    pub fn to_string(&self) -> String {
        return self.get_tree_string("");
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
