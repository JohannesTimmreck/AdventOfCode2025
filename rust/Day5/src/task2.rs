fn extend_for_overlap(
    range: &std::ops::RangeInclusive<u64>,
    new_range: &std::ops::RangeInclusive<u64>,
) -> Result<std::ops::RangeInclusive<u64>, bool> {
    if range.contains(new_range.start())
        || range.contains(new_range.end())
        || new_range.contains(range.start())
        || new_range.contains(range.end())
    {
        let lowest_start: u64 = if range.start() <= new_range.start() {
            *range.start()
        } else {
            *new_range.start()
        };
        let highest_end: u64 = if range.end() >= new_range.end() {
            *range.end()
        } else {
            *new_range.end()
        };
        println!("{} {}", lowest_start, highest_end);
        return Ok(lowest_start..=highest_end);
    }
    println!("No intersection");
    return Err(false);
}

fn handle_fresh_ingredients_list(
    fresh_ingredients: &mut Vec<std::ops::RangeInclusive<u64>>,
    new_range: std::ops::RangeInclusive<u64>,
) {
    let mut got_absorbed = false;
    for it in 0..fresh_ingredients.len() {
        let overlap_result = extend_for_overlap(&fresh_ingredients[it], &new_range);
        match overlap_result {
            Ok(overlap_range) => {
                if let Some(pos) = fresh_ingredients
                    .iter()
                    .position(|x| *x == fresh_ingredients[it])
                {
                    fresh_ingredients.remove(pos);
                }
                got_absorbed = true;
                handle_fresh_ingredients_list(fresh_ingredients, overlap_range);
                break;
            }
            Err(_) => (),
        }
    }
    if !got_absorbed {
        fresh_ingredients.push(new_range);
    }
}

pub fn task2(file_input: Vec<String>) {
    println!("Executing Task 2!");
    let mut fresh_ingredients: Vec<std::ops::RangeInclusive<u64>> = vec![];

    for line in file_input.iter() {
        if line == "" {
            break;
        }
        let splitted_line: Vec<&str> = line.split('-').collect();
        let new_range = splitted_line[0]
            .parse()
            .expect("Ingredient Range should be Int")
            ..=splitted_line[1]
                .parse()
                .expect("Ingredient Range should be Int");
        handle_fresh_ingredients_list(&mut fresh_ingredients, new_range);
    }
    // combine ranges

    println!("{:?}", fresh_ingredients);
    let sum: u64 = fresh_ingredients
        .iter()
        .map(|r| r.end() - r.start() + 1)
        .sum();

    println!("{} are fresh", sum);
}
