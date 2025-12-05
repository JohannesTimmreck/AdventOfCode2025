use std::convert::TryInto;
use std::env;
use std::fs::read_to_string;

fn checkRange(rangeStart: i64, rangeEnd: i64) -> i64 {
    println!("Check Range: {}-{}", rangeStart, rangeEnd);
    let mut sumOfInvalid: i64 = 0;
    let mut currentNumber = rangeStart - 1;
    while currentNumber < rangeEnd {
        currentNumber += 1;
        let currentNumberString = currentNumber.to_string();
        let charactersAmount: i64 = currentNumberString.chars().count().try_into().unwrap();
        if charactersAmount % 2 == 1 {
            continue;
        }
        let mut iteration: i64 = -1;
        while iteration < charactersAmount / 2 {
            iteration += 1;
            if currentNumberString
                .chars()
                .nth(iteration.try_into().unwrap())
                != currentNumberString
                    .chars()
                    .nth((iteration + charactersAmount / 2).try_into().unwrap())
            {
                break;
            }
            if iteration + 1 == charactersAmount / 2
                && currentNumberString
                    .chars()
                    .nth(iteration.try_into().unwrap())
                    == currentNumberString
                        .chars()
                        .nth((iteration + charactersAmount / 2).try_into().unwrap())
            {
                sumOfInvalid = sumOfInvalid + currentNumber;
                // println!("{} is Invalid Number", currentNumber);
                // println!("New Sum is = {}", sumOfInvalid);
            }
        }
    }
    // println!("Sum of Range: {}", sumOfInvalid);
    return sumOfInvalid;
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
    let mut sumOfInvalidTotal: i64 = 0;

    let ranges: Vec<&str> = lines[0].split(',').collect();
    for range in ranges {
        let rangeLimits: Vec<&str> = range.split('-').collect();
        sumOfInvalidTotal = sumOfInvalidTotal
            + checkRange(
                rangeLimits[0].parse().expect("Expect Range Start Number"),
                rangeLimits[1].parse().expect("Expect Range End Number"),
            );
    }

    println!("Total Sum of Invalid: {}", sumOfInvalidTotal);
}
