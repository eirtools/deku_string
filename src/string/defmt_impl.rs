//! `defmt` implementations for [`crate::StringDeku`].
//!
//! it's more efficient than using [`defmt::Display2Format`].
use crate::StringDeku;
use defmt::{Format, Formatter, write as defmt_write};

impl Format for StringDeku {
    #[inline]
    fn format(&self, fmt: Formatter) {
        defmt_write!(fmt, "{}", self.0.as_str());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use defmt::export::fetch_bytes;
    use defmt::println as defmt_println;

    const TEST_CONTENT: &str = "test_content";
    const EXPECTED: [u8; 26] = [
        0, 0, 1, 0, 2, 0, 3, 0, 12, 0, 0, 0, 116, 101, 115, 116, 95, 99, 111, 110, 116,
        101, 110, 116, 0, 0,
    ];

    #[test]
    fn format_string_deku() {
        let string_deku: StringDeku = TEST_CONTENT.into();

        defmt_println!("{}", string_deku);
        let output = fetch_bytes();
        assert_eq!(output, EXPECTED);
    }
}
