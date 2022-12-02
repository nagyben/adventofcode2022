mod d1;
mod d2;
use std::fs;

fn main() {

    println!("========================== Day 1 ==========================");
    let input = fs::read_to_string("src/d1/input.txt").unwrap();
    println!("most calories: {}", d1::get_most_calories(input.clone()));
    println!("sum of top3 calories: {}", d1::get_top3_calories(input.clone()));
    println!();
    println!("========================== Day 2 ==========================");
    let input = fs::read_to_string("src/d2/input.txt").unwrap();
    println!("[1] score from strategy: {}", d2::strategy(&input, &d2::part1_map));
    println!("[2] score from revised strategy: {}", d2::strategy(&input, &d2::part2_map));
}