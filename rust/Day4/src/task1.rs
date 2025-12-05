fn checkAbove(pos_x: usize, pos_y: usize, input_map: &Vec<String>) -> u32 {
    let mut paper_count = 0;
    if pos_y != 0 {
        let row_to_check = pos_y - 1;
        // Check left
        if pos_x != 0 && (input_map.iter().nth(row_to_check).expect("Should be String input_map").chars().nth(pos_x - 1) != Some('.')) {
            paper_count += 1;
        }
        // check central
        if input_map.iter().nth(row_to_check).expect("Should be String input_map").chars().nth(pos_x) != Some('.') {
            paper_count += 1;
        }
        // /check right
        if pos_x < input_map.iter().nth(row_to_check).expect("Should be String input_map").len() - 1 && (input_map.iter().nth(row_to_check).expect("Should be String input_map").chars().nth(pos_x + 1) != Some('.')) {
            paper_count += 1;
        }
    }
    return paper_count;
}
fn checkSides(pos_x: usize, pos_y: usize, input_map: &Vec<String>) -> u32 {
    let mut paper_count = 0;
    let row_to_check = pos_y;
    // Check left
    if pos_x != 0 && (input_map.iter().nth(row_to_check).expect("Should be String input_map").chars().nth(pos_x - 1) != Some('.')) {
        paper_count += 1;
    }
    // /check right
    if pos_x < input_map.iter().nth(row_to_check).expect("Should be String input_map").len() - 1 && (input_map.iter().nth(row_to_check).expect("Should be String input_map").chars().nth(pos_x + 1) != Some('.')) {
        paper_count += 1;
    }
    return paper_count;

}
fn checkBelow(pos_x: usize, pos_y: usize, input_map: &Vec<String>) -> u32 {
    let mut paper_count = 0;
    if pos_y < input_map.iter().len() - 1 {
        let row_to_check = pos_y + 1;
        // Check left
        if pos_x != 0 && (input_map.iter().nth(row_to_check).expect("Should be String input_map").chars().nth(pos_x - 1) != Some('.')) {
            paper_count += 1;
        }
        // check central
        if input_map.iter().nth(row_to_check).expect("Should be String input_map").chars().nth(pos_x) != Some('.') {
            paper_count += 1;
        }
        // /check right
        if pos_x < input_map.iter().nth(row_to_check).expect("Should be String input_map").len() - 1 && (input_map.iter().nth(row_to_check).expect("Should be String input_map").chars().nth(pos_x + 1) != Some('.')) {
            paper_count += 1;
        }
    }
    return paper_count;

}


pub fn task1(file_input: Vec<String>) {
    println!("Executing Task 1!");
    let mut moveable_amount = 0;
    file_input.iter().for_each(|line| println!("{}", line));
    let mut heat_map = file_input.clone();

    for pos_y in 0..file_input.len() {
        for pos_x in 0..file_input.iter().nth(0).expect("Should be String Vector").len() {
            if file_input.iter().nth(pos_y).expect("Should be String Vector").chars().nth(pos_x) == Some('.') {
                continue
            }
            let paper_role_amount = checkAbove(pos_x, pos_y, &file_input) + checkSides(pos_x, pos_y, &file_input) + checkBelow(pos_x, pos_y, &file_input);
            // println!("(y|x) ({}|{}): {}", pos_y, pos_x, paper_role_amount);
            if paper_role_amount < 4 {
                moveable_amount += 1;
                heat_map[pos_y].replace_range(pos_x..pos_x + 1, "x");
            }
        }
    }
    println!("{} Piles are moveable!", moveable_amount);
    // heat_map.iter().for_each(|line| println!("{}", line));
}
