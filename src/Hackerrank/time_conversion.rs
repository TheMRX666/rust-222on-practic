use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'timeConversion' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 */

fn timeConversion(s: &str) -> String {
    let hour = &s[0..2];
    let minute = &s[3..5];
    let second = &s[6..8];
    let period = &s[8..];

    let mut hour_int: i32 = hour.parse().unwrap();

    if period == "AM" {
        if hour_int == 12 {
            hour_int = 0;
        }
    } else {
        if hour_int != 12 {
            hour_int += 12;
        }
    }

    format!("{:02}:{:}:{:}", hour_int, minute, second)
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = timeConversion(&s);

    writeln!(&mut fptr, "{}", result).ok();
}
