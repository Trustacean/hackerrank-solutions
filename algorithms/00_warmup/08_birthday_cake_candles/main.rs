use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'birthdayCakeCandles' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY candles as parameter.
 */

//  Complexity = {Time: O(n), Space: O(1)}
fn birthday_cake_candles(candles: &[i32]) -> i32 {
    let mut max = 0;
    let mut count = 0;
    for &i in candles {
        if i == max {
            count += 1;
        }
        if max < i {
            max = i;
            count = 1;
        }
    }
    count
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _candles_count = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let candles: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = birthday_cake_candles(&candles);

    writeln!(&mut fptr, "{}", result).ok();
}
