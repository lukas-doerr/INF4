use std::fs;


pub struct Node {
    n: usize,              // Current number of keys
    keys: Vec<i32>,      // Keys of the node
    childs: Vec<Box<Node>>, // Child nodes
    is_leaf: bool,       // Indicates if it's a leaf node
}

pub trait Tree {
    fn new(_m: usize) -> Self;
    fn make_node(&self) -> Box<Node>;
    fn search(&self, node: &Box<Node>, key: i32) -> Option<&Box<Node>>;
    fn insert(&mut self, key: i32);
    fn split_child(&mut self, node: &Box<Node>, i: usize); 
    fn walk(&self, x: &Node); 
    fn insert_in_node(&mut self, node: &Box<Node>, key:i32); 
    fn height_root(&self, node: &Node, h: usize) -> usize;
    fn height(&self) -> usize;
    //fn traverse_inorder(&self)   ;
}

pub struct BTree {
    root: Option<Box<Node>>,
    m: usize,             // Upper Limit of nodes
}

impl Tree for BTree {
    fn new(_m: usize) -> Self {
        return Self { root: None,
                        m: _m
        };
    }

    fn make_node(&self) -> Box<Node> {
        Box::new(Node {
            n: 0,
            keys: vec![],
            childs: vec![],
            is_leaf: true 
        })
    }
    
    fn search(&self, node: &Box<Node>, key: i32) -> Option<&Box<Node>> {
        let mut i: usize = 0;
        
        
        while i < node.n && key > node.keys[i]
        {
            i = i + 1;
        }
        if i < node.n && key == node.keys[i]
        {
            return Some(node);
        }
        if node.is_leaf
        {
            return None;
        }
        return self.search(&node.childs[i], key);
    }

    fn insert(&mut self, key: i32) {
        if let Some(r) = self.root {
            if r.n == 2 * self.m - 1 
            {
                let h = self.make_node();
                self.root = Some(h);
                h.is_leaf = false;
                h.n = 0;
                h.childs[0] = r;
                self.split_child(&h, 0);
                self.insert_in_node(&h, key)
            }
            else
            {
                self.insert_in_node(&r, key);
            }
            
            let mut root = self.make_node();
            root.keys[0] = key;
            root.n = 1;
            self.root = Some(root);
        }
    }

    fn insert_in_node(&mut self, node: &Box<Node>, key:i32) {
        let mut i: usize = node.n;

        if node.is_leaf {
            while i > 1 && key < node.keys[i-1] 
            {
                node.keys[i] = node.keys[i-1];
                i = i - 1;
            }
            node.keys[i] = key;
            node.n = node.n + 1;
        }
        else {
            let mut j: usize = 0;
            while j < node.n && key > node.keys[j] 
            {
                j = j + 1;
                if node.childs[j].n == 2*self.m - 1 {
                    self.split_child(node, j);
                    if key > node.keys[j] {
                        j = j + 1;
                    }
                }
                self.insert_in_node(&node.childs[j], key);
            }
        }
    }

    fn walk(&self, node: &Node) {
        for i in 0..node.n 
        {
            if !node.is_leaf {
                self.walk(&node.childs[i as usize]);
            }
            println!("{}", node.keys[i as usize]); 
        }   
     }

      

    fn height(&self) -> usize {
        if let Some(root) = &self.root {
            return self.height_root(&root, 0);
        }
        else {
            return 0;
        }
    }

    fn height_root(&self, node: &Node, h: usize) -> usize
    {
        let mut max_child_height = h;

        if node.is_leaf {
            return h + 1;
        }
        
        for child in &node.childs 
        {
            let child_height = self.height_root(child, h + 1);
            if child_height > max_child_height {
            	max_child_height = child_height;
            }
            
        }
        return max_child_height;
        
       
    }

    fn split_child(&mut self, node: &Box<Node>, i: usize) {
        let k = &node.childs[i];
        let mut h = self.make_node();
        h.is_leaf = k.is_leaf;
        h.n = self.m - 1;

        for j in 0..self.m-1
        {
            h.keys[j] = k.keys[j + self.m];
        }

        if !h.is_leaf
        {
            for j in 0..self.m
            {
                h.childs[j] = Box::new(k.childs[j + self.m);
            }      
        }

        k.n = self.m - 1;

        for j in i+1..node.n 
        {
            node.childs[j+1] = node.childs[j];
        }

        node.childs[i+1] = h;

        for j in i+1..node.n 
        {
            node.keys[j] = node.keys[j-1];
        }

        node.keys[i] = k.keys[self.m - 1];
        node.n = node.n + 1;
    }
}


fn main() {
    let mut bst = BTree::new(3);

    // Insert some keys
    bst.insert(10);
    bst.insert(20);
    bst.insert(30);
    bst.insert(40);
    bst.insert(50);
    bst.insert(60);
    bst.insert(70);
    bst.insert(80);
    bst.insert(90);
    bst.insert(100);
    bst.insert(110);
    bst.insert(120);

    // Print the tree height
    let height = bst.height();
    println!("Tree height: {}", height);

    // Print the keys in the tree using inorder traversal
    bst.walk(bst.root.as_ref().unwrap());

    // Search for a key
    let key = 50;
    let result = bst.search(bst.root.as_ref().unwrap(), key);
    match result {
        Some(node) => println!("Key {} found in the tree!", key),
        None => println!("Key {} not found in the tree.", key),
    }
}


/*fn wakj() {
    // Create a new binary search tree
    use std::time::Instant;
  
    let timer = Instant::now();
    // Go through all Sequences
    for i in 0..1
    {
            // read Data from Sequence
A>      let mut bst = BinSearchTree::new();
  
  
  
        let file_path = "../../Files/seq".to_owned() + &i.to_string() + ".txt";
        println!("{file_path}");
        let data = fs::read_to_string(file_path).unwrap();
  
        //for m in [6, 5, 7, 2, 5, 8]
        for d in data.lines()
        {
            let m = d.parse::<i32>().unwrap();
            bst.insert(m);
            println!("{:?}", m);
        }

        println!("Numbers from Seq{i}");

        bst.successor(&bst.root, 14);
    }
}
*/

