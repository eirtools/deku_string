macro_rules! _create_test_impl_write_rejected_size {
    (error: io, $layout: ident, $case: ident) => {
        paste! {[<IO_ $layout: upper _ $case: upper _SIZE>]}
    };
    (error: $error: ident, $layout: ident, $case: ident) => {
        u64::MAX
    };
}

/// Create write rejected tests
macro_rules! create_test_impl_write_rejected {
    ($layout: ident, error: $error: ident, $(($case: ident)),+ $(,)?) => {
        create_test_impl_write_rejected!(
            $layout, endian: little, encoding: utf_8, ctx: prime, error: $error, $(($case)),+
        );
        create_test_impl_write_rejected!(
            $layout, endian: little, encoding: utf_16, ctx: prime, error: $error, $(($case)),+
        );
        create_test_impl_write_rejected!(
            $layout, endian: little, encoding: utf_32, ctx: prime, error: $error, $(($case)),+
        );
        create_test_impl_write_rejected!(
            $layout, endian: big, encoding: utf_8, ctx: prime, error: $error, $(($case)),+
        );
        create_test_impl_write_rejected!(
            $layout, endian: big, encoding: utf_16, ctx: prime, error: $error, $(($case)),+
        );
        create_test_impl_write_rejected!(
            $layout, endian: big, encoding: utf_32, ctx: prime, error: $error, $(($case)),+
        );
        create_test_impl_write_rejected!(
            $layout, endian: little, encoding: utf_8, ctx: alt, error: $error, $(($case)),+
        );
        create_test_impl_write_rejected!(
            $layout, endian: little, encoding: utf_16, ctx: alt, error: $error, $(($case)),+
        );
        create_test_impl_write_rejected!(
            $layout, endian: little, encoding: utf_32, ctx: alt, error: $error, $(($case)),+
        );
        create_test_impl_write_rejected!(
            $layout, endian: big, encoding: utf_8, ctx: alt, error: $error, $(($case)),+
        );
        create_test_impl_write_rejected!(
            $layout, endian: big, encoding: utf_16, ctx: alt, error: $error, $(($case)),+
        );
        create_test_impl_write_rejected!(
            $layout, endian: big, encoding: utf_32, ctx: alt, error: $error, $(($case)),+
        );
    };
    (
        $layout: ident,
        endian: $endian: ident,
        encoding: $encoding: ident,
        ctx: $ctx: ident,
        error: $error: ident,
        $(($case: ident)),+
        $(,)?
    ) => {
        paste!{
            #[rstest]
            $(
                #[case::$case(paste!{[<$error: upper _ $case: upper>]}, _create_test_impl_write_rejected_size!(error: $error, $layout, $case))]
            )+
            fn [<write_ $encoding _ $layout _ $endian _ctx_ $ctx _ $error _rejected>] (
                #[case] raw_data_static: &str,
                #[case] byte_breaks: u64,
            ) {
                let model: StringDeku = raw_data_static.into();
                let ctx = _deku_ctx!(ctx: $ctx, $endian, $encoding, $layout);

                let error = assert_model_write_error(model, ctx, byte_breaks);
                _rejected_check!(error, error: $error);
            }
        }
    };
}

pub(crate) use {
    _create_test_impl_write_rejected_size, create_test_impl_write_rejected,
};
