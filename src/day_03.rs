enum MapItem {
    Empty,
    Tree,
}

struct Cursor {
    row_index: usize,
    column_index: usize,
    row_delta: usize,
    column_delta: usize,
}

impl Cursor {
    fn new(row_delta: usize, column_delta: usize) -> Cursor {
        Cursor {
            row_index: 0,
            column_index: 0,
            row_delta,
            column_delta,
        }
    }

    fn next(&mut self) {
        self.row_index += self.row_delta;
        self.column_index = (self.column_index + self.column_delta) % 31;
    }
}

fn to_map(rows: &[String]) -> Vec<Vec<MapItem>> {
    rows.iter()
        .map(|r| {
            r.chars()
                .map(|c| match c {
                    '.' => MapItem::Empty,
                    '#' => MapItem::Tree,
                    _ => panic!("Unknown character"),
                })
                .collect()
        })
        .collect()
}

pub fn part_one(rows: &[String]) -> i32 {
    let map = to_map(rows);

    let mut cursor = Cursor::new(1, 3);
    let mut trees_encountered = 0;

    while let Some(row) = &map.get(cursor.row_index) {
        match row.get(cursor.column_index) {
            Some(map_item) => match map_item {
                MapItem::Tree => trees_encountered += 1,
                MapItem::Empty => (),
            },
            None => panic!("Should not happen!"),
        }

        cursor.next();
    }

    trees_encountered
}

pub fn part_two(rows: &[String]) -> i64 {
    let map = to_map(rows);

    let mut cursors = vec![
        Cursor::new(1, 1),
        Cursor::new(1, 3),
        Cursor::new(1, 5),
        Cursor::new(1, 7),
        Cursor::new(2, 1),
    ];

    cursors
        .iter_mut()
        .map(|c| {
            let mut trees_encountered = 0;

            while let Some(row) = &map.get(c.row_index) {
                match row.get(c.column_index) {
                    Some(map_item) => match map_item {
                        MapItem::Tree => trees_encountered += 1,
                        MapItem::Empty => (),
                    },
                    None => panic!("Should not happen!"),
                }

                c.next();
            }

            trees_encountered
        })
        .product()
}
