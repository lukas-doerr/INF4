use std::env;
use std::fs;


fn main() {
    use std::time::Instant;
    let now = Instant::now();   
    let args: Vec<String> = env::args().collect();

    let file_path = args[1].to_string();
    println!("In file {}", file_path);

    let mut contents: Vec<i32> = read_seq(&file_path);
    let mut c = contents.clone();
    println!("Numbers:{:?}\n", contents);
    let n: usize = contents.len();
    println!("----------------- N={n} --------------------");
    

    let t1 = now.elapsed();
    //split(&mut contents, 0, N-1, "Start");
    merge_sort(&mut contents);
    let t2 = now.elapsed();
    
    println!("\n\tMerge-Sort-Time: {:?}", t2-t1);
    //println!("{:?}", contents);

    let t1 = now.elapsed();
    split(&mut c, 0, n-1);
    let t2 = now.elapsed();
    
    println!("\n\tSplit-Time: {:?}", t2-t1);
    //println!("{:?}", c);
}


fn read_seq(file_path: &String) -> Vec<i32>{
    let mut contents: Vec<i32> = Vec::new();
    let data = fs::read_to_string(file_path).unwrap();

    for d in data.lines() {
        contents.push(d.parse::<i32>().unwrap());
    }

    return contents;
}

fn split(z: &mut Vec<i32>, l: usize, r: usize) {
    static mut CNT: i32 = 0;

    unsafe { 
        println!("Cnt: {CNT}");
        CNT+=1; 
    }


    if l == r {
        return;
    }

    let m = (r+l)/2;      
    split(z, l, m);
    split(z, m+1, r);
    merge(z, l, m, r);
}

fn merge(z: &mut Vec<i32>, l: usize, m: usize, r: usize) {
    let left: Vec<i32> = z[l..m+1].to_vec();
    let right: Vec<i32> = z[m+1..r+1].to_vec();

    //println!("{l}");
    //println!("Left: {:?}\tRight: {:?}", left, right);

    let (mut i, mut j , mut k) = (0, 0, l);

    while i < left.len() && j < right.len() {
        if left[i] <= right[j]
        {
            //print!("L ");
            z[k] = left[i];
            i+=1;
        }
        else {
            //print!("R ");
            z[k] = right[j];
            j+=1;
        }
        k+=1;
    }

    while i < left.len() {
        //print!("LL ");
        z[k] = left[i];
        i+=1;
        k+=1;
    }
 
    while j < right.len() {
        //print!("RR ");
        z[k] = right[j];
        j+=1;
        k+=1;
    }



}


fn merge_sort(z: &mut Vec<i32>) -> Vec<i32>{

    let len = z.len();
    let m = len/2;


    if len == 1{
        //println!("Val: {:?}", z);
        return vec![z[0]];
    } 


    
    let lsort = merge_sort(&mut z[0..m].to_vec());
    let rsort = merge_sort(&mut z[m..len].to_vec());

    let (mut i, mut j, mut k) = (0, 0, 0);
    
    // Sortieren
    while i < lsort.len() && j < rsort.len() 
    {
        if lsort[i] <= rsort[j] 
        {
            z[k] = lsort[i];
            i+=1;
        }
        else
        {
            z[k] = rsort[j];
            j+=1;    
        }
        k+=1;
    }

    while i < lsort.len()
    {
        z[k] = lsort[i];
        i+=1;
        k+=1;
    }

    while j < rsort.len()
    {
        z[k] = rsort[j];
        j+=1;
        k+=1;
    }

    return z.to_vec();
}
