fn count_permutation(shipments: &Vec<u32>) -> Option<usize> {
    let n = shipments.len();
    let sum: u32 = shipments.iter().sum();
    if sum as usize % n != 0 { return None; }
    let avg = sum / n as u32;
    Some(shipments.iter().filter(|&&x| x > avg).map(|&x| (x - avg) as usize).sum())
}

fn gen_shipments() -> Vec<u32> {
    vec![8, 2, 2, 4, 4]
}

fn main() {
    let shipments = gen_shipments();
    println!("Shipments: {:?}", shipments);
    if let Some(moves) = count_permutation(&shipments) {
        let avg = shipments.iter().sum::<u32>() / shipments.len() as u32;
        println!("Total load: {}", shipments.iter().sum::<u32>());
        println!("Average load per ship: {}\n", avg);
        println!("Moves needed to equalize:");
        for (i, &load) in shipments.iter().enumerate() {
            if load > avg {
                println!("- Ship {}: {} -> needs to give away {} ({} - {})", i, load, load - avg, load, avg);
            } else if load < avg {
                println!("- Ship {}: {} -> needs to receive {} ({} - {})", i, load, avg - load, avg, load);
            } else {
                println!("- Ship {}: {} -> exactly {}, no action needed", i, load, avg);
            }
        }
        println!("\nMinimum moves to equalize: {}", moves);
    } else {
        println!("Equal distribution is not possible.");
    }
}