fn main() {
    let times = 3;
    let values = vec![1, 2, 3, 4];

    for &val in &values {
        for _ in 0..times {
            println!("{}", val);
        }
    }
}