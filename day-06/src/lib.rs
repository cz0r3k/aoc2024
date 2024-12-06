use array2d::Array2D;

#[derive(Copy, Clone)]
pub struct Field {
    pub is_valid: bool,
    pub is_visited: bool,
}

impl Into<char> for &Field {
    fn into(self) -> char {
        if !self.is_valid {
            return '#';
        }
        if self.is_visited {
            return 'X';
        }
        '.'
    }
}

#[allow(unused)]
pub fn print_map_field(map: &Array2D<Field>) {
    map.as_rows().iter().for_each(|row| {
        println!(
            "{}",
            row.iter()
                .map(|field| {
                    let ch: char = field.into();
                    ch
                })
                .collect::<String>()
        );
    });
}

#[derive(Copy, Clone)]
pub struct NewField {
    pub is_valid: bool,
    pub directions: [bool; 4],
}

impl Into<char> for &NewField {
    fn into(self) -> char {
        if !self.is_valid {
            return '#';
        }
        match self.directions {
            [true, true, _, _] => '+',
            [true, _, _, true] => '+',
            [_, true, true, _] => '+',
            [_, _, true, true] => '+',
            [false, false, false, false] => '.',
            [_, false, _, false] => '|',
            [false, _, false, _] => '-',
        }
    }
}

#[allow(unused)]
pub fn print_map_new_field(map: &Array2D<NewField>) {
    map.as_rows().iter().for_each(|row| {
        println!(
            "{}",
            row.iter()
                .map(|field| {
                    let ch: char = field.into();
                    ch
                })
                .collect::<String>()
        );
    });
}

pub enum Direction {
    Left,
    Up,
    Right,
    Bottom,
}

pub fn new_position(position: (isize, isize), direction: &Direction) -> (isize, isize) {
    match direction {
        Direction::Left => (position.0 - 1, position.1),
        Direction::Up => (position.0, position.1 - 1),
        Direction::Right => (position.0 + 1, position.1),
        Direction::Bottom => (position.0, position.1 + 1),
    }
}

pub fn next_direction(direction: &Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Bottom,
        Direction::Bottom => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

pub fn direction_to_usize(direction: &Direction) -> usize {
    match direction {
        Direction::Up => 0,
        Direction::Right => 1,
        Direction::Bottom => 2,
        Direction::Left => 3,
    }
}
