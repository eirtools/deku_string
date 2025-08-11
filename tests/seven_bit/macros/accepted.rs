/// Actual test implementation for read accepted test
#[macro_export]
macro_rules! create_test_impl_read_accepted {
    ($(($case:ident, $original_bytes:expr, $underlying_value:expr)),+ $(,)?) => {
        create_test_impl_read_accepted!(underlying_type: u32, $(($case, $original_bytes, $underlying_value)),+);
        create_test_impl_read_accepted!(underlying_type: u64, $(($case, $original_bytes, $underlying_value)),+);
        create_test_impl_read_accepted!(underlying_type: u128, $(($case, $original_bytes, $underlying_value)),+);
    };
    (
        underlying_type: $underlying_type: ident,
        $(($case:ident, $original_bytes:expr, $underlying_value:expr)),+
        $(,)?
    ) => {
        paste! {
            #[rstest]
            $(
                #[case::$case(&$original_bytes, $underlying_value)]
            )+
            fn [<read_ $underlying_type _accepted>] (
                #[case] raw_data: &[u8],
                #[case] expected_value: $underlying_type,
            ) {
                let mut cursor = std::io::Cursor::new(raw_data);
                let mut deku_reader = Reader::new(&mut cursor);

                match [<SevenBit $underlying_type:upper>]::from_reader_with_ctx(&mut deku_reader, ()) {
                    Err(err) => panic!("Unable to read data: {err:#?}"),
                    Ok(value) => assert_eq!(value, expected_value),
                };
            }
        }
    };
}

/// Actual test implementation for write accepted test
#[macro_export]
macro_rules! create_test_impl_write_accepted {
    ($(($case:ident, $input_value:expr, $target_bytes:expr)),+ $(,)?) => {
        create_test_impl_write_accepted!(underlying_type: u32, $(($case, $string_value, $target_bytes)),+);
        create_test_impl_write_accepted!(underlying_type: u64, $(($case, $string_value, $target_bytes)),+);
        create_test_impl_write_accepted!(underlying_type: u128, $(($case, $string_value, $target_bytes)),+);
    };
    (
        underlying_type: $underlying_type: ident,
        $(($case:ident, $input_value:expr, $target_bytes:expr)),+
        $(,)?
    ) => {
        paste! {
            #[rstest]
            $(
            #[case::$case($input_value, &$target_bytes)]
            )+
            fn [<write_ $underlying_type _accepted>] (
                #[case] input_value: $underlying_type,
                #[case] expected_data: &[u8],
            ) {
                let raw_data: [<SevenBit $underlying_type:upper>] = input_value.into();

                let mut output = Vec::new();
                let mut cursor = no_std_io::Cursor::new(&mut output);
                let mut deku_writer = Writer::new(&mut cursor);

                match raw_data.to_writer(&mut deku_writer, ()){
                    Err(err) => panic!("Unable to write data: {err:#?}"),
                    Ok(()) => {
                        deku_writer.finalize().unwrap();
                        assert_eq!(output, expected_data);
                    }
                };
            }
        }
    };
}

/// Generate both read and write tests with given parameters
#[macro_export]
macro_rules! create_test_impl_rw_accepted{
    (
        underlying_type: $underlying_type: ident,
        $(($case:ident, $original_bytes:expr, $underlying_value:expr)),+
        $(,)?
    ) => {
        create_test_impl_read_accepted!(underlying_type: $underlying_type, $(($case, $original_bytes, $underlying_value)),+);
        create_test_impl_write_accepted!(underlying_type: $underlying_type, $(($case, $underlying_value, $original_bytes)),+);
    };
    (underlying_type: $underlying_type: ident, $(($case:ident)),+ $(,)?) => {
        create_test_impl_rw_accepted!(
            underlying_type: $underlying_type,
            $(($case, paste! { [<S7_ $underlying_type:upper _ $case:upper _OUT>] }, paste!{ [<S7_ $underlying_type:upper _ $case:upper _IN>] })),+
        );
    };
    (
        $(($case:ident)),+
        $(,)?
    ) => {
        create_test_impl_rw_accepted!(underlying_type: u32, $(($case)),+);
        create_test_impl_rw_accepted!(underlying_type: u64, $(($case)),+);
        create_test_impl_rw_accepted!(underlying_type: u128, $(($case)),+);
    };
}
