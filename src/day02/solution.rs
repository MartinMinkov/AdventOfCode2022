use std::collections::HashMap;
use std::fs::read_to_string;

pub fn solve() {
    let path = "./src/day02/input.txt";
    let input = read_to_string(path).expect("Should have been able to read the file");

    let total_points1 = calculate_points(&input, init_hashmap1());
    let total_points2 = calculate_points(&input, init_hashmap2());

    println!("Total points: {}", total_points1);
    println!("Total points: {}", total_points2);
}

fn calculate_points(input: &str, hashmap_outcomes: HashMap<&str, i32>) -> i32 {
    let mut total_points = 0;

    for line in input.split("\n") {
        let outcome = hashmap_outcomes
            .get(line)
            .expect("Could not find outcome in hashmap");
        total_points += outcome
    }
    total_points
}

pub fn init_hashmap1<'a>() -> HashMap<&'a str, i32> {
    let rock_points = 1;
    let paper_points = 2;
    let scissors_points = 3;

    let win_points = 6;
    let draw_points = 3;
    let lose_points = 0;

    let mut hashmap_outcomes = HashMap::new();
    // A = Rock, B = Paper, C = Scissors
    // X = Rock, Y = Paper, Z = Scissors
    hashmap_outcomes.insert("A X", draw_points + rock_points);
    hashmap_outcomes.insert("A Y", win_points + paper_points);
    hashmap_outcomes.insert("A Z", lose_points + scissors_points);

    hashmap_outcomes.insert("B X", lose_points + rock_points);
    hashmap_outcomes.insert("B Y", draw_points + paper_points);
    hashmap_outcomes.insert("B Z", win_points + scissors_points);

    hashmap_outcomes.insert("C X", win_points + rock_points);
    hashmap_outcomes.insert("C Y", lose_points + paper_points);
    hashmap_outcomes.insert("C Z", draw_points + scissors_points);

    hashmap_outcomes
}

pub fn init_hashmap2<'a>() -> HashMap<&'a str, i32> {
    let rock_points = 1;
    let paper_points = 2;
    let scissors_points = 3;

    let win_points = 6;
    let draw_points = 3;
    let lose_points = 0;

    let mut hashmap_outcomes = HashMap::new();
    // A = Rock, B = Paper, C = Scissors
    // X = Lose, Y = Draw, Z = Win
    hashmap_outcomes.insert("A X", lose_points + scissors_points);
    hashmap_outcomes.insert("A Y", draw_points + rock_points);
    hashmap_outcomes.insert("A Z", win_points + paper_points);

    hashmap_outcomes.insert("B X", lose_points + rock_points);
    hashmap_outcomes.insert("B Y", draw_points + paper_points);
    hashmap_outcomes.insert("B Z", win_points + scissors_points);

    hashmap_outcomes.insert("C X", lose_points + paper_points);
    hashmap_outcomes.insert("C Y", draw_points + scissors_points);
    hashmap_outcomes.insert("C Z", win_points + rock_points);

    hashmap_outcomes
}
