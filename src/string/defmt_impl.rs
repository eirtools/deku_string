use crate::StringDeku;
use defmt::{Format, Formatter};

impl Format for StringDeku {
    fn format(&self, fmt: Formatter) {
        defmt::write!(fmt, "{}", self.0.as_str());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CONTENT: &str = "test_content";
    const EXPECTED: [u8; 26] = [
        0, 0, 1, 0, 2, 0, 3, 0, 12, 0, 0, 0, 116, 101, 115, 116, 95, 99, 111, 110, 116,
        101, 110, 116, 0, 0,
    ];

    #[test]
    fn format_string_deku() {
        let string_deku: StringDeku = TEST_CONTENT.into();

        defmt::println!("{}", string_deku);
        let output = defmt::export::fetch_bytes();
        assert_eq!(output, EXPECTED);
    }
}
