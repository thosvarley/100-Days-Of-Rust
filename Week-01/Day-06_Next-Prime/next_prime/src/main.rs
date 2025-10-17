fn sieve_of_eratosthenes(n_max: i32) -> Vec<i32> {
    let mut nums: Vec<i32> = (2..n_max).collect();

    let mut i = 2;
    while i * i <= n_max {
        if nums.contains(&i) {
            nums = nums.into_iter().filter(|&x| x == i || x % i != 0).collect();
        }

        // Find next i from current nums > i
        i = match nums.iter().find(|&&x| x > i) {
            Some(&next) => next,
            None => break,
        };
    }

    nums
}

fn next_prime(num: i32, n_max: i32) -> i32 {

    let primes: Vec<i32> = sieve_of_eratosthenes(n_max);
    let filtered: Vec<i32> = primes.into_iter().filter(|x| x >= &num).collect();

    filtered[0]
}

fn main() {
    
    println!("{}", next_prime(12, 50));
    println!("{}", next_prime(24, 50));
    println!("{}", next_prime(11, 50));

}
