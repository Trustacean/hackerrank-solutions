use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'aVeryBigSum' function below.
 *
 * The function is expected to return a LONG_INTEGER.
 * The function accepts LONG_INTEGER_ARRAY ar as parameter.
 */

//  Complexity = {Time: O(n), Space: O(1)}
fn a_very_big_sum(ar: &[i64]) -> i64 {
    let mut sum : i64 = 0;
    for &i in ar {
        sum += i;
    }
    sum
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _ar_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let ar: Vec<i64> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i64>().unwrap())
        .collect();

    let result = a_very_big_sum(&ar);

    writeln!(&mut fptr, "{}", result).ok();
}
