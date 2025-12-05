use std::env;
use std::fs::read_to_string;

fn compute_joltage(bank: &str, battery_amount: usize) -> u128 {
    println!("Current bank: {}", bank);
    let mut activated_batteries: Vec<u128> = vec![0; battery_amount];

    let mut char_iter = bank.chars().peekable();
    while let Some(digit_ch) = char_iter.next() {
        let digit: u128 = digit_ch.to_digit(10).unwrap().into();
        // move digits forward
        for pos in 0..battery_amount - 1 {
            // if pos > 0 {
            if pos < battery_amount && activated_batteries[pos + 1] > activated_batteries[pos] {
                activated_batteries[pos] = activated_batteries[pos + 1];
                activated_batteries[pos + 1] = 0;
            }
            // if activated_batteries[pos] > activated_batteries[pos - 1] {
            //     activated_batteries[pos - 1] = activated_batteries[pos];
            //     activated_batteries[pos] = 0;
            // }
            // }
        }
        // check if digit should fill last slot
        if activated_batteries[battery_amount - 1] < digit {
            activated_batteries[battery_amount - 1] = digit;
        }
    }
    println!("activated batteries: {:?}", activated_batteries);
    let joltage = activated_batteries
        .iter()
        .rev()
        .enumerate()
        .map(|(pos, d)| d * (10 as u128).pow((pos as u128).try_into().unwrap()))
        .sum();
    println!("Bank Joltage: {}", joltage);
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
        total_joltage += compute_joltage(&line, 12);
    }
    println!("Computed total joltage: {}", total_joltage);
}
