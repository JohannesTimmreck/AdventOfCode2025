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
        grid.iter().nth(current_column).expect("").push(str);
        current_column += 1;
    }
}

fn calculate_column(col: Vec<u32>, op: char) {}

pub fn task1(file_input: Vec<String>) {
    println!("Executing Task 1!");
}
