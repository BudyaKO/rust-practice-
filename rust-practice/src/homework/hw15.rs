fn main() {
    let mut count = 0;
    for m in 1..10 {
        for u in 0..10 {
            for x in 0..10 {
                for a in 0..10 {
                    for s in 1..10 {
                        for l in 0..10 {
                            for o in 0..10 {
                                for n in 0..10 {
                                    let digits = [m,u,x,a,s,l,o,n];
                                    if digits.iter().collect::<std::collections::HashSet<_>>().len() != digits.len() {
                                        continue;
                                    }
                                    let muza = m*1000 + u*100 + x*10 + a;
                                    let xa = x*10 + a;
                                    let slon = s*1000 + l*100 + o*10 + n;
                                    if muza + xa == slon {
                                        count += 1;
                                        println!("Рішення #{}:\n  {}{}{}{}\n+     {}{}\n-----------\n  {}{}{}{}\n", 
                                            count, m,u,x,a,x,a,s,l,o,n);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("Загальна кількість рішень: {}", count);
}
//Відповідь: 16