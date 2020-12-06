#![feature(iterator_fold_self)]

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod util;

fn main() {
    let day_one_input = util::file_to_set("input/01.txt");
    let day_two_input = util::file_to_vec("input/02.txt");
    let day_three_input = util::file_to_vec("input/03.txt");
    let day_four_input = util::file_to_string("input/04.txt");
    let day_five_input = util::file_to_vec("input/05.txt");
    let day_six_input = util::file_to_string("input/06.txt");

    println!(
        "Day 1, part 1: {}",
        day_01::part_one(&day_one_input).unwrap()
    );
    println!(
        "Day 1, part 2: {}",
        day_01::part_two(&day_one_input).unwrap()
    );
    println!("Day 2, part 1: {}", day_02::part_one(&day_two_input));
    println!("Day 2, part 2: {}", day_02::part_two(&day_two_input));
    println!("Day 3, part 1: {}", day_03::part_one(&day_three_input));
    println!("Day 3, part 2: {}", day_03::part_two(&day_three_input));
    println!("Day 4, part 1: {}", day_04::part_one(&day_four_input));
    println!("Day 4, part 2: {}", day_04::part_two(&day_four_input));
    println!("Day 5, part 1: {}", day_05::part_one(&day_five_input));
    println!(
        "Day 5, part 2: {}",
        day_05::part_two(&day_five_input).unwrap()
    );
    println!("Day 6, part 1: {}", day_06::part_one(&day_six_input));
    println!("Day 6, part 2: {}", day_06::part_two(&day_six_input));
}
