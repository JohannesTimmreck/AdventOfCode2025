#[derive(Debug)]
struct line_information {
    operation: char,
    digit_amount: usize,
    numbers: Vec<String>,
}

fn insert_first_line(line: &String) -> Vec<Vec<String>> {
    let mut grid: Vec<Vec<String>> = vec![];

    let mut cur_char = 0;
    let mut current_column = 0;

    grid.push(vec![]);
    for num in line.chars() {
        if cur_char == 3 {
            cur_char = 0;
            current_column += 1;
            grid.push(vec![]);
            continue;
        } else {
            grid[current_column].push(String::from(num));
        }
        cur_char += 1;
    }
    // let split_line: Vec<&str> = line.split(" ").collect();
    //
    // for num in split_line.iter() {
    //     let str = String::from(*num);
    //     if str == "" {
    //         continue;
    //     }
    //     let mut new_nums: Vec<String> = vec![];
    //     while new_nums.len() + str.len() < 3 {
    //         new_nums.push(String::from(""));
    //     }
    //     for c in str.chars() {
    //         new_nums.push(String::from(c));
    //     }
    //     grid.push(new_nums);
    // }
    return grid;
}

fn insert_operations_line(line: &String) -> Vec<line_information> {
    let mut line_infos: Vec<line_information> = vec![];
    let mut remember_char: char = ' ';
    let mut digit_amount: usize = 0;

    for ch in line.chars() {
        digit_amount += 1;
        if ch == '+' || ch == '*' {
            if remember_char != ' ' {
                line_infos.push(line_information {
                    operation: remember_char,
                    digit_amount: digit_amount - 1,
                    numbers: vec![String::from(""); (digit_amount - 1) as usize],
                });
            }
            digit_amount = 0;
            remember_char = ch;
        }
    }
    line_infos.push(line_information {
        operation: remember_char,
        digit_amount: digit_amount + 1,
        numbers: vec![String::from(""); (digit_amount + 1) as usize],
    });
    return line_infos;
}

fn insert_line(grid: &mut Vec<line_information>, line: &String) {
    // println!("Current Line Information");
    // println!("{grid:?}");
    // let split_line: Vec<&str> = line.split(" ").collect();
    let mut current_column: usize = 0;
    let mut cur_char: usize = 0;

    for num in line.chars() {
        if cur_char == grid[current_column].digit_amount {
            cur_char = 0;
            current_column += 1;
            continue;
        } else {
            grid[current_column].numbers[cur_char].push(num);
        }
        cur_char += 1;
    }
}

fn calculate_column(col: &mut line_information) -> u64 {
    if col.operation == '+' {
        return col
            .numbers
            .iter()
            .map(|x| x.trim().parse::<u64>().unwrap())
            .sum();
    } else if col.operation == '*' {
        return col
            .numbers
            .iter()
            .map(|x| x.trim().parse::<u64>().unwrap())
            .product();
    } else {
        println!("Encountered error, no valid operation");
        return 0;
    }
}

pub fn task2(file_input: Vec<String>) {
    println!("Executing Task 2!");
    // let mut grid = insert_first_line(&file_input[0]);
    let mut grid = insert_operations_line(&file_input.last().unwrap());
    for line in 0..file_input.len() - 1 {
        insert_line(&mut grid, &file_input[line]);
    }
    println!("Current Line Information");
    println!("{grid:?}");
    let total_sum: u64 = grid.iter_mut().map(|col| calculate_column(col)).sum();
    println!("TOTAL SUM: {total_sum}");
}
