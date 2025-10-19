fn merge_sorted(vec_1: Vec<i32>, vec_2: Vec<i32>) -> Vec<i32> {
    
    let mut vec_12: Vec<i32> = [vec_1, vec_2].concat();
    vec_12.sort();

    vec_12

}

fn main() {
    
    let vec_1 = vec![1,2,3];
    let vec_2 = vec![2,5,6];

    let vec_12 = merge_sorted(vec_1, vec_2);

    println!("{:#?}", vec_12)

}
