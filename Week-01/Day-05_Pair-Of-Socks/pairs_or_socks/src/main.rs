use std::collections::HashMap;

fn sock_pairs(socks: &str) -> i32 {
   
    let mut counts: HashMap<char, i32> = HashMap::new();
    
    for c in socks.chars() {
        counts.entry(c)
            .and_modify(|x| *x += 1)
            .or_insert(1);
    }
    
    let num_pairs: i32 = counts
        .values()
        .map(|&v| v / 2)
        .sum();

    num_pairs 
}


fn main() {
    println!("{}", sock_pairs("CABBACCCC"))
}
