use std::fs;

fn quicksort(z: &mut Vec<i8>, p: usize, r: usize)
{
    if p < r
    { 
        let q: usize = partition(z, p, r);
        quicksort(z, p, q-1);
        quicksort(z, q+1, r);
    }
}

fn partition(z: &mut Vec<i8>, p: usize, r: usize) -> usize
{   
    let pivot = z[r];
    let mut i = p - 1;

    for j in p..r
    {
        if z[j] <= pivot
        {
            i = i+1;
            swap(z, i, j);

        }
    }
    swap(z, i+1, r);
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
    
    // Go through all Sequences
    for i in 0..4
    {
        // read Data from Sequence
        let mut content: Vec<i8> =  read_seq(i);
        let n = content.len();
        
        println!("Numbers from Seq{i}");
        println!("--------------- N = {n} ---------------");
        if i==0 {println!("Numbers:\t {:?}", content);}

        // Start algorithm
        let start = timer.elapsed();
        quicksort(&mut content, 1, n-1);
        let end  = timer.elapsed();

        // Store needed time for analytics
        times.push(end-start);

        // print time
        if i==0 {println!("Sorted Numbers:  {:?}\n", content);}
        println!("Time for N({n}): {:.2?}\n", end-start); 
    }  

}
