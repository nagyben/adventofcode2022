mod d1;
use std::fs;

fn main() {
    let input = fs::read_to_string("src/d1/input.txt").unwrap();

    println!("most calories: {}", d1::get_most_calories(input.clone()));
    println!("sum of top3 calories: {}", d1::get_top3_calories(input));
}