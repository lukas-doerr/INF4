pub struct BinTreeElement {
    key: i32,
    left: Option<Box<BinTreeElement>>,
    right: Option<Box<BinTreeElement>>
}

pub trait Tree {
    fn new() -> Self;
    fn traverse(&self, f: &dyn Fn(i32));
    fn search(&mut self, key: i32) -> bool;
    fn insert(&mut self, key: i32);
}

pub struct BinSearchTree {
    root: Option<Box<BinTreeElement>>
}


fn main() {
    println!("Hello, world!");
}
