use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'dayOfProgrammer' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts INTEGER year as parameter.
 */

//  Complexity = {Time: O(1), Space: O(1)}
fn day_of_programmer(year: i32) -> String {
    let mut days = 256;
    let mut m_counter = 1;
    let feb;
    
    if year == 1918 {
        feb = 15;
    } else if (year < 1918 && year % 4 == 0) || year % 400 == 0 || (year % 4 == 0 && year % 100 != 0) {
        feb = 29;
    } else {
        feb = 28;
    }
    
    let month_days = [31, feb, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    
    for &days_in_month in &month_days {
        if days < days_in_month {
            break;
        }
        days -= days_in_month;
        m_counter += 1;
    }
    
    format!("{:0>2}.{:0>2}.{:0>4}", days, m_counter, year)
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let year = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let result = day_of_programmer(year);

    writeln!(&mut fptr, "{}", result).ok();
}
