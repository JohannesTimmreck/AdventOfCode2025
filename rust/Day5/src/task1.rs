struct Ingredient {
    id: u64,
    fresh: bool,
    amount_of_lists: u64,
}

impl Ingredient {
    fn is_fresh(&mut self, range: &std::ops::RangeInclusive<u64>) {
        if range.contains(&self.id) {
            self.fresh = true;
            self.amount_of_lists += 1;
        }
    }
}

fn create_ingredient(id: u64) -> Ingredient {
    Ingredient {
        id,
        fresh: false,
        amount_of_lists: 0,
    }
}

pub fn task1(file_input: Vec<String>) {
    println!("Executing Task 1!");
    let mut range_block = true;
    let mut fresh_ingredients: u64 = 0;
    let mut ingrediant_ranges: Vec<std::ops::RangeInclusive<u64>> = vec![];
    let mut ingredients: Vec<Ingredient> = vec![];

    for line in file_input.iter() {
        if line == "" {
            range_block = false;
            continue;
        }
        if range_block {
            let splitted_line: Vec<&str> = line.split('-').collect();
            ingrediant_ranges.push(
                splitted_line[0]
                    .parse()
                    .expect("Ingredient Range should be Int")
                    ..=splitted_line[1]
                        .parse()
                        .expect("Ingredient Range should be Int"),
            );
        } else {
            let mut ingredient = create_ingredient(line.parse().expect(""));
            for range in &ingrediant_ranges {
                ingredient.is_fresh(range);
            }
            if ingredient.fresh {
                fresh_ingredients += 1
            }
            ingredients.push(ingredient);
        }
    }
    println!("{fresh_ingredients} are fresh");
}
