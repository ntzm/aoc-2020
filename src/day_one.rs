use std::collections::BTreeSet;

pub fn part_one(nums: &BTreeSet<i32>) -> Option<i32> {
    let target = 2020;

    for num in nums {
        let find = target - num;
        if nums.contains(&find) {
            return Some(num * find);
        }
    }

    None
}

pub fn part_two(nums: &BTreeSet<i32>) -> Option<i32> {
    let target = 2020;

    for a in nums {
        for b in nums {
            let find = target - (a + b);
            if nums.contains(&find) {
                return Some(a * b * find);
            }
        }
    }

    None
}
