use std::collections::HashMap;

fn sockMerchant(n: i32, ar: &[i32]) -> i32 {
    let mut freq_map = HashMap::new();

    for &sock in ar {
        *freq_map.entry(sock).or_insert(0) += 1;
    }

    let mut pairs = 0;

    for &count in freq_map.values() {
        pairs += count / 2;
    }

    pairs
}

#[test]
fn main(){
    let ar = vec![1, 1, 2, 2, 3, 3];
    assert_eq!(sockMerchant(ar.len() as i32, &ar), 3);
    println!("Success!");
}