struct R(u32);
impl R {
    fn new(s:u32)->Self{R(s)}
    fn n(&mut self)->u32{self.0=self.0.wrapping_mul(1664525).wrapping_add(1013904223);self.0}
    fn r(&mut self,l:i32,h:i32)->i32{(self.n()%((h-l)as u32)+l as u32) as i32}
}

fn main() {
    let mut r=R::new(123);
    let v: Vec<i32> = (0..20).map(|_| r.r(10,100)).collect();
    let (mut i, mut m) = (0, v[0]+v[1]);
    for j in 1..v.len()-1 {
        let s = v[j] + v[j+1];
        if s < m { m = s; i = j; }
    }

    print!("indexes:");
    for idx in 0..v.len() { print!(" {:2}.", idx); }
    println!();
    print!("data:   ");
    for &x in &v { print!(" {:2},", x); }
    println!();

    print!("indexes:");
    for idx in 0..v.len() {
        if idx == i { print!("\\__"); }
        else if idx == i+1 { print!(" __/"); }
        else { print!("   "); }
    }
    println!();
    println!("Min adjacent sum = {} + {} = {} at indexes {}, {}", v[i], v[i+1], m, i, i+1);
}