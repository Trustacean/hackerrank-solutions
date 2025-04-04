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

// Complexity = {time: O(n), space: O(1)}
fn counting_valleys(_steps: i32, path: &str) -> i32 {
    let mut altitude = 0;
    let mut valley_count = 0;

    for c in path.chars() {
        if c == 'U' {
            altitude += 1;
        }
        if c == 'D' {
            if altitude == 0 {
                valley_count += 1;
            }
            altitude -= 1;
        }
    }

    valley_count
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let steps = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let path = stdin_iterator.next().unwrap().unwrap();

    let result = counting_valleys(steps, &path);

    writeln!(&mut fptr, "{}", result).ok();
}
