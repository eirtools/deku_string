/// Encoding value for Deku context
macro_rules! _ctx_encoding {
    (encoding: utf_8) => {
        Encoding::Utf8
    };
    (encoding: utf_16) => {
        Encoding::Utf16
    };
    (encoding: utf_32) => {
        Encoding::Utf32
    };
}

/// Endian value for Deku context
macro_rules! _ctx_endian {
    (endian: little) => {
        Endian::Little
    };
    (endian: big) => {
        Endian::Big
    };
}

/// Match value for all supported errors
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
macro_rules! _rejected_check {
    ($value: ident, error: $error: ident) => {
        assert!(
            matches!($value, _match_error!(error: $error)),
            "value doesn't match! {:#?}",
            $value
        )
    };
}

macro_rules! _deku_ctx {
    (ctx: prime, $endian: ident, $encoding: ident, $layout: ident) => {
        (_ctx_endian!(endian: $endian), _ctx_encoding!(encoding: $encoding), paste! { [<LAYOUT_ $layout: upper>] })
    };

    (ctx: alt, $endian: ident, $encoding: ident, $layout: ident) => {
        (_ctx_endian!(endian: $endian), (_ctx_encoding!(encoding: $encoding), paste! { [<LAYOUT_ $layout: upper>] }))
    };
}

pub(crate) use _ctx_encoding;
pub(crate) use _ctx_endian;
pub(crate) use _deku_ctx;
pub(crate) use _match_error;
pub(crate) use _rejected_check;
