fn insert_first_line(line: &String) -> Vec<Vec<String>> {
    let mut grid: Vec<Vec<String>> = vec![];

    let split_line: Vec<&str> = line.split(" ").collect();

    for num in split_line.iter() {
        let str = String::from(*num);
        if str == "" {
            continue;
        }
        grid.push(vec![str]);
    }
    return grid;
}

fn insert_line(grid: &mut Vec<Vec<String>>, line: &String) {
    let split_line: Vec<&str> = line.split(" ").collect();
    let mut current_column = 0;

    for num in split_line.iter() {
        let str = String::from(*num);
        if str == "" {
            continue;
        }
        grid.get_mut(current_column).expect("").push(str);
        current_column += 1;
    }
}

fn calculate_column(col: &mut Vec<String>) -> u64 {
    let operation = col.pop().unwrap();
    println!("{col:?}");

    if operation == "+" {
        return col.iter().map(|x| x.parse::<u64>().unwrap()).sum();
    } else if operation == "*" {
        return col.iter().map(|x| x.parse::<u64>().unwrap()).product();
    } else {
        println!("Encountered error, no valid operation");
        return 0;
    }
}

pub fn task1(file_input: Vec<String>) {
    println!("Executing Task 1!");
    let mut grid = insert_first_line(&file_input[0]);
    for line in 1..file_input.len() {
        insert_line(&mut grid, &file_input[line]);
    }
    let total_sum: u64 = grid.iter_mut().map(|col| calculate_column(col)).sum();
    println!("TOTAL SUM: {total_sum}");
}
