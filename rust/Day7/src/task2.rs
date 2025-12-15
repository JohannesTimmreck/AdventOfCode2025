#[derive(Debug)]
struct Node {
    pos: usize,
    val: u64,
}

fn create_node(pos: usize, val: u64) -> Node {
    return Node { pos: pos, val: val };
}

fn calculate_splits(map: &Vec<String>, cur_line: usize, splits: &mut Vec<Node>) {
    for cur_index in 0..map[cur_line].len() - 1 {
        let cur_char = map[cur_line].chars().nth(cur_index).unwrap();
        if cur_char == '^' {
            if cur_index + 1 < splits.len() {
                splits[cur_index + 1].val += splits[cur_index].val;
            }
            if cur_index > 0 {
                splits[cur_index - 1].val += splits[cur_index].val;
            }
            splits[cur_index].val = 0;
        }
    }
    let mut s = String::new();
    splits.iter().for_each(|line| {
        s.push('|');
        s.push_str(&line.val.to_string());
    });
    println!("{:?}", s);
    if cur_line + 1 == map.len() {
        return;
    }
    return calculate_splits(map, cur_line + 1, splits);
}

pub fn task2(file_input: Vec<String>) {
    println!("Executing Task 2!");
    let start = file_input[0].find('S').unwrap();
    let mut splits: Vec<Node> = vec![];
    for i in 0..file_input[0].len() {
        if i == start {
            splits.push(create_node(i, 1));
            continue;
        }
        splits.push(create_node(i, 0));
    }

    calculate_splits(&file_input, 1, &mut splits);

    let mut amount_splits = 0;
    let mut s = String::new();
    splits.iter().for_each(|line| {
        s.push('|');
        s.push_str(&line.val.to_string());
        amount_splits += line.val;
    });
    // println!("{:?}", s);
    println!("Amount of Splits: {amount_splits}");
}
