struct line_information


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

fn insert_line(grid: &mut Vec<Vec<String>>, line: &String) {
    // let split_line: Vec<&str> = line.split(" ").collect();
    let mut cur_char = 0;
    let mut current_column = 0;

    for num in line.chars() {
        if cur_char == 4 {
            cur_char = 0;
            current_column += 1;
            continue;
        } else {
            if num == '+' || num == '*' {
                grid[current_column].push(String::from(num));
            } else {
                grid[current_column][cur_char].push(num);
            }
        }
        cur_char += 1;
    }
}

fn calculate_column(col: &mut Vec<String>) -> u64 {
    println!("{col:?}");
    let operation = col.pop().unwrap();
    println!("{col:?}");

    if operation == "+" {
        return col.iter().map(|x| x.trim().parse::<u64>().unwrap()).sum();
    } else if operation == "*" {
        return col
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
    let mut grid = insert_first_line(&file_input[0]);
    for line in 1..file_input.len() {
        insert_line(&mut grid, &file_input[line]);
    }
    let total_sum: u64 = grid.iter_mut().map(|col| calculate_column(col)).sum();
    println!("TOTAL SUM: {total_sum}");
}
