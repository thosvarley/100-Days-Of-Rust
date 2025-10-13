use std::io;

fn find_nemo(sentence: &str) {
    
    let parts = sentence.split_whitespace();
    
    let mut counter = 0;
    for part in parts {
        counter += 1;
        if part == "Nemo" {
            println!("I found Nemo at {}!", counter);
            return;
        }
    }
    println!("I can't find Nemo :(")
}


fn main() {
    
    println!("Please input a sentence.");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");
    
    find_nemo(&input)

}
