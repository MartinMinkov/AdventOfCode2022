use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

type ElfCalories = u32;

pub fn solve() {
    let path = "./src/day01/input.txt";
    let input = read_input(path).unwrap();
    let mut elf_calories = get_elf_calories(input);
    elf_calories.sort();

    let max = elf_calories.iter().max().unwrap();
    println!("Max calories of top 1: {}", max);

    let sum = elf_calories.iter().rev().take(3).sum::<ElfCalories>();
    println!("Calories of top 3: {}", sum);
}

pub fn read_input(path: impl AsRef<Path>) -> io::Result<BufReader<File>> {
    let file = File::open(path)?;
    Ok(BufReader::new(file))
}

fn get_elf_calories(input: BufReader<File>) -> Vec<ElfCalories> {
    let mut elf_calories = Vec::new();
    let mut current_calories = 0;

    for line in input.lines() {
        let line = line.unwrap();
        if line == "" {
            elf_calories.push(current_calories);
            current_calories = 0;
        } else {
            let calories = line.parse::<ElfCalories>().unwrap();
            current_calories += calories;
        }
    }
    elf_calories
}
