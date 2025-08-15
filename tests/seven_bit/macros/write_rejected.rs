/// Create write rejected tests
macro_rules! create_test_impl_write_rejected {
    ($(($case: ident)),+ $(,)?) => {
        create_test_impl_write_rejected!(underlying_type: u32, $(($case)),+);
        create_test_impl_write_rejected!(underlying_type: u64, $(($case)),+);
        create_test_impl_write_rejected!(underlying_type: u128, $(($case)),+);

    };
    (underlying_type: $underlying_type: ident, $(($case: ident)),+ $(,)?) => {
        paste!{
            #[rstest]
            $(
                #[case::$case([<S7_ $underlying_type: upper _ $case: upper>], [<S7_ $underlying_type: upper _ $case: upper _SIZE>])]
            )+
            fn [<write_ $underlying_type _ io _rejected>] (
                #[case] raw_data_static: $underlying_type,
                #[case] byte_breaks: u64,
            ) {
                let raw_data: [<SevenBit $underlying_type: upper>] = raw_data_static.into();

                let mut output = InvalidBufferType::new(byte_breaks);
                let mut deku_writer = Writer::new(&mut output);

                match raw_data.to_writer(&mut deku_writer, ()) {
                    Ok(_) => panic!("Error was expected, data has been written"),
                    Err(value) => _rejected_check!(value, error: io),
                }
            }
        }
    };
}
pub(crate) use create_test_impl_write_rejected;
