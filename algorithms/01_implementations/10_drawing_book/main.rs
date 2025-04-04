use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'pageCount' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. INTEGER p
 */

// Complexity = {time: O(1), space: O(1)}
fn page_pount(n: i32, p: i32) -> i32 {
    let front = p / 2;
    let back = (n / 2) - front;
    
    front.min(back)
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let p = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let result = page_pount(n, p);

    writeln!(&mut fptr, "{}", result).ok();
}
