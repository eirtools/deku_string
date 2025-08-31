/// Create read rejected tests
macro_rules! create_test_impl_read_rejected {
    // given layout, all endian, all encodings
    ( $layout: ident, error: $error: ident, $( ($case: ident) ),+ $(,)?) => {
        create_test_impl_read_rejected!($layout, encoding: utf_8, error: $error, $(($case)),+);
        create_test_impl_read_rejected!($layout, encoding: utf_16, error: $error, $(($case)),+);
        create_test_impl_read_rejected!($layout, encoding: utf_32, error: $error, $(($case)),+);
    };

    // given layout, all endian, given encoding. Generate test case from name
    ( $layout: ident, encoding: $encoding: ident, error: $error: ident, $( ($case: ident) ),+ $(,)?) => {
        create_test_impl_read_rejected!($layout, endian: little, encoding: $encoding, error: $error, $(($case)),+);
        create_test_impl_read_rejected!($layout, endian: big, encoding: $encoding, error: $error, $(($case)),+);
    };

    // given layout, given endian, given encoding. Generate test case from name
    ( $layout: ident, endian: $endian: ident, encoding: $encoding: ident, error: $error: ident, $( ($case: ident) ),+ $(,)?) => {
        create_test_impl_read_rejected!($layout, endian: $endian, encoding: $encoding, error: $error, ctx: prime, $(($case)),+);
        create_test_impl_read_rejected!($layout, endian: $endian, encoding: $encoding, error: $error, ctx: alt, $(($case)),+);
    };

    // generate actual function for given layout, given endian, given encoding and given exact test cases
    ( $layout: ident, endian: $endian: ident, encoding: $encoding: ident, error: $error: ident, ctx: $ctx: ident, $( ($case: ident) ),+ $(,)?) => {
        paste! {
            #[rstest]
            $(
            #[case::$case( [<$encoding: upper _ $layout: upper _ $endian: upper _ $case: upper>] )]
            )+
            fn [<read_ $encoding _ $layout _ $endian _ $error _ctx_ $ctx _rejected>] (
                #[case] bytes: &[u8],
            ) {
                let ctx = _deku_ctx!(ctx: $ctx, $endian, $encoding, $layout);

                let error = assert_model_read_error::<
                        StringDeku,
                        _deku_ctx_type!(ctx: $ctx)>
                        (bytes, ctx);
                _rejected_check!(error, error: $error)
            }
        }
    };
}

pub(crate) use create_test_impl_read_rejected;
