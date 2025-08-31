use super::misc;

/// Actual test implementation for read accepted test
macro_rules! _create_test_impl_read_accepted {
    (
        $layout:ident,
        data: $data: ident,
        endian: $endian: ident,
        $(($case:ident, $original_bytes:expr, $vec_value:expr) ),+
        $(,)?
    ) => {
        _create_test_impl_read_accepted!(
            $layout,
            data: $data,
            endian: $endian,
            ctx: prime,
            $(($case, $original_bytes, $vec_value) ),+
        );
        _create_test_impl_read_accepted!(
            $layout,
            data: $data,
            endian: $endian,
            ctx: alt,
            $(($case, $original_bytes, $vec_value) ),+
        );
    };
    (
        $layout:ident,
        data: $data: ident,
        endian: $endian: ident,
        ctx: $ctx: ident,
        $(($case:ident, $original_bytes:expr, $vec_value:expr) ),+
        $(,)?
    ) => {

        paste!{
            #[rstest]
            $(
            #[case::$case($original_bytes, $vec_value)]
            )+
            fn [<read_ $data _ $layout _ $endian _ctx_ $ctx _accepted>] (
                #[case] bytes: &[u8],
                #[case] expected_data_static: &[_data_type_static!(data: $data)],
            ) {
                let expected_model = _data_convert!(data: $data, expected_data_static).into();
                let ctx = _deku_ctx!(data: $data, ctx: $ctx, $layout, $endian);

                assert_model_read_ctx::<
                        VecDeku<_data_type!(data: $data)>,
                        _deku_ctx_type!(data: $data, ctx: $ctx)>
                        (bytes, &expected_model, ctx);
            }
        }
    };
}

/// Actual test implementation for write accepted test
macro_rules! _create_test_impl_write_accepted {
    (
        $layout:ident,
        data: $data: ident,
        endian: $endian: ident,
        $( ($case:ident, $vec_value:expr, $target_bytes:expr) ),+
        $(,)?
    ) => {
        _create_test_impl_write_accepted!(
            $layout,
            data: $data,
            endian: $endian,
            ctx: prime,
            $(($case, $vec_value, $target_bytes) ),+
        );
        _create_test_impl_write_accepted!(
            $layout,
            data: $data,
            endian: $endian,
            ctx: alt,
            $(($case, $vec_value, $target_bytes) ),+
        );
    };
    (
        $layout:ident,
        data: $data: ident,
        endian: $endian: ident,
        ctx: $ctx: ident,
        $( ($case:ident, $vec_value:expr, $target_bytes:expr) ),+
        $(,)?
    ) => {
        paste! {
            #[rstest]
            $(
            #[case::$case($vec_value, $target_bytes)]
            )+
            fn [<write_ $data _ $layout _ $endian _ctx_ $ctx _accepted>] (
                #[case] raw_data_static: &[_data_type_static!(data: $data)],
                #[case] expected_bytes: &[u8],
            ) {
                let raw_data_vec: Vec< _data_type!(data: $data) > = _data_convert!(data: $data, raw_data_static);
                let model = VecDeku::new(&raw_data_vec);
                let ctx = _deku_ctx!(data: $data, ctx: $ctx, $layout, $endian);
                assert_model_write_ctx(model, expected_bytes, ctx);
            }
        }
    };
}

macro_rules! _create_test_impl_rw_accepted_impl{
    ($layout:ident, data: $data: ident, endian: $endian: ident, $( ($case:ident, $in_vec_value:expr, $output_bytes: expr, $out_vec_value:expr) ),+ $(,)?) => {
        _create_test_impl_write_accepted!(
            $layout,
            data: $data,
            endian: $endian,
             $(($case, $in_vec_value, $output_bytes)),+
        );
        _create_test_impl_read_accepted!(
            $layout,
            data: $data,
            endian: $endian,
            $(($case, $output_bytes, $out_vec_value)),+
        );
    };
}

/// Generate data constant names
macro_rules! _rw_accepted_naming_impl {
    (_in_, $layout:ident, $endian:ident, $data:ident, $case:ident) => {
        paste! { [<$data:upper _ $case:upper _BUF_IN>] }
    };

    (_data_, $layout:ident, $endian:ident, $data:ident, $case:ident) => {
        paste! { [<$data:upper _ $layout:upper _ $endian:upper _ $case:upper _BUF>] }
    };

    (in_data_in, $layout:ident, $endian:ident, $data:ident, $case:ident) => {
        paste! { [<$data:upper _ $case:upper _BUF_IN>] }
    };

    (in_data_out, $layout:ident, $endian:ident, $data:ident, $case:ident) => {
        paste! { [<$data:upper _ $case:upper _BUF_OUT>] }
    };
}

/// Generate read and write tests, parameters will be generated
macro_rules! create_test_impl_rw_accepted {
    // Goal
    (@accum $layout:ident, data: $data: ident, endian: $endian: ident, ($($out:tt)+) $(,)?) => {
        _create_test_impl_rw_accepted_impl!(
            $layout, data: $data, endian: $endian, $($out)+
        );
    };
    // Handle case: in -> data -> in/out (last)
    (@accum $layout:ident, data: $data: ident, endian: $endian: ident, ($($out:tt)*), ($case:ident, $variant:ident) $(,)?) => {
        create_test_impl_rw_accepted!(
            @accum $layout, data: $data, endian: $endian,
            ($($out)* (
                $case,
                _rw_accepted_naming_impl!(_in_, $layout, $endian, $data, $case),
                _rw_accepted_naming_impl!(_data_, $layout, $endian, $data, $case),
                _rw_accepted_naming_impl!($variant, $layout, $endian, $data, $case)
            ),)
        );
    };
    // Handle case: in -> data -> in, (rest)
    (@accum $layout:ident, data: $data: ident, endian: $endian: ident, ($($out:tt)*), ($case:ident, $variant:ident), $($rest:tt)+) => {
        create_test_impl_rw_accepted!(
            @accum $layout, data: $data, endian: $endian,
            ($($out)* (
                $case,
                _rw_accepted_naming_impl!(_in_, $layout, $endian, $data, $case),
                _rw_accepted_naming_impl!(_data_, $layout, $endian, $data, $case),
                _rw_accepted_naming_impl!($variant, $layout, $endian, $data, $case)
            ),),
            $($rest)*
        );
    };
    // Entry point
    ($layout:ident, all_data, $($rest:tt)*) => {
        create_test_impl_rw_accepted!(@accum $layout, data: u8, endian: little, (), $($rest)*);
        create_test_impl_rw_accepted!(@accum $layout, data: u8, endian: big, (), $($rest)*);
        create_test_impl_rw_accepted!(@accum $layout, data: str, endian: little, (), $($rest)*);
        create_test_impl_rw_accepted!(@accum $layout, data: str, endian: big, (), $($rest)*);
    };

    ($layout:ident, data: $data: ident, $($rest:tt)*) => {
        create_test_impl_rw_accepted!(@accum $layout, data: $data, endian: little, (), $($rest)*);
        create_test_impl_rw_accepted!(@accum $layout, data: $data, endian: big, (), $($rest)*);
    };
}

pub(crate) use {
    _create_test_impl_read_accepted, _create_test_impl_rw_accepted_impl,
    _create_test_impl_write_accepted, _rw_accepted_naming_impl,
    create_test_impl_rw_accepted,
};
