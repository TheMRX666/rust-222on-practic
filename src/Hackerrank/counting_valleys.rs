use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'countingValleys' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER steps
 *  2. STRING path
 */

fn countingValleys(steps: i32, path: &str) -> i32 {
    let mut altitude = 0;
    let mut valleys = 0;
    let mut in_valley = false;

    for step in path.chars() {
        if step == 'U' {
            altitude += 1;
        } else if step == 'D' {
            altitude -= 1;
        }

        if altitude == 0 && in_valley {
            valleys += 1;
            in_valley = false;
        }

        if altitude < 0 && !in_valley {
            in_valley = true;
        }
    }
    valleys
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let steps = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let path = stdin_iterator.next().unwrap().unwrap();

    let result = countingValleys(steps, &path);

    writeln!(&mut fptr, "{}", result).ok();
}