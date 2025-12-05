use std::env;
use std::fs::read_to_string;

fn compute_joltage(bank: &str) -> u32 {
    // println!("Current bank: {}", bank);
    let mut max_battery_1: u32 = 0;
    let mut max_battery_2: u32 = 0;

    let mut char_iter = bank.chars().peekable();
    while let Some(digit_ch) = char_iter.next() {
        let digit: u32 = digit_ch.to_digit(10).unwrap();

        if max_battery_2 > max_battery_1 {
            max_battery_1 = max_battery_2;
            max_battery_2 = digit;
        }
        if digit > max_battery_2 {
            max_battery_2 = digit;
        }
    }
    let joltage = max_battery_1 * 10 + max_battery_2;
    // println!("Bank Joltage: {}", joltage);
    return joltage;
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let lines = read_lines(&args[1]);
    let mut total_joltage = 0;

    for line in lines {
        total_joltage += compute_joltage(&line);
    }
    println!("Computed total joltage: {}", total_joltage);
}
