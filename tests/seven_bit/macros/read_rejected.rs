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
            #[case::$case( &[<S7_ $underlying_type: upper _ $error: upper _ $case: upper>] )]
            )+
            fn [<read_ $underlying_type _ $error _rejected>] (
                #[case] bytes: &[u8],
            ) {
                let error = assert_model_read_error::<[<SevenBit $underlying_type: upper>], ()>(bytes, ());
                _rejected_check!(error, error: $error)
            }
        }
    };
}

pub(crate) use create_test_impl_read_rejected;
