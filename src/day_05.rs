pub fn part_one(boarding_passes: &[String]) -> u32 {
    boarding_passes
        .iter()
        .map(|boarding_pass| {
            let mut rows: &[u32] = &(0..=127).into_iter().collect::<Vec<u32>>();
            let mut columns: &[u32] = &(0..=7).into_iter().collect::<Vec<u32>>();

            for char in boarding_pass.chars() {
                match char {
                    'F' => rows = &rows[..(rows.len() / 2)],
                    'B' => rows = &rows[(rows.len() / 2)..],
                    'L' => columns = &columns[..(columns.len() / 2)],
                    'R' => columns = &columns[(columns.len() / 2)..],
                    _ => panic!("Unknown character"),
                }
            }

            rows[0] * 8 + columns[0]
        })
        .max()
        .unwrap()
}

pub fn part_two(boarding_passes: &[String]) -> Option<u32> {
    let mut taken_seats: Vec<_> = boarding_passes
        .iter()
        .map(|boarding_pass| {
            let mut rows: &[u32] = &(0..=127).into_iter().collect::<Vec<u32>>();
            let mut columns: &[u32] = &(0..=7).into_iter().collect::<Vec<u32>>();

            for char in boarding_pass.chars() {
                match char {
                    'F' => rows = &rows[..(rows.len() / 2)],
                    'B' => rows = &rows[(rows.len() / 2)..],
                    'L' => columns = &columns[..(columns.len() / 2)],
                    'R' => columns = &columns[(columns.len() / 2)..],
                    _ => panic!("Unknown character"),
                }
            }

            (rows[0], columns[0])
        })
        .collect();

    taken_seats.sort_unstable();

    for pair in taken_seats.windows(2) {
        let seat = pair[0];
        let next_seat = pair[1];

        // handle roll over
        if seat.1 == 7 && next_seat.1 == 0 {
            continue;
        }

        if next_seat.1 != seat.1 + 1 {
            return Some(seat.0 * 8 + seat.1 + 1);
        }
    }

    None
}
