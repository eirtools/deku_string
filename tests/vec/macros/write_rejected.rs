use super::misc;

macro_rules! _create_test_impl_write_rejected_size {
    (error: io, $layout:ident, $case:ident) => {
        paste! {[<IO_ $layout:upper _ $case:upper _SIZE>]}
    };
    (error: $error:ident, $layout:ident, $case:ident) => {
        999999
    };
}

/// Create write rejected tests
macro_rules! create_test_impl_write_rejected {
    ($layout:ident, error: $error: ident, $(($case:ident)),+ $(,)?) => {
        create_test_impl_write_rejected!(
            $layout, data: u8, error: $error, $(($case)),+
        );

        create_test_impl_write_rejected!(
            $layout, data: str, error: $error, $(($case)),+
        );
    };
    ($layout:ident, data: $data: ident, error: $error: ident, $(($case:ident)),+ $(,)?) => {
        create_test_impl_write_rejected!(
            $layout, data: $data, endian: little, ctx: prime, error: $error, $(($case)),+
        );
        create_test_impl_write_rejected!(
            $layout, data: $data, endian: big, ctx: prime, error: $error, $(($case)),+
        );
        create_test_impl_write_rejected!(
            $layout, data: $data, endian: little, ctx: alt, error: $error, $(($case)),+
        );
        create_test_impl_write_rejected!(
            $layout, data: $data, endian: big, ctx: alt, error: $error, $(($case)),+
        );
    };
    (
        $layout:ident,
        data: $data: ident,
        endian: $endian: ident,
        ctx: $ctx: ident,
        error: $error: ident,
        $(($case:ident)),+
        $(,)?
    ) => {
        paste!{
            #[rstest]
            $(
                #[case::$case(paste!{[<$error:upper _ $data:upper _ $case:upper>]}, _create_test_impl_write_rejected_size!(error: $error, $layout, $case))]
            )+
            fn [<write_ $data _ $layout _ $endian _ctx_ $ctx _ $error _rejected>] (
                #[case] raw_data_static: &[_data_type_static!(data: $data)],
                #[case] byte_breaks: u64,
            ) {
                let raw_data_vec: Vec< _data_type!(data: $data) > = _data_convert!(data: $data, raw_data_static);
                let raw_data = VecDeku::new(&raw_data_vec);

                let mut output = InvalidBufferType::new(byte_breaks);
                let mut deku_writer = Writer::new(&mut output);
                let ctx = _deku_ctx!(data: $data, ctx: $ctx, $layout, $endian);

                let value = raw_data
                    .to_writer(&mut deku_writer, ctx)
                    .expect_err("Error was expected, data has been written");
                _rejected_check!(value, error: $error);
            }
        }
    };
}

pub(crate) use {
    _create_test_impl_write_rejected_size, create_test_impl_write_rejected,
};
