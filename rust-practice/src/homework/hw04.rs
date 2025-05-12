fn main() {
    let height = 6;

    for i in 0..height {
        let spaces = height - i - 1;
        let stars = 2 * i + 1;
        println!("{}{}", " ".repeat(spaces), "*".repeat(stars));
    }

    for i in (0..height - 1).rev() {
        let spaces = height - i - 1;
        let stars = 2 * i + 1;
        println!("{}{}", " ".repeat(spaces), "*".repeat(stars));
    }
}