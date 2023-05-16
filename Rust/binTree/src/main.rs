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


impl Tree for BinSearchTree
{
    fn new() -> Self 
    {
        Self {root: None}
    }

    fn traverse(&self, f: &dyn Fn(i32))
    {
        if self.root.is_some()
        {
            self.traverse(self.root.unwrap().left);
            println!("{:?}", self.root.unwrap().key);
            self.traverse(self.root.unwrap().right);
        }
    }

    fn search(&mut self, _key: i32) -> bool
    {
        if (self.root.is_none() ) || (_key == self.root.unwrap().key) {return self}

        if _key < self.root.unwrap().key 
        {
            return self.search(self.root.unwrap().left, _key)
        }
        else 
        {
            return self.search(self.root.unwrap().right, _key)
        }
    }

    fn insert(&mut self, key: i32) 
    {
        let mut y = None;
        let mut x = self.root;

        while x.is_some() 
        {
            y = x.clone();

            if key < x.unwrap().key
            {
                x = x.unwrap()left;
            }
            else 
            {
                x = x.unwrap().right;
            }
        }

        if y.is_none() 
        {
            *self.root.unwrap() = BinTreeElement {key: key, right: None, left: None};
        }
        else if key < y.unwrap().key
        {
            y.unwrap().left = BinTreeElement {key: key, right: None, left: None};
        }
        else 
        {
            y.unwrap().right = BinTreeElement {key: key, right: None, left: None};
        }

    }

}

fn main() {
    println!("Hello, world!");
}
