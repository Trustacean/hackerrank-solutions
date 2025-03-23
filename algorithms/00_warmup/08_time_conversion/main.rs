use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'timeConversion' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 */

fn timeConversion(s: &str) -> String {
    let mut parts = s.split(":");
    let mut h : i32 = parts.next().unwrap().parse::<i32>().unwrap();
    let m = parts.next().unwrap();
    let s = parts.next().unwrap();
    if &s[2..4] == "PM" {
        if h != 12 {
            h +=12;
        }
    } else {
        if h == 12 {
            h = 0;
        }
    }
    format!("{:02}:{}:{}", h, m, &s[0..2])
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = timeConversion(&s);

    writeln!(&mut fptr, "{}", result).ok();
}
