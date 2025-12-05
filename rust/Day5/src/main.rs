use std::env;
use std::fs::read_to_string;

mod task1;
mod task2;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Expected Usage: ./main <task1|task2> <file_to_input_file>");
        return;
    }
    let lines = read_lines(&args[2]);

    if &args[1] == "task1" {
        task1::task1(lines);
    } else if &args[1] == "task2" {
        task2::task2(lines);
    } else {
        eprintln!("TASK NOT FOUND!");
    }
}
