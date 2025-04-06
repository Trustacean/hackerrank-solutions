use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'pickingNumbers' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY a as parameter.
 */

fn picking_numbers(a: &[i32]) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut max = 0;

    for &i in a {
        let counter = map.entry(i).or_insert(0);
        *counter += 1;
    }

    let keys: Vec<i32> = map.keys().cloned().collect();

    for key in keys {
        let lower = *map.entry(key - 1).or_insert(0);
        let count = *map.get(&key).unwrap();
        let higher = *map.entry(key + 1).or_insert(0);

        max = max.max(lower + count).max(higher + count);
    }

    max
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let a: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = picking_numbers(&a);

    writeln!(&mut fptr, "{}", result).ok();
}
