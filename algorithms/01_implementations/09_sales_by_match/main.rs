use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::collections::HashMap;

/*
 * Complete the 'sockMerchant' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. INTEGER_ARRAY ar
 */

// Complexity = {time: O(n), space: O(m)} where n = array size, m = colors
fn sock_merchant(_n: i32, ar: &[i32]) -> i32 {
    let mut socks : HashMap<i32, i32> = HashMap::new();
    let mut pairs = 0;
    
    for &i in ar {
        let counter = socks.entry(i).or_insert(0);
        *counter += 1;
        if socks.get(&i).unwrap() % 2 == 0 {
            pairs += 1;
        }
    }
    
    pairs
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let ar: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = sock_merchant(n, &ar);

    writeln!(&mut fptr, "{}", result).ok();
}
