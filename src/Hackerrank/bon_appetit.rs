fn bonAppetit(bill: &[i32], k: i32, b: i32) {
    let total_cost: i32 = bill.iter().enumerate().filter_map(|(i, &cost)| {
        if i != k as usize {
            Some(cost)
        } else {
            None
        }
    }).sum();

    let correct_share = total_cost / 2;

    if correct_share == b {
        println!("Bon Appetit");
    } else {
        println!("{}", b - correct_share);
    }
}