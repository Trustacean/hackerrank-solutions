use std::io::{self, BufRead};
use std::cmp::Ordering;

/*
 * Complete the 'plusMinus' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

//  Complexity = {Time: O(n), Space: O(1)}
fn plus_minus(arr: &[i32]) {
    let mut positives = 0;
    let mut negatives = 0;
    let mut zeros = 0;
    
    for i in arr {
        match i.cmp(&0) {
            Ordering::Greater => positives += 1,
            Ordering::Less => negatives += 1,
            Ordering::Equal => zeros += 1,
        }
    }
    
    
    let total : f32 = arr.len() as f32;
    let pos_ratio : f32 = positives as f32/total;
    let neg_ratio : f32 = negatives as f32/total;
    let zer_ratio : f32 = zeros as f32/total;
    
    println!("{}", pos_ratio);
    println!("{}", neg_ratio);
    println!("{}", zer_ratio);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    plus_minus(&arr);
}