//!
//! Macros rules to generate most of the tests.
//!
//! Theoretically, almost all tests can be generated
//! from few tables to improve consistency and readability.

/// Encoding value for Deku context
///
#[macro_export]
macro_rules! _ctx_encoding {
    (encoding: utf_8) => {
        Encoding::Utf8
    };
    (encoding: utf_16) => {
        Encoding::Utf16
    };
}

/// Endian value for Deku context
#[macro_export]
macro_rules! _ctx_endian {
    (endian: little) => {
        Endian::Little
    };
    (endian: big) => {
        Endian::Big
    };
}

/// Match value for all supported errors
#[macro_export]
macro_rules! _match_error {
    (error: assertion) => {
        deku::DekuError::Assertion(_)
    };
    (error: parse) => {
        deku::DekuError::Parse(_)
    };
    (error: incomplete) => {
        deku::DekuError::Incomplete(deku::error::NeedSize { .. })
    };
    (error: io) => {
        deku::DekuError::Io(_)
    };
}

/// Assert for given error
#[macro_export]
macro_rules! _rejected_check {
    ($value: ident, error: $error: ident) => {
        assert!(
            matches!($value, _match_error!(error: $error)),
            "value doesn't match! {:#?}",
            $value
        )
    };
}

/// Actual test implementation for read accepted test
#[macro_export]
macro_rules! create_test_impl_read_accepted {
    ($layout:ident, endian: $endian: ident, encoding: $encoding: ident,  $( ($case:ident, $original_bytes:expr, $string_value:expr) ),+ $(,)?) => {
        paste! {
            #[rstest]
            $(
            #[case::$case($original_bytes, $string_value)]
            )+
            fn [<read_ $encoding _ $layout _ $endian _accepted>] (
            #[case] raw_data: &[u8],
            #[case] expected_string: &str,
            ) {
                let mut cursor = std::io::Cursor::new(raw_data);
                let mut deku_reader = Reader::new(&mut cursor);
                let ctx = (_ctx_endian!(endian: $endian), _ctx_encoding!(encoding: $encoding), paste! { [<LAYOUT_ $layout:upper>] });

                match StringDeku::from_reader_with_ctx(&mut deku_reader, ctx) {
                    Err(err) => panic!("Unable to read data: {err:#?}"),
                    Ok(value) => assert_eq!(value, expected_string),
                };
            }
        }
    };
}

/// Actual test implementation for write accepted test
#[macro_export]
macro_rules! create_test_impl_write_accepted {
    ($layout:ident, endian: $endian: ident, encoding: $encoding: ident, $( ($case:ident, $string_value:expr, $target_bytes:expr) ),+ $(,)?) => {
        paste! {
            #[rstest]
            $(
            #[case::$case($string_value, $target_bytes)]
            )+
            fn [<write_ $encoding _ $layout _ $endian _accepted>] (
                #[case] string: &str,
                #[case] expected_data: &[u8],
            ) {
                let raw_data: StringDeku = string.into();

                let mut output = Vec::new();
                let mut cursor = no_std_io::Cursor::new(&mut output);
                let mut deku_writer = Writer::new(&mut cursor);
                let ctx = (_ctx_endian!(endian: $endian), _ctx_encoding!(encoding: $encoding), paste! { [<LAYOUT_ $layout:upper>] });

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
#[macro_export]
macro_rules! _create_test_impl_rw_accepted_internal{
    ($layout:ident, endian: $endian: ident, encoding: $encoding: ident, $( ($case:ident, $original_bytes:expr, $string_value:expr, $target_bytes:expr) ),+ $(,)?) => {
        create_test_impl_read_accepted!(
            $layout, endian: $endian, encoding: $encoding, $(($case, $original_bytes, $string_value)),+
        );
        create_test_impl_write_accepted!(
            $layout, endian: $endian, encoding: $encoding, $(($case, $string_value, $target_bytes)),+
        );
    };
}

/// Generate read and write tests, parameters will be generated
#[macro_export]
macro_rules! create_test_impl_rw_accepted {
    (@accum $layout:ident, endian: $endian: ident, encoding: $encoding: ident, ($($out:tt)+) $(,)?) => {
        _create_test_impl_rw_accepted_internal!(
            $layout, endian: $endian, encoding: $encoding, $($out)+
        );
    };
    // Handle case: in -> str -> in, (rest)
    (@accum $layout:ident, endian: $endian: ident, encoding: $encoding: ident, ($($out:tt)*), ($case:ident, in_str_in), $($rest:tt)+) => {
        create_test_impl_rw_accepted!(
            @accum $layout, endian: $endian, encoding: $encoding,
            ($($out)* (
                $case,
                paste! { [<$encoding:upper _ $layout:upper _ $endian:upper _ $case:upper _IN>] },
                paste! { [<$layout:upper _ $case:upper _STR>] },
                paste! { [<$encoding:upper _ $layout:upper _ $endian:upper _ $case:upper _IN>] }
            ),),
            $($rest)*
        );
    };
    // Handle case: in -> str -> out, (rest)
    (@accum $layout:ident, endian: $endian: ident, encoding: $encoding: ident, ($($out:tt)*), ($case:ident, in_str_out), $($rest:tt)+) => {
        create_test_impl_rw_accepted!(
            @accum $layout, endian: $endian, encoding: $encoding,
            ($($out)* (
                $case,
                paste! { [<$encoding:upper _ $layout:upper _ $endian:upper _ $case:upper _IN>] },
                paste! { [<$layout:upper _ $case:upper _STR>] },
                paste! { [<$encoding:upper _ $layout:upper _ $endian:upper _ $case:upper _OUT>] }
            ),),
            $($rest)*
        );
    };
    // Handle case: in -> str -> in
    (@accum $layout:ident, endian: $endian: ident, encoding: $encoding: ident, ($($out:tt)*), ($case:ident, in_str_in)) => {
        create_test_impl_rw_accepted!(
            @accum $layout, endian: $endian, encoding: $encoding,
            ($($out)* (
                $case,
                paste! { [<$encoding:upper _ $layout:upper _ $endian:upper _ $case:upper _IN>] },
                paste! { [<$layout:upper _ $case:upper _STR>] },
                paste! { [<$encoding:upper _ $layout:upper _ $endian:upper _ $case:upper _IN>] }
            ),)
        );
    };
    // Handle case: in -> str -> out
    (@accum $layout:ident, endian: $endian: ident, encoding: $encoding: ident, ($($out:tt)*), ($case:ident, in_str_out)) => {
        create_test_impl_rw_accepted!(
            @accum $layout, endian: $endian, encoding: $encoding,
            ($($out)* (
                $case,
                paste! { [<$encoding:upper _ $layout:upper _ $endian:upper _ $case:upper _IN>] },
                paste! { [<$layout:upper _ $case:upper _STR>] },
                paste! { [<$encoding:upper _ $layout:upper _ $endian:upper _ $case:upper _OUT>] }
            ))
        );
    };
    // Entry point
    ($layout:ident, endian: $endian: ident, encoding: $encoding: ident, $($rest:tt)*) => {
        create_test_impl_rw_accepted!(@accum $layout, endian: $endian, encoding: $encoding, (), $($rest)*);
    };

    // Entry point (both cases of encoding and endian)
    ($layout:ident, all_encodings, $($rest:tt)*) => {
        create_test_impl_rw_accepted!(@accum $layout, endian: little, encoding: utf_8, (), $($rest)*);
        create_test_impl_rw_accepted!(@accum $layout, endian: little, encoding: utf_16, (), $($rest)*);
        create_test_impl_rw_accepted!(@accum $layout, endian: big, encoding: utf_8, (), $($rest)*);
        create_test_impl_rw_accepted!(@accum $layout, endian: big, encoding: utf_16, (), $($rest)*);
    };
}

/// Create read rejected tests
#[macro_export]
macro_rules! create_test_impl_read_rejected {
    // All layouts, all endian, all encodings
    (error: $error: ident, $( ($case:ident) ),+ $(,)?) => {
        create_test_impl_read_rejected!(encoding: utf_8, error: $error, $(($case)),+);
        create_test_impl_read_rejected!(encoding: utf_16, error: $error, $(($case)),+);
    };
    // All layouts, all endian
    (encoding: $encoding: ident, error: $error: ident, $( ($case:ident) ),+ $(,)?) => {
        create_test_impl_read_rejected!(fixed_force_zero, encoding: $encoding, error: $error, $(($case)),+);
        create_test_impl_read_rejected!(fixed_allow_no_zero, encoding: $encoding, error: $error, $(($case)),+);
        create_test_impl_read_rejected!(prefix_u8, encoding: $encoding, error: $error, $(($case)),+);
        create_test_impl_read_rejected!(prefix_u16, encoding: $encoding, error: $error, $(($case)),+);
        create_test_impl_read_rejected!(prefix_u32, encoding: $encoding, error: $error, $(($case)),+);
        create_test_impl_read_rejected!(zero_ended, encoding: $encoding, error: $error, $(($case)),+);
    };
    // given layout, all endian, all encodings
    ( $layout: ident, error: $error: ident, $( ($case:ident) ),+ $(,)?) => {
        create_test_impl_read_rejected!($layout, encoding: utf_8, error: $error, $(($case)),+);
        create_test_impl_read_rejected!($layout, encoding: utf_16, error: $error, $(($case)),+);
    };
    // given layout, all endian, given encoding. Generate test case from name
    ( $layout: ident, encoding: $encoding: ident, error: $error: ident, $( ($case:ident) ),+ $(,)?) => {
        create_test_impl_read_rejected!($layout, endian: little, encoding: $encoding, error: $error, $(($case, paste! { [<$encoding:upper _ $layout:upper _LITTLE_ $case:upper>] })),+);
        create_test_impl_read_rejected!($layout, endian: big, encoding: $encoding, error: $error, $(($case, paste! { [<$encoding:upper _ $layout:upper _BIG_ $case:upper>] })),+);
    };
    // given layout, given endian, given encoding. Generate test case from name
    ( $layout: ident, endian: $endian: ident, encoding: $encoding: ident, error: $error: ident, $( ($case:ident) ),+ $(,)?) => {
        create_test_impl_read_rejected!($layout, endian: $endian, encoding: $encoding, error: $error, $(($case, paste! { [<$layout:upper _ $endian:upper _ $case:upper>] })),+);
    };
    // generate actual function for given layout, given endian, given encoding and given exact test cases
    ( $layout:ident, endian: $endian: ident, encoding: $encoding: ident, error: $error: ident, $( ($case:ident, $original_bytes:expr) ),+ $(,)?) => {
        paste! {
            #[rstest]
            $(
            #[case::$case($original_bytes)]
            )+
            fn [<read_ $encoding _ $layout _ $endian _ $error _rejected>] (
                #[case] raw_data: &[u8],
            ) {
                let mut cursor = std::io::Cursor::new(raw_data);
                let mut deku_reader = Reader::new(&mut cursor);
                let ctx = (_ctx_endian!(endian: $endian), _ctx_encoding!(encoding: $encoding), paste! { [<LAYOUT_ $layout:upper>] });

                match StringDeku::from_reader_with_ctx(&mut deku_reader, ctx) {
                    Ok(value) => panic!("Error was expected, data has been read: {value:#?}"),
                    Err(value) => _rejected_check!(value, error: $error),
                }
            }
        }
    };
}

/// Create write rejected tests
#[macro_export]
macro_rules! create_test_impl_write_rejected {
    // given layout, all endian, all encodings, actual value
    ($layout:ident, $( ($case:ident, $string_value:expr) ),+ $(,)?) => {
        create_test_impl_write_rejected!($layout, endian: little, encoding: utf_8, paste! { [<LAYOUT_ $layout:upper>] }, $( ($case, $string_value) ), +);
        create_test_impl_write_rejected!($layout, endian: big, encoding: utf_8, paste! { [<LAYOUT_ $layout:upper>] }, $( ($case, $string_value) ), +);
        create_test_impl_write_rejected!($layout, endian: little, encoding: utf_16, paste! { [<LAYOUT_ $layout:upper>] }, $( ($case, $string_value) ), +);
        create_test_impl_write_rejected!($layout, endian: big, encoding: utf_16, paste! { [<LAYOUT_ $layout:upper>] }, $( ($case, $string_value) ), +);
    };

    // generate actual function for given layout, given endian, given encoding and given exact test cases
    ($fn:ident, endian: $endian: ident, encoding: $encoding: ident, $layout:expr, $( ($case:ident, $string_value:expr) ),+ $(,)?) => {
        paste! {
            #[rstest]
            $(
            #[case::$case($string_value)]
            )+
            fn [<write_ $encoding _ $fn _ $endian _rejected>] (
                #[case] string: &str,
            ) {
                let raw_data: StringDeku = string.into();

                let mut output = Vec::new();
                let mut cursor = no_std_io::Cursor::new(&mut output);
                let mut deku_writer = Writer::new(&mut cursor);
                let ctx = (_ctx_endian!(endian: $endian), _ctx_encoding!(encoding: $encoding), $layout);

                match raw_data.to_writer(&mut deku_writer, ctx) {
                    Ok(_) => panic!("Error was expected, data has been written: {string:#?}"),
                    Err(value) => _rejected_check!(value, error: assertion),
                }
            }
        }
    };
}

/// Create write io/rejected tests (special case)
#[macro_export]
macro_rules! create_test_impl_write_io_rejected {
    ($layout:ident) => {
        create_test_impl_write_io_rejected!($layout, endian: little, encoding: utf_8, paste! { [<LAYOUT_ $layout:upper>] });
        create_test_impl_write_io_rejected!($layout, endian: big, encoding: utf_8, paste! { [<LAYOUT_ $layout:upper>] });
        create_test_impl_write_io_rejected!($layout, endian: little, encoding: utf_16, paste! { [<LAYOUT_ $layout:upper>] });
        create_test_impl_write_io_rejected!($layout, endian: big, encoding: utf_16, paste! { [<LAYOUT_ $layout:upper>] });
    };

    ($fn:ident,  endian: $endian: ident, encoding: $encoding: ident, $layout:expr) => {
        paste! {
            #[rstest]
            fn [<write_ $encoding _ $fn _ $endian _io_rejected>] () {
                let raw_data: StringDeku = "valid data".into();

                let mut output: InvalidBufferType = InvalidBufferType{};
                let mut deku_writer = Writer::new(&mut output);
                let ctx = (_ctx_endian!(endian: $endian), _ctx_encoding!(encoding: $encoding), $layout);

                match raw_data.to_writer(&mut deku_writer, ctx) {
                    Ok(_) => panic!("Error was expected, data has been written"),
                    Err(value) => _rejected_check!(value, error: io),
                }
            }
        }
    };
}

///
/// Generates 4k repetitions of given expression.
/// Currently it's used to create a static string with length more than 64k
///
/// External file may be used.
#[macro_export]
macro_rules! rep64k {
    ($t:expr, 4) => {
        concat!($t, $t, $t, $t)
    };
    ($t:expr, 16) => {
        concat!(
            rep64k!($t, 4),
            rep64k!($t, 4),
            rep64k!($t, 4),
            rep64k!($t, 4)
        )
    };
    ($t:expr, 64) => {
        concat!(
            rep64k!($t, 16),
            rep64k!($t, 16),
            rep64k!($t, 16),
            rep64k!($t, 16)
        )
    };
    ($t:expr, 256) => {
        concat!(
            rep64k!($t, 64),
            rep64k!($t, 64),
            rep64k!($t, 64),
            rep64k!($t, 64)
        )
    };
    ($t:expr, 1024) => {
        concat!(
            rep64k!($t, 256),
            rep64k!($t, 256),
            rep64k!($t, 256),
            rep64k!($t, 256)
        )
    };
    ($t:expr, 4096) => {
        concat!(
            rep64k!($t, 1024),
            rep64k!($t, 1024),
            rep64k!($t, 1024),
            rep64k!($t, 1024)
        )
    };
}
