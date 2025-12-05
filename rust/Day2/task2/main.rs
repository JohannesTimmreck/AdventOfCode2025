use std::convert::TryInto;
use std::env;
use std::fs::read_to_string;

fn checkRange(rangeStart: i64, rangeEnd: i64) -> i64 {
    println!("Check Range: {}-{}", rangeStart, rangeEnd);
    let mut sumOfInvalid: i64 = 0;
    let mut currentNumber = rangeStart;
    while currentNumber <= rangeEnd {
        let curNumStr = currentNumber.to_string();
        let curNumLen: i64 = curNumStr.chars().count().try_into().unwrap();
        let mut repNumLen = 1;
        while repNumLen <= curNumLen / 2 {
            if curNumLen % repNumLen != 0 {
                repNumLen += 1;
                continue;
            }
            let mut isInvalid = true;
            let mut iteration: i64 = 0;
            while iteration != repNumLen {
                isInvalid = true;
                let currentDigit = curNumStr.chars().nth(iteration.try_into().unwrap());
                let mut possibleSlot = iteration + repNumLen;
                let mut slotsToCheck: Vec<i64> = Vec::new();
                while possibleSlot < curNumLen {
                    slotsToCheck.push(possibleSlot);
                    possibleSlot += repNumLen;
                }
                for slotToCheck in slotsToCheck {
                    if currentDigit != curNumStr.chars().nth(slotToCheck.try_into().unwrap()) {
                        isInvalid = false;
                        break;
                    }
                }
                if isInvalid == false {
                    break;
                }
                iteration += 1;
            }
            if isInvalid == true {
                println!("Found invalid Number {}", currentNumber);
                sumOfInvalid += currentNumber;
                break;
            }
            repNumLen += 1;
        }
        currentNumber += 1;
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
