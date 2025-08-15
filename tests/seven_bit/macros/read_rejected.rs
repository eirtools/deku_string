/// Create read rejected tests
macro_rules! create_test_impl_read_rejected {
    (error: $error: ident, $( ($case: ident) ),+ $(,)?) => {
        create_test_impl_read_rejected!(underlying_type: u32, error: $error, $(($case)),+);
        create_test_impl_read_rejected!(underlying_type: u64, error: $error, $(($case)),+);
        create_test_impl_read_rejected!(underlying_type: u128, error: $error, $(($case)),+);
    };
    (underlying_type: $underlying_type: ident, error: $error: ident, $( ($case: ident) ),+ $(,)?) => {
        paste! {
            #[rstest]
            $(
            #[case::$case( &[<S7_ $underlying_type: upper _ $case: upper>] )]
            )+
            fn [<read_ $underlying_type _ $error _rejected>] (
                #[case] raw_data: &[u8],
            ) {
                let mut cursor = std::io::Cursor::new(raw_data);
                let mut deku_reader = Reader::new(&mut cursor);

                match [<SevenBit $underlying_type: upper>]::from_reader_with_ctx(&mut deku_reader, ()) {
                    Ok(value) => panic!("Error was expected, data has been read: {value:#?}"),
                    Err(value) => _rejected_check!(value, error: $error),
                }
            }
        }
    };
}

pub(crate) use create_test_impl_read_rejected;
