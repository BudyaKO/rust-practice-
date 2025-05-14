fn gcd(a: u32, b: u32) -> u32 {
    let mut a = a;
    let mut b = b; 
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn main() {
    let data = [
        (24, 60),
        (15, 9),
        (15, 6),
        (140, 40),
        (24, 16),
        (100, 10),
        (120, 80),
        (80, 120),
        (100, 20),
        (37, 11),
        (120, 90),
    ];

    for (a, b) in data {
        println!("gcd({}, {}) = {}", a, b, gcd(a, b));
    }
}
