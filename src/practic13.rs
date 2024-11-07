use rand::Rng;

fn count_permutation(shipments: &Vec<u32>) -> usize {
    let n = shipments.len() as u32;
    let total_weight: u32 = shipments.iter().sum();

    if total_weight % n != 0 {
        return usize::MAX;
    }

    let target_weight = total_weight / n;
    let mut moves: usize = 0;

    for &weight in shipments {
        if weight > target_weight {
            moves += (weight - target_weight) as usize;
        }
    }

    moves
}

fn count_permutation2(shipments: &Vec<u32>) -> Option<usize> {
    let n = shipments.len() as u32;
    let total_weight: u32 = shipments.iter().sum();

    if total_weight % n != 0 {
        return None;
    }

    let target_weight = total_weight / n;
    let mut moves: usize = 0;

    for &weight in shipments {
        if weight > target_weight {
            moves += (weight - target_weight) as usize;
        }
    }

    Some(moves)
}


fn gen_shipments(n: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let target_weight = rng.gen_range(1..10);
    let total_weight = target_weight * n as u32;

    let mut shipments = vec![target_weight; n];
    let adjustments: usize = rng.gen_range(0..n / 2);

    for _ in 0..adjustments {
        let idx = rng.gen_range(0..n);
        shipments[idx] += rng.gen_range(0..target_weight / 2);
    }

    let current_total: u32 = shipments.iter().sum();
    let correction = total_weight as i32 - current_total as i32;
    shipments[0] = (shipments[0] as i32 + correction) as u32;

    shipments
}

#[test]
fn main() {
    let shipments = vec![1, 1, 1, 1, 6];
    let result = count_permutation(&shipments);
    println!("{}", result);
    let result2 = count_permutation(&shipments);
    println!("{}", result2);
    let result3 = gen_shipments(5);
    println!("{:?}", result3);
}