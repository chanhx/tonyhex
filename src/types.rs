enum CharType {
    Null,
    NonAscii,
    Space,
    PrintableChar,
    ControlChar,
}

impl CharType {
    fn new(b: u8) -> Self {
        match b {
            0x00 => CharType::Null,
            0x20 => CharType::Space,
            0x21..=0x7e => CharType::PrintableChar,
            0x01..=0x1f | 0x7f => CharType::ControlChar,
            _ => CharType::NonAscii,
        }
    }
}

pub(crate) struct DisplayChar {
    ch: u8,
    ty: CharType,
}

impl DisplayChar {
    pub fn new(b: u8) -> Self {
        DisplayChar {
            ch: b,
            ty: CharType::new(b),
        }
    }

    pub fn color(&self) -> u8 {
        match self.ty {
            CharType::Null => 37,
            CharType::Space => 92,
            CharType::PrintableChar => 96,
            CharType::ControlChar => 95,
            CharType::NonAscii => 93,
        }
    }
}

impl ToString for DisplayChar {
    fn to_string(&self) -> String {
        match self.ty {
            CharType::PrintableChar => format!("{}", self.ch as char),
            _ => ".".to_string(),
        }
    }
}
