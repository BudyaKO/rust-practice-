const W: usize = 30;
const H: usize = 15;
fn main() {
    if W < 10 || W > 80 || H < 10 || H > 80 {
        println!();
        return;
    }
    for y in 0..H {
        for x in 0..W {
            let left_diag = x * H == y * W;
            let right_diag = (W - 1 - x) * H == y * W;
            let border = y == 0 || y == H - 1 || x == 0 || x == W - 1;
            let ch = if border || left_diag || right_diag {
                '*'
            } else {
                ' '
            };
            print!("{}", ch);
        }
        println!();
    }
}