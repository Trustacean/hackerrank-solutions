use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'divisibleSumPairs' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. INTEGER k
 *  3. INTEGER_ARRAY ar
 */

//  Complexity = {Time: O(n^2), Space: O(1)}
fn divisible_sum_pairs(n: i32, k: i32, ar: &[i32]) -> i32 {
    let mut total = 0;
    
    // for i in 0..n {
    //     for j in i..n {
    //         let i = i as usize;
    //         let j = j as usize;
    //         if i == j {
    //             continue;
    //         }    A
    //         if (ar[i] + ar[j]) % k == 0 {
    //             total += 1;
    //         }
    //     }
    // }
    
    for (i, j) in (0..n).map(|i| (i..n).map(move |j| (i,j))).flatten() {
        if i != j && (ar[i as usize] + ar[j as usize]) % k == 0 {
            total += 1;
        }
    }
    
    total
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let n = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let k = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let ar: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = divisible_sum_pairs(n, k, &ar);

    writeln!(&mut fptr, "{}", result).ok();
}
