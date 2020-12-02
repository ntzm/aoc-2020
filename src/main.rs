mod day_one;
mod day_two;
mod util;

fn main() {
    println!("Day 1, part 1: {}", day_one::part_one().unwrap());
    println!("Day 1, part 2: {}", day_one::part_two().unwrap());
    println!("Day 2, part 1: {}", day_two::part_one());
    println!("Day 2, part 2: {}", day_two::part_two());
}
