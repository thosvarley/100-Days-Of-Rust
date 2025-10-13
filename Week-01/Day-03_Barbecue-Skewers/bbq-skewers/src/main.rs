fn count_skewers(text: &[&str]) -> [usize; 2] {
    let n = text.len();

    let mut counter = 0;
    for i in 0..n {
        let skewer = text[i];
        if skewer.contains("x") == false {
            counter += 1;
        }
    }

    return [counter, n - counter];
}

fn main() {
    let text = [
        "--oooo-ooo--",
        "--xx--x--xx--",
        "--o---o--oo--",
        "--xx--x--ox--",
        "--xx--x--ox--",
    ];

    let result = count_skewers(&text);
    println!("{:?}", result);
}
