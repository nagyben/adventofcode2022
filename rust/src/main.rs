mod d1;
mod d2;
mod d3;
mod d4;
use std::fs;

fn main() {
    println!("========================== Day 1 ==========================");
    let input = fs::read_to_string("src/d1/input.txt").unwrap();
    println!("most calories: {}", d1::get_most_calories(input.clone()));
    println!(
        "sum of top3 calories: {}",
        d1::get_top3_calories(input.clone())
    );
    println!();

    println!("========================== Day 2 ==========================");
    let input = fs::read_to_string("src/d2/input.txt").unwrap();
    println!(
        "[1] score from strategy: {}",
        d2::strategy(&input, &d2::part1_map)
    );
    println!(
        "[2] score from revised strategy: {}",
        d2::strategy(&input, &d2::part2_map)
    );
    println!();

    println!("========================== Day 3 ==========================");
    let input = fs::read_to_string("src/d3/input.txt").unwrap();
    println!(
        "[1] sum of priorities of common items: {}",
        d3::get_priority_sum(&input)
    );
    println!(
        "[2] sum of priorities of group badges: {}",
        d3::get_badge_priority_total(&input)
    );
    println!();

    println!("========================== Day 4 ==========================");
    let input = fs::read_to_string("src/d4/input.txt").unwrap();
    println!(
        "[1] fully contained assignment pairs: {}",
        d4::get_total_number_of_fully_contained_assignment_pairs(&input)
    );
    println!(
        "[1] overlapping assignment pairs: {}",
        d4::get_total_number_of_overlapping_assignment_pairs(&input)
    );

    println!();
}
