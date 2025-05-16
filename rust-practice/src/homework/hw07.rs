pub fn invert_the_case(s: String) -> String {
    s.chars()
        .map(|c| match c {
            c if c.is_uppercase() => c.to_lowercase().next().unwrap(),
            c if c.is_lowercase() => c.to_uppercase().next().unwrap(),
            _ => c,
        })
        .collect()
}

fn main() {
    let input = "Привіт".to_string();
    let output = invert_the_case(input);
    println!("{}", output);
}