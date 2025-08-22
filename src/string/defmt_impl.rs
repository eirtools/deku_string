use crate::StringDeku;
use defmt::{Format, Formatter};

impl Format for StringDeku {
    fn format(&self, fmt: Formatter) {
        defmt::write!(fmt, "{}", self.0.as_str());
    }
}

#[cfg(test)]
mod tests {
    use std::format;

    use super::*;

    const TEST_CONTENT: &str = "test_content";

    #[test]
    fn format_string_deku() {
        let string_deku = StringDeku::from(TEST_CONTENT);
        let formatter = defmt::Display2Format(&string_deku);
        let output = format!("{}", formatter);
        assert_eq!(output, TEST_CONTENT);
    }
}
