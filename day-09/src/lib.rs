use std::iter::repeat;

#[derive(Clone, Copy)]
pub enum Memory {
    File(u32, usize),
    FreeSpace(usize),
}

impl Into<String> for &Memory {
    fn into(self) -> String {
        match self {
            Memory::File(id, len) => repeat(id)
                .take(*len)
                .map(|digit| digit.to_string())
                .collect(),
            Memory::FreeSpace(len) => repeat(".")
                .take(*len)
                .map(|digit| digit.to_string())
                .collect(),
        }
    }
}
