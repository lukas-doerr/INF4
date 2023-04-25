use std::env;
use std::fs;

fn main() {
    use std::time::Instant;
    let now = Instant::now();   
    let args: Vec<String> = env::args().collect();

    let file_path = args[1].to_string();
    println!("In file {}", file_path);

    let contents: Vec<i32> = read_seq(&file_path);
    println!("Numbers:{:?}\n", contents);
    let N = contents.len();
    println!("----------------- N={N} --------------------");

    // Kubische Maxfolge
    let el1 = now.elapsed();
    maxfolge_kubisch(&contents);
    let el2 = now.elapsed();
    println!("Elapsed: {:.2?}", el2-el1);

    // Teile-Hersche Maxfolge
    let el1 = now.elapsed();
    let r = contents.len() - 1;
    let (zwi_max, zwi_l, zwi_r) = maxfolge_teile_hersche(&contents, 0, r);
    println!("\n--> Teile\t Max: {zwi_max}, Links: {zwi_l}, Rechts: {zwi_r}");
    let el2 = now.elapsed();
    println!("Elapsed: {:.2?}", el2-el1);


    
    // Lineare Maxfolge
    read_linear(&file_path);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed-el2);

}

fn read_linear(file_path: &String) {
    let data = fs::read_to_string(file_path).unwrap();

    let mut max: f64 = -f64::INFINITY;
    let mut akt_sum = 0;
    let mut akt_links = 0;
    let mut links = 0;
    let mut rechts = 0;
    let mut i=0;

    for d in data.lines() {
        let val = d.parse::<i32>().unwrap();

        akt_sum = akt_sum + val;

        if akt_sum > max as i32 {
            max = akt_sum as f64;
            links = akt_links;
            rechts = i;
        }

        if akt_sum < 0 {
            akt_sum = 0;
            akt_links = i + 1;
        }
        i+=1;
    }

    println!("--> Linear\t Max: {max}, Links: {links}, Rechts: {rechts}");

}

fn maxfolge_teile_hersche(z: &Vec<i32>, l: usize, r: usize) -> (f64, usize, usize) {
    if l == r {
        return (z[l] as f64, l, r);
    }

    let m = (l+r)/2;

    let (links_max, links_l, links_r): (f64, usize, usize) = maxfolge_teile_hersche(z, l, m);
    let (rechts_max, rechts_l, rechts_r): (f64, usize, usize) = maxfolge_teile_hersche(z, m+1, r);
    let (zwi_max, zwi_l, zwi_r): (f64, usize, usize) = finde_zwischen(z, l, m, r);

    if links_max >= rechts_max && links_max >= zwi_max {
        return (links_max, links_l, links_r);
    }

    if rechts_max >= links_max && rechts_max >= zwi_max {
        return (rechts_max, rechts_l, rechts_r);
    }

    return (zwi_max, zwi_l, zwi_r);
}

fn finde_zwischen(z: &Vec<i32>, l: usize, m: usize, r: usize) -> (f64, usize, usize) {
    let mut links_max: f64 = -f64::INFINITY;
    let mut sum: i32 = 0;
    let mut links = 0;
    let mut rechts = 0;

    for i in (l..m+1).rev() {
        sum = sum + z[i];
        if sum > links_max as i32 {
            links_max = sum as f64;
            links = i;            
        }
    }
    
    let mut rechts_max: f64 = -f64::INFINITY;
    sum = 0;
    for i in (m+1)..r {
        sum = sum + z[i];

        if sum > rechts_max as i32 {
            rechts_max = sum as f64;
            rechts = i;            
        }

    }

    return (links_max + rechts_max, links, rechts);

}

fn maxfolge_kubisch(contents: &Vec<i32>) -> (f64, usize, usize) {
    let n = contents.len();
    let mut max: f64 = -f64::INFINITY;
    let mut links = 0;
    let mut rechts = 0;

    for i in 0..(n-1) {
        for j in i..(n-1) {
            let mut sum: i32 = 0;

            for k in i..j+1 {
                sum = sum + contents[k];
            }

            if sum > max as i32 {
                max = sum as f64;
                links = i;
                rechts = j;
            }
        }
    }
    

    println!("--> Kubisch\t Max: {max}, Links: {links}, Rechts: {rechts}");

    return (max, links, rechts);
}


fn read_seq(file_path: &String) -> Vec<i32>{
    let mut contents: Vec<i32> = Vec::new();
    let data = fs::read_to_string(file_path).unwrap();

    for d in data.lines() {
        contents.push(d.parse::<i32>().unwrap());
    }

    return contents;
}