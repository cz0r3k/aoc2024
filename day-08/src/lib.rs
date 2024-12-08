pub struct Field {
    pub antenna: Option<char>,
    pub anti_node: bool,
}

pub fn print_map(map: &[Vec<Field>]) {
    map.iter()
        .for_each(|row| println!("{}", row.iter().map(Into::<char>::into).collect::<String>()));
}

impl Into<char> for &Field {
    fn into(self) -> char {
        match (self.antenna, self.anti_node) {
            (Some(antenna), _) => antenna,
            (_, true) => '#',
            (_, false) => '.',
        }
    }
}
