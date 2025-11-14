include!(concat!(env!("OUT_DIR"), "/keypad_paths.rs"));

pub enum Keypad {
    Directional([[&'static str; 5]; 5]),
    Numeric([[&'static str; 11]; 11]),
}

impl Keypad {
    pub const DIRECTIONAL: Self = Self::Directional(DIRECTIONAL_PATHS);
    pub const NUMERIC: Self = Self::Numeric(NUMERIC_PATHS);

    pub fn path(&self, source: char, target: char) -> &'static str {
        match self {
            Self::Directional(paths) => {
                let start = Self::directional_char_to_index(source);
                let end = Self::directional_char_to_index(target);
                paths[start][end]
            }
            Self::Numeric(paths) => {
                let start = Self::numeric_char_to_index(source);
                let end = Self::numeric_char_to_index(target);
                paths[start][end]
            }
        }
    }

    const fn directional_char_to_index(direction: char) -> usize {
        match direction {
            'A' => 0,
            '^' => 1,
            '<' => 2,
            'v' => 3,
            '>' => 4,
            _ => panic!("Not a valid key!"),
        }
    }

    const fn numeric_char_to_index(key: char) -> usize {
        match key {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'A' => 10,
            _ => panic!("Not a valid key!"),
        }
    }
}
