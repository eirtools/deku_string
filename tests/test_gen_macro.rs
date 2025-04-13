//!
//! Macros rules to generate most of the tests.
//!
//! Theoretically, almost all tests can be generated
//! from few tables to improve consistency and readability.
//!
#[macro_export]
macro_rules! ctx_encoding {
    (encoding: utf_8) => {
        Encoding::Utf8
    };
    (encoding: utf_16) => {
        Encoding::Utf16
    };
}

#[macro_export]
macro_rules! ctx_endian {
    (endian: little) => {
        Endian::Little
    };
    (endian: big) => {
        Endian::Big
    };
}

#[macro_export]
macro_rules! rejected_check {
    ($value: ident, $match_expr: pat) => {
        assert!(
            matches!($value, $match_expr),
            "value doesn't match! {:#?}",
            $value
        )
    };
}

#[macro_export]
macro_rules! create_test_impl_read_accepted {
    ($fn:ident, endian: $endian: ident, encoding: $encoding: ident, $layout:expr, $( ($case:ident, $original_bytes:expr, $string_value:expr) ),+ $(,)?) => {
        paste! {
            #[rstest]
            $(
            #[case::$case($original_bytes, $string_value)]
            )+
            fn [<read_ $encoding _ $fn _ $endian _accepted>] (
            #[case] raw_data: &[u8],
            #[case] expected_string: &str,
            ) {
                let mut cursor = std::io::Cursor::new(raw_data);
                let mut deku_reader = Reader::new(&mut cursor);
                let ctx = (ctx_endian!(endian: $endian), ctx_encoding!(encoding: $encoding), $layout);

                match StringDeku::from_reader_with_ctx(&mut deku_reader, ctx) {
                    Err(err) => assert!(false, "Unable to read data: {err:#?}"),
                    Ok(value) => assert_eq!(value, expected_string),
                };
            }
        }
    };
}

#[macro_export]
macro_rules! create_test_impl_write_accepted {
    ($fn:ident, endian: $endian: ident, encoding: $encoding: ident, $layout:expr, $( ($case:ident, $string_value:expr, $target_bytes:expr) ),+ $(,)?) => {
        paste! {
            #[rstest]
            $(
            #[case::$case($string_value, $target_bytes)]
            )+
            fn [<write_ $encoding _ $fn _ $endian _accepted>] (
                #[case] string: &str,
                #[case] expected_data: &[u8],
            ) {
                let raw_data: StringDeku = string.into();

                let mut output = Vec::new();
                let mut cursor = no_std_io::Cursor::new(&mut output);
                let mut deku_writer = Writer::new(&mut cursor);
                let ctx = (ctx_endian!(endian: $endian), ctx_encoding!(encoding: $encoding), $layout);

                match raw_data.to_writer(&mut deku_writer, ctx){
                    Err(err) =>assert!(false, "Unable to write data: {err:#?}"),
                    Ok(()) => {
                        deku_writer.finalize().unwrap();
                        assert_eq!(output, expected_data);
                    }
                };
            }
        }
    };
}

#[macro_export]
macro_rules! create_test_impl_rw_accepted {
    ($fn:ident, endian: $endian: ident, encoding: $encoding: ident, $layout:expr, $( ($case:ident, $original_bytes: expr, $string_value:expr, $target_bytes:expr) ),+ $(,)?) => {
         create_test_impl_read_accepted!(
            $fn, endian: $endian, encoding: $encoding, $layout, $(($case, $original_bytes, $string_value)),+
        );

        create_test_impl_write_accepted!(
            $fn, endian: $endian, encoding: $encoding, $layout, $(($case, $string_value, $target_bytes)),+
        );
     };

    ($fn:ident, endian: $endian: ident, encoding: $encoding: ident, $layout:expr, $( ($case:ident, $original_bytes: expr, $string_value:expr) ),+ $(,)?) => {
        create_test_impl_rw_accepted!(
            $fn, endian: $endian, encoding: $encoding, $layout, $(($case, $original_bytes, $string_value, $original_bytes)),+
        );
    };
}

#[macro_export]
macro_rules! create_test_impl_read_rejected {
    ( $fn:ident, endian: $endian: ident, encoding: $encoding: ident, $layout:expr, $match_expr: pat, $( ($case:ident, $original_bytes:expr) ),+ $(,)?) => {
        paste! {
            #[rstest]
            $(
            #[case::$case($original_bytes)]
            )+
            fn [<read_ $encoding _ $fn _ $endian _rejected>] (
                #[case] raw_data: &[u8],
            ) {
                let mut cursor = std::io::Cursor::new(raw_data);
                let mut deku_reader = Reader::new(&mut cursor);
                let ctx = (ctx_endian!(endian: $endian), ctx_encoding!(encoding: $encoding), $layout);

                match StringDeku::from_reader_with_ctx(&mut deku_reader, ctx) {
                    Ok(value) => assert!(false, "Error was expected, data has been read: {value:#?}"),
                    Err(value) => rejected_check!(value, $match_expr),
                }
            }
        }
    };
}

#[macro_export]
macro_rules! create_test_impl_write_rejected {
    ($fn:ident, $layout:expr, $( ($case:ident, $string_value:expr) ),+ $(,)?) => {
        create_test_impl_write_rejected!($fn, endian: little, encoding: utf_8, $layout, $( ($case, $string_value) ), +);
        create_test_impl_write_rejected!($fn, endian: big, encoding: utf_8, $layout, $( ($case, $string_value) ), +);
        create_test_impl_write_rejected!($fn, endian: little, encoding: utf_16, $layout, $( ($case, $string_value) ), +);
        create_test_impl_write_rejected!($fn, endian: big, encoding: utf_16, $layout, $( ($case, $string_value) ), +);
    };

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
                let ctx = (ctx_endian!(endian: $endian), ctx_encoding!(encoding: $encoding), $layout);

                match raw_data.to_writer(&mut deku_writer, ctx) {
                    Ok(_) => assert!(false, "Error was expected, data has been written: {string:#?}"),
                    Err(value) => rejected_check!(value, deku::DekuError::Assertion(_)),
                }
            }
        }
    };
}

#[macro_export]
macro_rules! create_test_impl_write_io_rejected {
    ($fn:ident, $layout:expr, $(,)?) => {
        create_test_impl_write_io_rejected!($fn, endian: little, encoding: utf_8, $layout,);
        create_test_impl_write_io_rejected!($fn, endian: big, encoding: utf_8, $layout,);
        create_test_impl_write_io_rejected!($fn, endian: little, encoding: utf_16, $layout,);
        create_test_impl_write_io_rejected!($fn, endian: big, encoding: utf_16, $layout,);
    };

    ($fn:ident,  endian: $endian: ident, encoding: $encoding: ident, $layout:expr, $(,)?) => {
        paste! {
            #[rstest]
            fn [<write_ $encoding _ $fn _ $endian _io_rejected>] () {
                let raw_data: StringDeku = "valid data".into();

                let mut output: InvalidBufferType = InvalidBufferType{};
                let mut deku_writer = Writer::new(&mut output);
                let ctx = (ctx_endian!(endian: $endian), ctx_encoding!(encoding: $encoding), $layout);

                match raw_data.to_writer(&mut deku_writer, ctx) {
                    Ok(_) => assert!(false, "Error was expected, data has been written"),
                    Err(value) => rejected_check!(value, deku::DekuError::Io(_)),
                }
            }
        }
    };
}

// External file may be used
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
