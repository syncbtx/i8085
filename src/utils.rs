use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataFormat {
    Bin,
    Dec,
    Hex,
}

pub struct FormattedByte(pub u8, pub DataFormat);

impl fmt::Display for FormattedByte {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.1 {
            DataFormat::Bin => write!(f, "{:08b}", self.0),
            DataFormat::Dec => write!(f, "{}", self.0),
            DataFormat::Hex => write!(f, "0x{:02X}", self.0),
        }
    }
}

pub struct FormattedWord(pub u16, pub DataFormat);

impl fmt::Display for FormattedWord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.1 {
            DataFormat::Bin => write!(f, "{:016b}", self.0),
            DataFormat::Dec => write!(f, "{}", self.0),
            DataFormat::Hex => write!(f, "0x{:04X}", self.0),
        }
    }
}

pub struct FormattedSlice<'a>(pub &'a [u8], pub DataFormat);

impl<'a> fmt::Display for FormattedSlice<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        for &byte in self.0 {
            if !first {
                write!(f, " ")?;
            }
            write!(f, "{}", FormattedByte(byte, self.1))?;
            first = false;
        }
        Ok(())
    }
}

pub trait ToFormatted {
    fn format(&self, format: DataFormat) -> String;
}

impl ToFormatted for u8 {
    fn format(&self, format: DataFormat) -> String {
        format!("{}", FormattedByte(*self, format))
    }
}

impl ToFormatted for u16 {
    fn format(&self, format: DataFormat) -> String {
        format!("{}", FormattedWord(*self, format))
    }
}

impl ToFormatted for [u8] {
    fn format(&self, format: DataFormat) -> String {
        format!("{}", FormattedSlice(self, format))
    }
}

impl ToFormatted for Vec<u8> {
    fn format(&self, format: DataFormat) -> String {
        format!("{}", FormattedSlice(self, format))
    }
}
