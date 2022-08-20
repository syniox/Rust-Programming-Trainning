use text_io::read;
use std::fmt;

struct Node {
    v: i32,
    l: Option<Box<Node>>,
    r: Option<Box<Node>>
}

#[derive(Debug)]
enum Path {
    Left, Right
}

enum Error {
    DuplicateValue,
    NotFound,
    EmptyTree
}

struct Tree {
    rt: Option<Box<Node>>
}

impl Node{
    fn new(v: i32) -> Node {
        Node { v: v, l: None, r: None }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match *self {
            Self::DuplicateValue => "Duplicate value",
            Self::NotFound => "Not found",
            Self::EmptyTree => "Empty tree"
        })
    }
}

impl Tree{
    fn new() -> Tree {
        Tree { rt: None }
    }

    fn insert_tr(node: &mut Option<Box<Node>>, v: i32) -> Result<(), Error> {
        let node = match node.as_mut() {
            None => {
                *node = Some(Box::new(Node::new(v)));
                return Ok(());
            }
            Some(n) => n
        };
        if node.v == v {
            return Err(Error::DuplicateValue);
        }
        Self::insert_tr(
            if node.v > v { &mut node.l } else { &mut node.r },
            v)
    }
    fn print_tr(node: &Option<Box<Node>>, values: &mut Vec<i32>) {
        let node = match node.as_ref() {
            None => return,
            Some(n) => n
        };
        Self::print_tr(&node.l, values);
        values.push(node.v);
        Self::print_tr(&node.r, values);
    }
    fn find_tr(node: &Option<Box<Node>>, v: i32, path: &mut Vec<Path>) -> Result<(), Error> {
        let node = match node.as_ref() {
            None => return Err(Error::NotFound),
            Some(n) => n
        };
        if node.v == v {
            Ok(())
        } else if node.v > v {
            path.push(Path::Left);
            Self::find_tr(&node.l, v, path)
        } else {
            path.push(Path::Right);
            Self::find_tr(&node.r, v, path)
        }
    }

    fn insert(&mut self) -> Result<(), Error> {
        let v: i32 = read!();
        Self::insert_tr(&mut self.rt, v)
    }
    fn print(&self) -> Result<(), Error> {
        let mut values = Vec::<i32>::new();
        Self::print_tr(&self.rt, &mut values);
        println!("Values: {:?}", values);
        Ok(())
    }
    fn find(&self) -> Result<(), Error> {
        let v: i32 = read!();
        let mut path = Vec::<Path>::new();
        if let Err(e) = Self::find_tr(&self.rt, v, &mut path) {
            Err(e)
        } else {
            println!("Path: {:?}", path);
            Ok(())
        }
    }
    fn left(&mut self) -> Result<(), Error> {
        self.rt = match self.rt.as_mut() {
            None => return Err(Error::EmptyTree),
            Some(n) => n.l.take()
        };
        Ok(())
    }
}

fn main() {
    let n: i32 = read!();
    let mut tree = Tree::new();
    for _ in 0..n {
        let c: char = read!("{}");
        if let Err(e) = match c {
            'I' => tree.insert(),
            'P' => tree.print(),
            'F' => tree.find(),
            'L' => tree.left(),
            _ => panic!()
        } {
            println!("Error: {}", e);
        }
    }
}
