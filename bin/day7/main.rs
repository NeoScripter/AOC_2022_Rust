use std::hash::{Hash, Hasher};
use std::{
    rc::{Rc, Weak},
    cell::RefCell,
};

#[derive(Debug, Clone)]
struct Node {
    id: String,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
    size: Rc<RefCell<u32>>,
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Node {
    fn new(id: String) -> Rc<Self> {
        Rc::new(Node {
            id,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
            size: Rc::new(RefCell::new(0)),
        })
    }

    fn add_child(parent: &Rc<Self>, id: String) {
        let child = Rc::new(Node {
            id,
            parent: RefCell::new(Rc::downgrade(parent)),
            children: RefCell::new(vec![]),
            size: Rc::new(RefCell::new(0)),
        });
        parent.children.borrow_mut().push(Rc::clone(&child));
    }
    fn get_parent(&self) -> Option<Rc<Node>> {
        self.parent.borrow().upgrade()
    }

    fn get_child(&self, id: String) -> Option<Rc<Node>> {
        for child in self.children.borrow().iter() {
            if child.id == id {
                return Some(Rc::clone(child));
            }
        }
        None
    }
    fn add_to_size(&self, additional_size: u32) {
        *self.size.borrow_mut() += additional_size;
    }
    fn define_sizes(&self) {
        *self.size.borrow_mut() += self.children.borrow_mut().iter().map(|child| {
            child.define_sizes();
            *child.size.borrow()
        }).sum::<u32>();
    }
    fn added_right_sizes(&self) -> u32 {
        let mut sum = 0;
        if *self.size.borrow() <= 100000 {
            sum += *self.size.borrow();
        }

        for child in self.children.borrow().iter() {
            sum += child.added_right_sizes();
        }

        sum
    }
    fn find_cnds(&self, space_to_free: u32) -> Vec<u32> {
        let mut cnds = Vec::new();
        if *self.size.borrow() >= space_to_free {
            cnds.push(*self.size.borrow());
        }
        for child in self.children.borrow().iter() {
            cnds.extend(child.find_cnds(space_to_free));
        }
        cnds
    }
    fn print_tree(&self, indent: usize) {
        let indent_str = " ".repeat(indent * 4);
        println!("{} node: {}, size: {}", indent_str, self.id, *self.size.borrow());

        for child in self.children.borrow().iter() {
            child.print_tree(indent + 1);
        }
    }
}

fn build_tree() -> Rc<Node> {
    let input = include_str!("input7.txt");
    let node = Node::new(String::from("/"));
    let mut cur_node = Rc::clone(&node); 

    for line in input.lines().skip(1) {
        if line.starts_with("dir") {
            let (_, name) = line.split_once(' ').unwrap();
            Node::add_child(&cur_node, name.to_string());
        } else if line.contains("..") {
            cur_node = cur_node.get_parent().unwrap();
        } else if line.contains("$ cd") {
            let (_, name) = line.rsplit_once(' ').unwrap();
            let new_child = cur_node.get_child(name.to_string()).unwrap();
            cur_node = new_child;
        } else if line.contains("$ ls") {
        } else {
            let (size, _) = line.split_once(' ').unwrap();
            let size = size.parse::<u32>().unwrap();
            Node::add_to_size(&cur_node, size);
        }
    }
    node
}
fn part1() -> u32 {
    let tree = build_tree();
    tree.define_sizes();
    //tree.print_tree(2);
    tree.added_right_sizes()
}

fn part2() -> u32 {
    let total_space = 70000000;
    let needed = 30000000;
    let tree = build_tree();
    tree.define_sizes();
    let cur_taken = *tree.size.borrow();
    let space_to_free = needed - (total_space - cur_taken);
    let cnds = tree.find_cnds(space_to_free);
    cnds.into_iter().min().unwrap()
}

fn main() {
    println!("{}", part2());
}