fn progress_days(days: &[i32]) -> i32 {
    
    
    let mut counter: i32 = 0;
    for i in 1..days.len() {
        let today: i32 = days[i];
        let yesterday: i32 = days[i-1];
        
        if today > yesterday {
            counter += 1;
        }
    }
    counter 
}

fn main() {

    println!("{}", progress_days(&[10, 11, 12, 9, 10]))

}
