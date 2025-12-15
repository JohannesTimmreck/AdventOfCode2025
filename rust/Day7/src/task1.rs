fn calculate_splits(map: &mut Vec<String>, cur_line: usize) -> u32 {
    let mut splits = 0;
    for cur_index in 0..map[cur_line].len() - 1 {
        if map[cur_line - 1].chars().nth(cur_index).unwrap() != '|' {
            continue;
        }
        let c = map[cur_line - 1].chars().nth(cur_index).unwrap();
        let cur_char = map[cur_line].chars().nth(cur_index).unwrap();
        if cur_char == '.' {
            map[cur_line].replace_range(cur_index..cur_index + 1, "|");
        } else if cur_char == '^' {
            map[cur_line].replace_range(cur_index - 1..cur_index, "|");
            map[cur_line].replace_range(cur_index + 1..cur_index + 2, "|");
            splits += 1;
        }
    }
    if cur_line + 1 == map.len() {
        return splits;
    }
    return splits + calculate_splits(map, cur_line + 1);
}

pub fn task1(file_input: Vec<String>) {
    println!("Executing Task 1!");
    let mut map = file_input.clone();
    let start = file_input[0].find('S').unwrap();
    map[1].replace_range(start..start + 1, "|");

    let amount_splits = calculate_splits(&mut map, 1);
    map.iter().for_each(|line| println!("{line}"));
    println!("Amount of Splits: {amount_splits}");
}
