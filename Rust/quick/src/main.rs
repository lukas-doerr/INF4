use std::fs;
use std::env;

fn quicksort(z: &mut Vec<i8>, p: usize, r: usize, m: usize)
{
    if p < r
    { 
        let q: usize;
        if m==0
        {
            q = partition(z, p, r) as usize;
        }
        else 
        {
            q = fastpartition(z, p, r) as usize;
        }
        
        if q<1 {return;}

        quicksort(z, p, q-1, m);
        quicksort(z, q+1, r, m);
    }
}



fn vglpartion(z: &mut Vec<i8>, l: usize, r: usize) -> usize
{
    let m = (l + r) / 2;
    return m;
}




fn fastpartition(z: &mut Vec<i8>, l: usize, r: usize) -> i64
{
    let m = (l + r) / 2;
    
    println!("L({l})\tM({m})\tR({r})"); 
    println!("L({})\tM({})\tR({})\n", z[l], z[m], z[r]); 
    
    // Left Element is bigger than middle
    if z[m] < z[l]
    {
        // Move left Element to middle
        println!("Move Left to Middle"); 
        println!("L({l}):{}\tM({m}):{}\tR({r}):{}", z[l], z[m], z[r]); 
        swap(z, m, l);
        
        println!("L({l}):{}\tM({m}):{}\tR({r}):{}\n", z[l], z[m], z[r]); 
       
    }

    // Left is bigger than right 
    if z[r] < z[l]
    {
        // Move left Elemnet to right 
        println!("Move Left to Right"); 
        println!("L({l}):{}\tM({m}):{}\tR({r}):{}", z[l], z[m], z[r]); 
        swap(z, r, l);
        
        println!("L({l}):{}\tM({m}):{}\tR({r}):{}\n", z[l], z[m], z[r]); 
    }

    // Middle is bigger than right
    if z[r] < z[m]
    {
        // Move middle to right
        println!("Move Middle to Right"); 
        println!("L({l}):{}\tM({m}):{}\tR({r}):{}", z[l], z[m], z[r]); 
        swap(z, r, m);
        println!("L({l}):{}\tM({m}):{}\tR({r}):{}\n", z[l], z[m], z[r]); 
    }
    println!("\n\n");

    let r1 = r - 1;

    swap(z, m, r1);

    return partition(z, l, r1);
}


fn partition(z: &mut Vec<i8>, p: usize, r: usize) -> i64
{
    let pivot = z[r];
    let mut i: i64 = p as i64 - 1;

    for j in p..r
    {
        if z[j] <= pivot
        {
            i = i+1;
            swap(z, i as usize, j);

        }
    }
    swap(z, (i+1) as usize, r);
    return i+1;
}


fn swap(z: &mut Vec<i8>, i: usize, j: usize)
{
    let tmp = z[i];
    z[i] = z[j];
    z[j] = tmp;
}


fn read_seq(i: i8) -> Vec<i8> 
{
    let file_path = "../../../Rust/Files/seq".to_owned() + &i.to_string() + ".txt";
    let mut content: Vec<i8> = Vec::new();
    let data = fs::read_to_string(file_path).unwrap();

    for d in data.lines()
    {
        content.push(d.parse::<i8>().unwrap());
    }

    return content;
}


fn main() {
    use std::time::Instant;

    let timer = Instant::now();
    let mut times: Vec<std::time::Duration> = Vec::new();
   
    let args: Vec<String> = env::args().collect();
    let s = args[1].to_string().parse::<usize>().unwrap();
        

    // Go through all Sequences
    for i in 0..1
    {
        // read Data from Sequence
        let mut content: Vec<i8> =  read_seq(i);
        let n = content.len();
        
        println!("Numbers from Seq{i}");
        println!("--------------- N = {n} ---------------");
        if i==0 {println!("Numbers:\t {:?}", content);}

        // Start algorithm
        let start = timer.elapsed();
        quicksort(&mut content, 0, n-1, s);
        let end  = timer.elapsed();

        // Store needed time for analytics
        times.push(end-start);

        // print time
        if i==0 {println!("Sorted Numbers:  {:?}\n", content);}
        println!("Time for N({n}): {:.2?}\n", end-start); 
    }  

}
