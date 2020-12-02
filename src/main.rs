mod day_one;
mod day_two;
mod util;

fn main() {
    let day_one_input = util::file_to_set("input/1.txt");
    let day_two_input = util::file_to_vec("input/2.txt");

    for _ in 0..100 {
        println!(
            "Day 1, part 1: {}",
            day_one::part_one(&day_one_input).unwrap()
        );
        println!(
            "Day 1, part 2: {}",
            day_one::part_two(&day_one_input).unwrap()
        );
        println!("Day 2, part 1: {}", day_two::part_one(&day_two_input));
        println!("Day 2, part 2: {}", day_two::part_two(&day_two_input));
    }
}
