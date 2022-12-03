mod solutions;
use solutions::day1;
use solutions::day2;

fn main() {
    let input = std::fs::read_to_string("data/input1.txt").expect("no input file found");
    let summary = day1::get_calories_summary(&input);
    let max_calories = day1::get_max_calories(&summary);
    let max_three_sum = day1::count_max_three_sum(&summary);
    println!("=== Day 1:");
    println!("max calories: {}", max_calories);
    println!("max three sum: {}", max_three_sum);

    let input = std::fs::read_to_string("data/input2.txt").expect("no input file found");
    let game_result = day2::get_result(&input);
    let game_result2 = day2::get_result2(&input);
    println!("=== Day 2:");
    println!("game result strategy1: {}", game_result);
    println!("game result strategy2: {}", game_result2);
}
