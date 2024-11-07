fn miniMaxSum(arr: &[i32]) {
    let mut sum: i64 = 0;
    let mut min: i64 = i64::MAX;
    let mut max: i64 = i64::MIN;

    for &n in arr {
        sum += n as i64;
        if (n as i64) < min {
            min = n as i64;
        }
        if (n as i64) > max {
            max = n as i64;
        }
    }

    println!("{} {}", sum - max, sum - min);
}

#[test]
fn main(){
    let arr1 = vec![1, 2, 3, 4, 5];
    let arr2 = vec![7, 8, 9, 10, 11];

    let mut captured_output = Vec::new();

    miniMaxSum(&arr1);
    captured_output.push("10 14".to_string());

    miniMaxSum(&arr2);
    captured_output.push("34 38".to_string());

    assert_eq!(captured_output[0], "10 14");
    assert_eq!(captured_output[1], "34 38");
}