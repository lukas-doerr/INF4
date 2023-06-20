struct HashTable {
    keys: Vec<State>,
    m: usize,
}

#[derive(PartialEq, Copy, Clone)]
enum State {
    None,
    Some(i32),
    IsEmpty,
}

pub trait Table {
    fn new(_m: usize) -> Self;
    fn hash(&self, key: i32) -> usize;
    fn f(&self, h: usize, i: usize) -> usize;
    fn insert(&mut self, key: i32) -> bool;
    fn search(&self, key: i32) -> bool;
    fn delete(&mut self, key: i32);
    fn dump(&self);
    fn alpha(&self) -> usize;
}

impl Table for HashTable {
    fn new(_m: usize) -> Self {
        Self {
            keys: vec![State::None; _m],
            m: _m,
        }
    }

    fn hash(&self, key: i32) -> usize {
        let A: f32 = (f32::sqrt(5.0) - 1.0) / 2.0;
        let val = ((key as f32 * A - (key as f32 * A).floor()) * self.m as f32).floor();
        val as usize
    }

    fn f(&self, h: usize, i: usize) -> usize {
        let val = i % 2;

        if val == 1 {
            (((h as i64 - i.pow(2) as i64) % self.m as i64 + self.m as i64) % self.m as i64) as usize
        } else {
            ((h as i64 + i.pow(2) as i64) % self.m as i64) as usize
        }
    }

    fn insert(&mut self, key: i32) -> bool {
        let h = self.hash(key);
        let mut i = 0;
        
        loop {
            let fh = self.f(h, i);
            if self.keys[fh] == State::None || self.keys[fh] == State::IsEmpty {
                self.keys[fh] = State::Some(key);
                return true
            }

            if i==self.m {return false}

            i = i + 1;
        }    
    }

    fn search(&self, key: i32) -> bool {
        let h = self.hash(key);
        let mut i = 0;

        loop {
            let fh = self.f(h, i);
            match self.keys[fh] {
                State::None => return false,
                State::IsEmpty => {i = i + 1;},
                State::Some(x) => if key == x {
                                    println!("Found Key!");
                                    return true
                                 } else {
                                     i = i + 1;
                                 }
            }

            if i==self.m {return false}
        }
    }

    fn delete(&mut self, key: i32) {
        let h = self.hash(key);
        let mut i = 0;

        loop {
            let fh = self.f(h, i);
            match self.keys[fh] {
                State::None => {println!("Not Found!"); break;},
                State::IsEmpty => {i = i + 1;},
                State::Some(x) => if key == x {
                            println!("Delete Key!");
                            self.keys[h] = State::IsEmpty;
                        } else {
                            
                            i = i + 1;
                        }
            }

            if i==self.m {break;}
        }
    }

    fn dump(&self) {
        for val in &self.keys {
            match val {
                State::Some(v) => println!("{}", v),
                State::None => println!("Empty"),
                State::IsEmpty => println!("Deleted"),
            }
        }
    }

    fn alpha(&self) -> usize {
        self.keys
            .iter()
            .filter(|&val| *val != State::None && *val != State::IsEmpty)
            .count()
            / self.m
    }
}

fn main() {
    // Create a new binary search tree
    use std::time::Instant;
    use std::fs;

    let timer = Instant::now();

    // Go through all Sequences
    for i in 0..2
    {
        // read Data from Sequence
        let mut ht = HashTable::new(31);
        

   
        let file_path = "../../Files/seq".to_owned() + &i.to_string() + ".txt";
        println!("{file_path}");
        let data = fs::read_to_string(file_path).unwrap();

        //for m in [6, 5, 7, 2, 5, 8]
        for d in data.lines()
        {
            let m = d.parse::<i32>().unwrap();
            ht.insert(m);
            // println!("{:?}", m); 
        }

        println!("Numbers from Seq{i}");

        ht.search(14);
        ht.delete(14);
        // println!("{:?}", m.unwrap().key); 
        // Traverse and print the elements of the tree
        ht.dump();
    }

}

