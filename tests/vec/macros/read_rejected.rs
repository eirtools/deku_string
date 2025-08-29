use super::misc;

/// Create write rejected tests
macro_rules! create_test_impl_read_rejected {
    // given layout, all endian, all encodings, actual value
    ($layout:ident, $( ($case:ident) ),+ $(,)?) => {
        create_test_impl_read_rejected!($layout, data: u8, endian: little, ctx: prime, $(($case)), +);
        create_test_impl_read_rejected!($layout, data: u8, endian: big, ctx: prime, $(($case)), +);
        create_test_impl_read_rejected!($layout, data: str, endian: little, ctx: prime, $(($case)), +);
        create_test_impl_read_rejected!($layout, data: str, endian: big, ctx: prime, $(($case)), +);

        create_test_impl_read_rejected!($layout, data: u8, endian: little, ctx: alt, $(($case)), +);
        create_test_impl_read_rejected!($layout, data: u8, endian: big, ctx: alt, $(($case)), +);
        create_test_impl_read_rejected!($layout, data: str, endian: little, ctx: alt, $(($case)), +);
        create_test_impl_read_rejected!($layout, data: str, endian: big, ctx: alt, $(($case)), +);
    };

    // generate actual function for given layout, given endian, given encoding and given exact test cases
    ($layout:ident, data: $data: ident, endian: $endian: ident, ctx: $ctx: ident, $( ($case:ident) ),+ $(,)?) => {
        paste! {
            #[rstest]
            $(
            #[case::$case(paste!{[<incomplete:upper _ $layout:upper _ $endian:upper _ $case:upper>]})]
            )+
            fn [<read_ $data _ $layout _ $endian _ctx_ $ctx _assertion_rejected>] (
                #[case] raw_data: &[u8],
            ) {
                let mut cursor = std::io::Cursor::new(raw_data);
                let mut deku_reader = Reader::new(&mut cursor);
                let ctx = _deku_ctx!(data: $data, ctx: $ctx, $layout, $endian);

                let value = <VecDeku<_data_type!(data: $data)>>
                    ::from_reader_with_ctx(&mut deku_reader, ctx)
                    .expect_err("Error was expected, data has been read");
                _rejected_check!(value, error: incomplete)
            }
        }
    };
}

pub(crate) use create_test_impl_read_rejected;
