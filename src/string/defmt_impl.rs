use crate::StringDeku;
use defmt::{Format, Formatter};

impl Format for StringDeku {
    fn format(&self, fmt: Formatter) {
        defmt::write!(fmt, "{}", self.0.as_str());
    }
}
