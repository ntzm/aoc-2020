use crate::util::file_to_vec;

pub fn part_one() -> Option<i32> {
    let nums: Vec<i32> = file_to_vec("input/1.txt");
    let target = 2020;

    for num in &nums {
        let find = target - num;
        if nums.contains(&find) {
            return Some(num * find);
        }
    }

    None
}

pub fn part_two() -> Option<i32> {
    let nums: Vec<i32> = file_to_vec("input/1.txt");
    let target = 2020;

    for a in &nums {
        for b in &nums {
            let find = target - (a + b);
            if nums.contains(&find) {
                return Some(a * b * find);
            }
        }
    }

    None
}
