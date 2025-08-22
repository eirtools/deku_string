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

pub(crate) use {_match_error, _rejected_check};
