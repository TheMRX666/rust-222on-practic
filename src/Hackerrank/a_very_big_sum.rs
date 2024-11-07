fn aVeryBigSum(ar: &[i64]) -> i64 {
    let mut s = 0;
    for &value in ar {
        s += value;
    }
    s
}

#[test]
fn main() {
    let ar = vec![1000000001, 1000000002, 1000000003, 1000000004, 1000000005];
    let result = aVeryBigSum(&ar);
    println!("{}", result);
}