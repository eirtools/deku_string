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
                #[case] bytes: &[u8],
            ) {
                let ctx = _deku_ctx!(data: $data, ctx: $ctx, $layout, $endian);

                let error = assert_model_read_error::<
                        VecDeku<_data_type!(data: $data)>,
                        _deku_ctx_type!(data: $data, ctx: $ctx)>
                        (bytes, ctx);
                _rejected_check!(error, error: incomplete)
            }
        }
    };
}

pub(crate) use create_test_impl_read_rejected;
