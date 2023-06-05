use std::collections::VecDeque;
use std::fs;

pub struct BinTreeElement {
    key: i32,
    left: Option<Box<BinTreeElement>>,
    right: Option<Box<BinTreeElement>>,
}

pub trait Tree<'a> {
    fn new() -> Self;
    fn traverse(&self, x: &'a Option<Box<BinTreeElement>>);
    fn search(&self, start: &'a Option<Box<BinTreeElement>>, key: i32) -> Option<&'a BinTreeElement>;
    fn insert(&mut self, key: i32);
    fn walklevel(&self, x: &'a Option<Box<BinTreeElement>>);
    fn successor(&self, start: &'a Option<Box<BinTreeElement>>, key: i32) -> Option<&'a BinTreeElement>; 
}
pub struct BinSearchTree {
    root: Option<Box<BinTreeElement>>,
}

impl<'a> Tree<'a> for BinSearchTree {
    fn new() -> Self {
        Self { root: None }
    }

    fn walklevel(&self, x: &'a Option<Box<BinTreeElement>>) {
        if let Some(root) = x {
            let mut queue: VecDeque<&Box<BinTreeElement>> = VecDeque::new();
            queue.push_back(root);
            
            while !queue.is_empty() {
                let node = queue.pop_front().unwrap();
                println!("{:?}", node.key);

                if let Some(left) = &node.left {
                        queue.push_back(left);
                }


                if let Some(right) = &node.right {
                    queue.push_back(right);
                }


            }

                        
        }
    }



    

    fn successor(&self, start: &'a Option<Box<BinTreeElement>>, key: i32) -> Option<&'a BinTreeElement> {
        if let Some(node) = start {
            if key == node.key {
                if let Some(right) = &node.right {
                    println!("{}", right.key);
                    return Some(right);
                }
                println!("Kein Nachfolger!");
            } else if key < node.key {
                return self.successor(&node.left, key);
            } else {
                return self.successor(&node.right, key);
            }
        }
        None
    }

    fn traverse(&self, x: &'a Option<Box<BinTreeElement>>) {
        if let Some(node) = x {
            self.traverse(&node.left);
            println!("{:?}", node.key);
            self.traverse(&node.right);
        }
    }

    fn search(&self, start: &'a Option<Box<BinTreeElement>>, key: i32) -> Option<&'a BinTreeElement> {
        if let Some(node) = start {
            if key == node.key {
               return Some(node);
            } else if key < node.key {
                return self.search(&node.left, key);
            } else {
                return self.search(&node.right, key);
            }
        }
        None
    }

    fn insert(&mut self, key: i32) {
        if self.root.is_none() {
            self.root = Some(Box::new(BinTreeElement {
                key,
                left: None,
                right: None,
            }));
            return;
        }

        let mut current = &mut self.root;
        loop {
            let node = current.as_mut().unwrap();
            if key < node.key {
                if node.left.is_none() {
                    node.left = Some(Box::new(BinTreeElement {
                        key,
                        left: None,
                        right: None,
                    }));
                    return;
                } else {
                    current = &mut node.left;
                }
            } else {
                if node.right.is_none() {
                    node.right = Some(Box::new(BinTreeElement {
                        key,
                        left: None,
                        right: None,
                    }));
                    return;
                } else {
                    current = &mut node.right;
                }
            }
        }
    }
}

fn main() {
    // Create a new binary search tree
    use std::time::Instant;

    let timer = Instant::now();

    // Go through all Sequences
    for i in 0..1
    {
        // read Data from Sequence
        let mut bst = BinSearchTree::new();
        

   
        let file_path = "../../Files/seq".to_owned() + &i.to_string() + ".txt";
        println!("{file_path}");
        let data = fs::read_to_string(file_path).unwrap();

        for m in [6, 5, 7, 2, 5, 8]
        //for d in data.lines()
        {
            //let m = d.parse::<i32>().unwrap();
            bst.insert(m);
            println!("{:?}", m); 
        }

        println!("Numbers from Seq{i}");

        bst.successor(&bst.root, 8);
       // println!("{:?}", m.unwrap().key); 
        // Traverse and print the elements of the tree
        // bst.traverse(&bst.root);
    }

}

