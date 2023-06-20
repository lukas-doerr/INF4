struct BTree {
    m: usize,
    root: Option<Box<Node>>,
}

struct Node {
    is_leaf: bool,
    keys: Vec<i32>,
    childs: Vec<Box<Node>>,
}

impl BTree {
    fn new(m: usize) -> Self {
        Self { m, root: None }
    }

    fn make_node(&self) -> Box<Node> {
        Box::new(Node {
            is_leaf: false,
            keys: Vec::new(),
            childs: Vec::new(),
        })
    }

    fn search(&self, key: i32) -> Option<&Node> {
        self.root.as_ref().and_then(|root| self.search_node(root, key))
    }

    fn search_node<'a>(&'a self, node: &'a Node, key: i32) -> Option<&Node> {
        let mut i = 0;
        while i < node.keys.len() && key > node.keys[i] {
            i += 1;
        }
        if i < node.keys.len() && key == node.keys[i] {
            Some(node)
        } else if node.is_leaf {
            None
        } else {
            self.search_node(&node.childs[i], key)
        }
    }

    fn insert(&mut self, key: i32) {
        let mut new_root = None;
        if let Some(mut root) = self.root.take() {
            if root.keys.len() == 2 * self.m - 1 {
                new_root = Some(self.make_node());
                std::mem::swap(&mut new_root.as_mut().unwrap().keys, &mut root.keys);
                std::mem::swap(&mut new_root.as_mut().unwrap().childs, &mut root.childs);
                self.split_child(new_root.as_mut().unwrap(), 0);
            }
            self.insert_in_node(&mut root, key);
            self.root = Some(root);
        } else {
            new_root = Some(self.make_node());
            new_root.as_mut().unwrap().is_leaf = true;
            new_root.as_mut().unwrap().keys.push(key);
            new_root.as_mut().unwrap().childs.push(self.make_node());
            self.root = new_root;
        }
    }

    fn insert_in_node(&mut self, node: &mut Node, key: i32) {
        let mut i = node.keys.len();
        if node.is_leaf {
            node.keys.push(key);
            node.keys.sort();
        } else {
            while i > 0 && key < node.keys[i - 1] {
                i -= 1;
            }
            let child = &mut node.childs[i];
            if child.keys.len() == 2 * self.m - 1 {
                self.split_child(child, i);
                if key > node.keys[i] {
                    i += 1;
                }
            }
            self.insert_in_node(child.as_mut(), key);
        }
    }

    fn split_child(&mut self, node: &mut Node, i: usize) {
        let mid = self.m - 1;
        let mut new_node = self.make_node();
        let mut child = node.childs[i].take();
        new_node.is_leaf = child.is_leaf;
        new_node.keys = child.keys.split_off(mid + 1);
        if !new_node.is_leaf {
            new_node.childs = child.childs.split_off(mid + 1);
        }
        node.keys.insert(i, child.keys.pop().unwrap());
        node.childs.insert(i + 1, Box::new(new_node));
    }

    fn walk(&self) {
        if let Some(root) = &self.root {
            self.walk_node(root);
        }
    }

    fn walk_node(&self, node: &Node) {
        for i in 0..node.keys.len() {
            if !node.is_leaf {
                self.walk_node(&node.childs[i]);
            }
            println!("{}", node.keys[i]);
        }
        if !node.is_leaf {
            self.walk_node(&node.childs[node.keys.len()]);
        }
    }

    fn height_root(&self) -> i32 {
        if let Some(root) = &self.root {
            self.height(root)
        } else {
            0
        }
    }

    fn height(&self, node: &Node) -> i32 {
        if node.is_leaf {
            0
        } else {
            self.height(&node.childs[0]) + 1
        }
    }
}

fn main() {
    let mut tree = BTree::new(3);
    tree.insert(7);
    tree.insert(3);
    tree.insert(9);
    tree.insert(1);
    tree.insert(6);
    tree.insert(4);
    tree.insert(2);
    tree.insert(8);
    tree.insert(5);

    tree.walk();
    println!("Height: {}", tree.height_root());
}


