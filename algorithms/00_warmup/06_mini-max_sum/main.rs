use std::io::{self, BufRead};

/*
 * Complete the 'miniMaxSum' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn mini_max_sum(arr: &[i32]) {
    let mut sum : i64 = 0;
    let mut max : i64 = 0;
    let mut min : i64 = std::i64::MAX;
    
    for &i in arr {
        let i = i as i64;
        sum += i;
        if max < i {
            max = i;
        }
        if min > i {
            min = i;
        }
    }
    
    println!("{} {}", sum - max, sum - min)
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    mini_max_sum(&arr);
}
