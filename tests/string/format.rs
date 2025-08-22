//!
//! Formatting tests for StringDeku

#[cfg(feature = "defmt")]
mod defmt {
    use ::deku_string::StringDeku;
    use rstest::rstest;

    const TEST_CONTENT: &str = "test_content";

    #[rstest]
    fn format_string_deku() {
        let string_deku = StringDeku::from(TEST_CONTENT);
        let formatted_wrapper = defmt::Display2Format(&string_deku);
        let output_string = format!("{}", formatted_wrapper);
        assert_eq!(output_string, TEST_CONTENT);
    }
}
