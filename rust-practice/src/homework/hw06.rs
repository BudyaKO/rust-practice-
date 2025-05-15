fn main() {
    let levels = 5;
    let max_width = 1 + 2 * levels;

    for level in 1..=levels {
        for i in 0..level {
            let stars = 1 + i * 2;
            let spaces = (max_width - stars) / 2;
            println!("{}{}", " ".repeat(spaces), "*".repeat(stars));
        }
    }
}