fn count_trapped_zeros(height: &Vec<i32>) -> usize {
    let n = height.len();
    
    // Find first non-zero from left
    let start = height.iter().position(|&x| x > 0);
    // Find first non-zero from right
    let end = height.iter().rposition(|&x| x > 0);
    
    match (start, end) {
        (Some(s), Some(e)) if s < e => {
            // Count zeros between the first and last non-zero
            height[s..=e].iter().filter(|&&x| x == 0).count()
        }
        _ => 0,
    }
}

fn main() {
    let mut height: Vec<i32> = vec![0,1,0,2,1,0,1,3,2,1,2,1];
    let mut total_water = 0;
    
    while height.iter().any(|&x| x > 0) {
        total_water += count_trapped_zeros(&height);
        height = height.iter().map(|&x| (x - 1).max(0)).collect();
    }
    
    println!("{}", total_water);
}
