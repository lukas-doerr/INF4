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
        if let Some(root) = &self.root {
            self.search_node(root, key)
        } else {
            None
        }
    }

    fn search_node(&self, node: &Node, key: i32) -> Option<&Node> {
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
        if let Some(root) = &self.root {
            if root.keys.len() == 2 * self.m - 1 {
                let mut new_root = self.make_node();
                root = &mut new_root;
                self.root = Some(new_root);
                self.split_child(&mut self.root.as_mut().unwrap(), 0);
            }
            self.insert_in_node(root, key);
        } else {
            let mut new_root = self.make_node();
            self.root = Some(new_root);
            let root = self.root.as_mut().unwrap();
            root.is_leaf = true;
            root.keys.push(key);
            root.childs.push(self.make_node());
        }
    }


    fn insert_in_node(&mut self, node: &mut Box<Node>, key: i32) {
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
            self.insert_in_node(child, key);
        }
    }

    fn split_child(&mut self, node: &mut Box<Node>, i: usize) {
        let mid = self.m - 1;
        let mut new_node = self.make_node();
        let mut child = node.childs[i].take();
        new_node.is_leaf = child.is_leaf;
        new_node.keys = child.keys.split_off(mid + 1);
        if !new_node.is_leaf {
            new_node.childs = child.childs.split_off(mid + 1);
        }
        node.keys.insert(i, child.keys.pop().unwrap());
        node.childs.insert(i + 1, new_node);
    }

    fn walk(&self, node: &Node) {
        for i in 0..node.keys.len() {
            if !node.is_leaf {
                self.walk(&node.childs[i]);
            }
            println!("{}", node.keys[i]);
        }
        if !node.is_leaf {
            self.walk(&node.childs[node.keys.len()]);
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

    tree.walk(tree.root.unwrap());
    println!("Height: {}", tree.height_root());
}

