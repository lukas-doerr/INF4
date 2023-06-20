



struct HashTable {
    keys: Vec<Option<i32>>, // Verwendung von Option<i32> für leere Buckets
    m: usize,
}

pub trait Table {
    fn new(_m: usize) -> Self;
    fn hash(&self, key: i32) -> usize;
    fn f(&self, h: usize, i: usize);
    fn insert(&mut self, key: i32) -> bool; 
    fn search(&self, key: i32) -> bool;
    fn delete(&mut self, key: i32); 
    fn dump(&self);
    fn alpha(&self) -> usize;
}

impl Table for HashTable {
    fn new(_m: usize) -> Self {
        Self {
            keys: vec![None; _m], // Initialisierung der Buckets mit None
            m: _m,
        }
    }

    fn hash(&self, key: i32) -> usize {
        let A: f32 = (f32::sqrt(5.0) - 1.0) / 2.0;
        let val = ((key as f32 * A - (key as f32 * A).floor()) * self.m as f32).floor();
        val as usize
    }

    fn f(&self, h: usize, i: usize) -> usize {
        (h + i + 14 * i.pow(2))
    }

    fn insert(&mut self, key: i32) -> bool {
        let h = self.hash(key);
        if self.keys[h].is_none() {
            self.keys[h] = Some(key);
            true
        } else {
            loop {
                

            }
        }
    }

    fn search(&self, key: i32) -> bool {
        let h = self.hash(key);
        if let Some(val) = self.keys[h] {
            val == key
        } else {
            false
        }
    }

    fn delete(&mut self, key: i32) {
        let h = self.hash(key);
        self.keys[h] = None;
    }

    fn dump(&self) {
        for val in &self.keys {
            if let Some(v) = val {
                println!("{}", v);
            } else {
                println!("Empty");
            }
        }
    }

    fn alpha(&self) -> usize {
        self.keys.iter().filter(|&val| val.is_some()).count() / self.m
    }
}

fn main() {
    // Create a new binary search tree
    use std::time::Instant;
    use std::fs;

    let timer = Instant::now();

    // Go through all Sequences
    for i in 0..1
    {
        // read Data from Sequence
        let mut ht = HashTable::new(100);
        

   
        let file_path = "../../Files/seq".to_owned() + &i.to_string() + ".txt";
        println!("{file_path}");
        let data = fs::read_to_string(file_path).unwrap();

        //for m in [6, 5, 7, 2, 5, 8]
        for d in data.lines()
        {
            let m = d.parse::<i32>().unwrap();
            ht.insert(m);
            println!("{:?}", m); 
        }

        println!("Numbers from Seq{i}");

        ht.search(14);
        // println!("{:?}", m.unwrap().key); 
        // Traverse and print the elements of the tree
        ht.dump();
    }

}

