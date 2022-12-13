use std::vec;

fn main() {
    let data = include_str!("./calories.txt");

    let lines = data.lines().map(|v| v.parse::<u32>().ok());

    let mut elf_totals: Vec<u32> = vec![];
    let mut current_elf: Vec<u32> = vec![];
    for line in lines {
        if line.is_some() {
            current_elf.push(line.unwrap());
        } else {
            elf_totals.push(current_elf.iter().sum());
            current_elf = vec![];
        }
    }

    let max_calories = elf_totals.iter().max();
    println!("Max calories = {}", max_calories.expect("msg"));

    elf_totals.sort();
    let len = elf_totals.len();
    let max = elf_totals[len - 1];
    let max_1 = elf_totals[len - 2];
    let max_2 = elf_totals[len - 3];

    println!("Top 3 total = {}", max + max_1 + max_2);
}
