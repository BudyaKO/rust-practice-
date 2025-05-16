fn is_prime(n: &u32) -> bool {
    if *n < 2 {
        return false;
    }
    for i in 2..=(*n as f64).sqrt() as u32 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let nums = [0, 1, 2, 3, 4, 5, 100, 10007];
    for n in nums {
        println!("{} is prime? {}", n, is_prime(&n));
    }
}
