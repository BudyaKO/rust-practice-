fn is_palindrome(x: u32) -> bool {
    let s = x.to_string();
    s.chars().eq(s.chars().rev())
}

fn main() {
    let test_numbers = [123, 121, 1221];
    for &num in &test_numbers {
        println!("{} palindrome? {}", num, is_palindrome(num));
    }
}