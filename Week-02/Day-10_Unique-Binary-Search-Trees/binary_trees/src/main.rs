
fn factorial(n: i32) -> i32 {

    let mut fact: i32 = 1;

    for i in 1..=n {
        fact *= i; 
    }

    fact 
}

fn catalan_number(n: i32) -> i32 {

    factorial(2*n) / (factorial(n+1)*factorial(n))

}


fn main() {

    let n: i32 = 5;

    println!("{}", catalan_number(n));

}
