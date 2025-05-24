fn gray(n: u8) -> Vec<String> {
    let count = 1 << n;
    let mut result = Vec::with_capacity(count);
    for i in 0..count {
        result.push(format!("{:0width$b}", i, width = n as usize));
    }
    result
}

fn main() {
    let codes = gray(3);
    for code in codes {
        println!("{}", code);
    }
}
