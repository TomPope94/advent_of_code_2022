use itertools::Itertools;
use thiserror::Error;

#[derive(Debug, Error)]
enum Errors {
    #[error("No matching items found!")]
    NoItemMatch,
}

#[derive(Debug, Copy, Clone)]
struct Item {
    item_type: char,
    priority: u32,
}
impl Item {
    fn new(item_type: char) -> Self {
        Self {
            item_type,
            priority: calc_char_priority(item_type),
        }
    }
}

fn calc_char_priority(c: char) -> u32 {
    match c {
        'a'..='z' => c as u32 - 96,
        'A'..='Z' => c as u32 - 38,
        _ => 0,
    }
}

#[derive(Debug, Clone)]
struct Rucksack {
    compartment_one: Vec<Item>,
    compartment_two: Vec<Item>,
}
impl Rucksack {
    fn new(items: &str) -> Self {
        let items: Vec<Item> = items.chars().map(|c| Item::new(c)).collect();
        let mid_point = items.len() / 2; // May need to floor this??
        Self {
            compartment_one: items[..mid_point].to_vec(),
            compartment_two: items[mid_point..].to_vec(),
        }
    }

    fn find_in_both(&mut self) -> Result<Item, Errors> {
        // NOTE: this is asc... swap a and &b to change to desc (> needs to change below if you do that)
        self.compartment_one
            .sort_by(|a, b| a.priority.cmp(&b.priority));
        self.compartment_two
            .sort_by(|a, b| a.priority.cmp(&b.priority));
        let mut pointer_one = 0;
        let mut pointer_two = 0;
        while pointer_one < self.compartment_one.len() && pointer_two < self.compartment_two.len() {
            let item_one = self.compartment_one[pointer_one];
            let item_two = self.compartment_two[pointer_two];
            if item_one.priority == item_two.priority {
                return Ok(item_one);
            } else if item_one.priority > item_two.priority {
                pointer_two += 1;
            } else {
                pointer_one += 1;
            }
        }
        return Err(Errors::NoItemMatch);
    }
}

fn find_common_item(
    rs_one: Rucksack,
    rs_two: Rucksack,
    rs_three: Rucksack,
) -> Result<Item, Errors> {
    let mut combined_comps_one = [rs_one.compartment_one, rs_one.compartment_two].concat();
    let mut combined_comps_two = [rs_two.compartment_one, rs_two.compartment_two].concat();
    let mut combined_comps_three = [rs_three.compartment_one, rs_three.compartment_two].concat();
    combined_comps_one.sort_by(|a, b| a.priority.cmp(&b.priority));
    combined_comps_two.sort_by(|a, b| a.priority.cmp(&b.priority));
    combined_comps_three.sort_by(|a, b| a.priority.cmp(&b.priority));

    let mut pointer_one = 0;
    let mut pointer_two = 0;
    let mut pointer_three = 0;

    while pointer_one < combined_comps_one.len()
        && pointer_two < combined_comps_two.len()
        && pointer_three < combined_comps_three.len()
    {
        let item_one = combined_comps_one[pointer_one];
        let item_two = combined_comps_two[pointer_two];
        let item_three = combined_comps_three[pointer_three];

        if item_one.priority == item_two.priority && item_two.priority == item_three.priority {
            return Ok(item_one);
        }

        let priority_vector = vec![item_one.priority, item_two.priority, item_three.priority];
        let max_priority = priority_vector.iter().max().unwrap();

        if &item_one.priority < max_priority {
            pointer_one += 1;
        }
        if &item_two.priority < max_priority {
            pointer_two += 1;
        }
        if &item_three.priority < max_priority {
            pointer_three += 1;
        }
    }

    return Err(Errors::NoItemMatch);
}

fn main() {
    let items = include_str!("./rucksacks.txt");

    // PART ONE
    let common_items_sum: u32 = items
        .lines()
        .map(|rs| Rucksack::new(rs).find_in_both())
        .map(|item| item.expect("No common item").priority)
        .sum();

    println!("The sum of common items = {}", common_items_sum);

    // PART TWO
    let mut rucksacks = items
        .lines()
        .map(|rs| Rucksack::new(rs))
        .collect::<Vec<_>>();

    let groups = rucksacks.chunks_mut(3);
    let groups_sum: u32 = groups
        .map(|group| {
            find_common_item(group[0].clone(), group[1].clone(), group[2].clone())
                .expect("no common item found!")
        })
        .map(|item| item.priority)
        .sum();

    println!("The sum of badges = {}", groups_sum);
}
