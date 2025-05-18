fn rotate(s: String, n: isize) -> String {
    let len = s.len();
    if len == 0 {
        return s;
    }
    let n = ((n % len as isize) + len as isize) % len as isize;
    let n = n as usize;
    let (a, b) = s.split_at(len - n);
    format!("{}{}", b, a)
}

fn main() {
    let s = "abcdefgh".to_string();
    let shifts = [0, 8, -8, 1, 2, 10, -1, -2, -10];

    for n in shifts.iter() {
        let result = rotate(s.clone(), *n);
        println!("rotate({}, {}) = {}", s, n, result);
    }
}