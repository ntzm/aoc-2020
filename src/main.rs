mod day_five;
mod day_four;
mod day_one;
mod day_three;
mod day_two;
mod util;

fn main() {
    let day_one_input = util::file_to_set("input/1.txt");
    let day_two_input = util::file_to_vec("input/2.txt");
    let day_three_input = util::file_to_vec("input/3.txt");
    let day_four_input = util::file_to_string("input/4.txt");
    let day_five_input = util::file_to_vec("input/5.txt");

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
    println!("Day 3, part 1: {}", day_three::part_one(&day_three_input));
    println!("Day 3, part 2: {}", day_three::part_two(&day_three_input));
    println!("Day 4, part 1: {}", day_four::part_one(&day_four_input));
    println!("Day 4, part 2: {}", day_four::part_two(&day_four_input));
    println!("Day 5, part 1: {}", day_five::part_one(&day_five_input));
    println!(
        "Day 5, part 2: {}",
        day_five::part_two(&day_five_input).unwrap()
    );
}
