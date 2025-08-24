/// Endian value for Deku context
macro_rules! _ctx_endian {
    (endian: little) => {
        Endian::Little
    };
    (endian: big) => {
        Endian::Big
    };
}

macro_rules! _data_type_static {
    (data: u8) => {
        u8
    };
    (data: str) => {
        &str
    };
}

macro_rules! _data_type {
    (data: u8) => {
        u8
    };
    (data: str) => {
        StringDeku
    };
}

macro_rules! _data_convert {
    (data: u8, $var:ident) => {
        $var.to_vec()
    };
    (data: str, $var:ident) => {
        $var.into_iter()
            .map(|&v| StringDeku::new(v))
            .collect::<Vec<StringDeku>>()
    };
}

macro_rules! _deku_ctx {
    (data: u8, ctx: prime, $layout:ident, $endian: ident) => {
        (_ctx_endian!(endian: $endian),  paste! {[<LAYOUT_ $layout:upper>]})
    };

    ( data: str, ctx: prime, $layout:ident, $endian: ident)=> {
        (_ctx_endian!(endian: $endian),  paste! {[<LAYOUT_ $layout:upper>]}, STRING_CTX)
    };

    (data: u8, ctx: alt, $layout:ident, $endian: ident) => {
        (_ctx_endian!(endian: $endian),  paste! {[<LAYOUT_ $layout:upper>]})
    };

    ( data: str, ctx: alt, $layout:ident, $endian: ident)=> {
        (_ctx_endian!(endian: $endian),  (paste! {[<LAYOUT_ $layout:upper>]}, STRING_CTX))
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

pub(crate) use {
    _ctx_endian, _data_convert, _data_type, _data_type_static, _deku_ctx, _match_error,
    _rejected_check,
};
