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

#[test]
fn main() {
    let shipments = vec![1, 1, 1, 1, 6];
    let result = count_permutation(&shipments);
    println!("{}", result);
    let result2 = count_permutation(&shipments);
    println!("{}", result2);
}