use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'migratoryBirds' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

//  Complexity = {Time: O(n), Space: O(n)}
fn migratory_birds(arr: &[i32]) -> i32 {
    let mut types: HashMap<i32, i32> = HashMap::new();
    let mut most_freq_type = arr[0];

    for &i in arr {
        let counter = types.entry(i).or_insert(0);
        *counter += 1;

        if types.get(&most_freq_type).unwrap() < types.get(&i).unwrap() {
            most_freq_type = i;
        } else if types.get(&most_freq_type).unwrap() == types.get(&i).unwrap()
            && most_freq_type > i
        {
            most_freq_type = i;
        }
    }

    most_freq_type
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _arr_count = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let arr: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = migratory_birds(&arr);

    writeln!(&mut fptr, "{}", result).ok();
}
