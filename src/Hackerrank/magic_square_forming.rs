//MEDIUM LEVEL

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn formingMagicSquare(s: &[Vec<i32>]) -> i32 {
    let target_sum = 15;
    let mut input_square = Vec::with_capacity(9);
    for row in s {
        input_square.extend(row.iter());
    }

    let mut min_cost = i32::MAX;
    let all_numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let permutations = get_permutations(all_numbers);

    for perm in permutations {
        let magic_square = perm;
        if is_magic_square(&magic_square, target_sum) {
            let cost: i32 = input_square.iter()
                .zip(magic_square.iter())
                .map(|(a, b): (&i32, &i32)| (a - b).abs())
                .sum();
            min_cost = min_cost.min(cost);
        }
    }

    min_cost
}

fn get_permutations(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut nums = nums;
    permute(&mut nums, &mut result, 0);
    result
}

fn permute(nums: &mut Vec<i32>, result: &mut Vec<Vec<i32>>, start: usize) {
    if start == nums.len() {
        result.push(nums.clone());
        return;
    }
    for i in start..nums.len() {
        nums.swap(start, i);
        permute(nums, result, start + 1);
        nums.swap(start, i);
    }
}

fn is_magic_square(square: &[i32], target_sum: i32) -> bool {
    for i in 0..3 {
        if square[i * 3] + square[i * 3 + 1] + square[i * 3 + 2] != target_sum {
            return false;
        }
        if square[i] + square[i + 3] + square[i + 6] != target_sum {
            return false;
        }
    }

    if square[0] + square[4] + square[8] != target_sum {
        return false;
    }

    if square[2] + square[4] + square[6] != target_sum {
        return false;
    }

    true
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let mut s: Vec<Vec<i32>> = Vec::with_capacity(3_usize);

    for i in 0..3_usize {
        s.push(Vec::with_capacity(3_usize));

        s[i] = stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();
    }

    let result = formingMagicSquare(&s);

    writeln!(&mut fptr, "{}", result).ok();
}
