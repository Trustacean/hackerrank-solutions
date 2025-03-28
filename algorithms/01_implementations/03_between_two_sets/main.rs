use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'getTotalX' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY a
 *  2. INTEGER_ARRAY b
 */

//  Complexity = {Time: O(k⋅(m+n)), Space: O(1)}
fn get_total_x(a: &[i32], b: &[i32]) -> i32 {
    let mut total = 0;
    let max_a = *a.iter().max().unwrap();
    let min_b = *b.iter().max().unwrap();

    for i in max_a..=min_b {
        if a.iter().all(|x| i % x == 0) && b.iter().all(|y| y % i == 0) {
            total += 1;
        }
    }

    total
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let first_multiple_input: Vec<String> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let n = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let m = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let brr: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let total = get_total_x(&arr, &brr);

    writeln!(&mut fptr, "{}", total).ok();
}
