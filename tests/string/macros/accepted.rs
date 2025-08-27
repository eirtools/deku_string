/// Actual test implementation for read accepted test
macro_rules! create_test_impl_read_accepted {
    (
        $layout: ident,
        endian: $endian: ident,
        encoding: $encoding: ident,
        $(($case: ident, $original_bytes: expr, $string_value: expr)),+
        $(,)?
    ) => {
        create_test_impl_read_accepted!(
            $layout,
            endian: $endian,
            encoding: $encoding,
            ctx: prime,
            $(($case, $original_bytes, $string_value)),+
        );
        create_test_impl_read_accepted!(
            $layout,
            endian: $endian,
            encoding: $encoding,
            ctx: alt,
            $(($case, $original_bytes, $string_value)),+
        );
    };
    (
        $layout: ident,
        endian: $endian: ident,
        encoding: $encoding: ident,
        ctx: $ctx: ident,
        $(($case: ident, $original_bytes: expr, $string_value: expr)),+
        $(,)?
    ) => {
        paste! {
            #[rstest]
            $(
            #[case::$case($original_bytes, $string_value)]
            )+
            fn [<read_ $encoding _ $layout _ $endian _ctx_ $ctx _accepted>] (
            #[case] raw_data: &[u8],
            #[case] expected_string: &str,
            ) {
                let mut cursor = std::io::Cursor::new(raw_data);
                let mut deku_reader = Reader::new(&mut cursor);
                let ctx = _deku_ctx!(ctx: $ctx, $endian, $encoding, $layout);

                let value = StringDeku
                    ::from_reader_with_ctx(&mut deku_reader, ctx)
                    .expect("Unable to read data: {err:#?}");

                assert_eq!(value, expected_string);
            }
        }
    };
}

/// Actual test implementation for write accepted test
macro_rules! create_test_impl_write_accepted {
    (
        $layout: ident,
        endian: $endian: ident,
        encoding: $encoding: ident,
        $(($case: ident, $string_value: expr, $target_bytes: expr)),+
        $(,)?
    ) => {
        create_test_impl_write_accepted!(
            $layout,
            endian: $endian,
            encoding: $encoding,
            ctx: prime,
            $(($case, $string_value, $target_bytes)),+
        );
        create_test_impl_write_accepted!(
            $layout,
            endian: $endian,
            encoding: $encoding,
            ctx: alt,
            $(($case, $string_value, $target_bytes)),+
        );
    };
    (
        $layout: ident,
        endian: $endian: ident,
        encoding: $encoding: ident,
        ctx: $ctx: ident,
        $(($case: ident, $string_value: expr, $target_bytes: expr)),+
        $(,)?
    ) => {
        paste! {
            #[rstest]
            $(
            #[case::$case($string_value, $target_bytes)]
            )+
            fn [<write_ $encoding _ $layout _ $endian _ctx_ $ctx _accepted>] (
                #[case] string: &str,
                #[case] expected_data: &[u8],
            ) {
                let raw_data: StringDeku = string.into();

                let mut output = Vec::new();
                let mut cursor = no_std_io::Cursor::new(&mut output);
                let mut deku_writer = Writer::new(&mut cursor);
                let ctx = _deku_ctx!(ctx: prime, $endian, $encoding, $layout);

                match raw_data.to_writer(&mut deku_writer, ctx){
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
macro_rules! _create_test_impl_rw_accepted_internal{
    (
        $layout: ident,
        endian: $endian: ident,
        encoding: $encoding: ident,
        $(($case: ident, $original_bytes: expr, $string_value: expr, $target_bytes: expr)),+
        $(,)?
    ) => {
        create_test_impl_read_accepted!(
            $layout, endian: $endian, encoding: $encoding, $(($case, $original_bytes, $string_value)),+
        );
        create_test_impl_write_accepted!(
            $layout, endian: $endian, encoding: $encoding, $(($case, $string_value, $target_bytes)),+
        );
    };
}

macro_rules! _rw_accepted_naming_impl {
    (_in_, $layout: ident, $endian: ident, $encoding: ident, $case: ident) => {
        paste! { [<$encoding: upper _ $layout: upper _ $endian: upper _ $case: upper _IN>] }
    };

    (_data_, $layout: ident, $endian: ident, $encoding: ident, $case: ident) => {
        paste! { [<$layout: upper _ $case: upper _STR>] }
    };

    (in_data_in, $layout: ident, $endian: ident, $encoding: ident, $case: ident) => {
        paste! { [<$encoding: upper _ $layout: upper _ $endian: upper _ $case: upper _IN>] }
    };

    (in_data_out, $layout: ident, $endian: ident, $encoding: ident, $case: ident) => {
        paste! { [<$encoding: upper _ $layout: upper _ $endian: upper _ $case: upper _OUT>] }
    };
}

/// Generate read and write tests, parameters will be generated
macro_rules! create_test_impl_rw_accepted {
    (
        @accum $layout: ident,
        endian: $endian: ident,
        encoding: $encoding: ident,
        ($($out: tt)+)
        $(,)?
    ) => {
        _create_test_impl_rw_accepted_internal!(
            $layout, endian: $endian, encoding: $encoding, $($out)+
        );
    };
    // Handle case: in -> data -> in/out (last)
    (@accum $layout: ident, endian: $endian: ident, encoding: $encoding: ident, ($($out: tt)*), ($case: ident, $variant: ident) $(,)?) => {
        create_test_impl_rw_accepted!(
            @accum $layout, endian: $endian, encoding: $encoding,
            ($($out)* (
                $case,
                _rw_accepted_naming_impl!(_in_, $layout, $endian, $encoding, $case),
                _rw_accepted_naming_impl!(_data_, $layout, $endian, $encoding, $case),
                _rw_accepted_naming_impl!($variant, $layout, $endian, $encoding, $case)
            ),)
        );
    };
    // Handle case: in -> data -> in/out, (rest)
    (
        @accum $layout: ident,
        endian: $endian: ident,
        encoding: $encoding: ident,
        ($($out: tt)*),
        ($case: ident, $variant: ident),
        $($rest: tt)+
    ) => {
        create_test_impl_rw_accepted!(
            @accum $layout, endian: $endian, encoding: $encoding,
            ($($out)* (
                $case,
                _rw_accepted_naming_impl!(_in_, $layout, $endian, $encoding, $case),
                _rw_accepted_naming_impl!(_data_, $layout, $endian, $encoding, $case),
                _rw_accepted_naming_impl!($variant, $layout, $endian, $encoding, $case)
            ),),
            $($rest)*
        );
    };

    // Entry point (all cases of encoding and endian)
    ($layout: ident, all_encodings, $($rest: tt)*) => {
        create_test_impl_rw_accepted!(@accum $layout, endian: little, encoding: utf_8, (), $($rest)*);
        create_test_impl_rw_accepted!(@accum $layout, endian: little, encoding: utf_16, (), $($rest)*);
        create_test_impl_rw_accepted!(@accum $layout, endian: little, encoding: utf_32, (), $($rest)*);
        create_test_impl_rw_accepted!(@accum $layout, endian: big, encoding: utf_8, (), $($rest)*);
        create_test_impl_rw_accepted!(@accum $layout, endian: big, encoding: utf_16, (), $($rest)*);
        create_test_impl_rw_accepted!(@accum $layout, endian: big, encoding: utf_32, (), $($rest)*);
    };
}

pub(crate) use {
    _create_test_impl_rw_accepted_internal, _rw_accepted_naming_impl,
    create_test_impl_read_accepted, create_test_impl_rw_accepted,
    create_test_impl_write_accepted,
};
