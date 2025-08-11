//! Additional "transparency" shim implementations for `StringDeku`.

use crate::StringDeku;

impl From<&str> for StringDeku {
    fn from(value: &str) -> StringDeku {
        StringDeku(value.into())
    }
}

impl From<std::borrow::Cow<'_, str>> for StringDeku {
    fn from(value: std::borrow::Cow<'_, str>) -> StringDeku {
        StringDeku(value.into())
    }
}

impl PartialEq<StringDeku> for &str {
    fn eq(&self, other: &StringDeku) -> bool {
        self == &other.0
    }
}

impl<'a> PartialEq<&'a str> for StringDeku {
    fn eq(&self, other: &&'a str) -> bool {
        &self.0 == other
    }
}

impl<'a> PartialEq<std::borrow::Cow<'a, str>> for StringDeku {
    fn eq(&self, other: &std::borrow::Cow<'a, str>) -> bool {
        &self.0 == other
    }
}

impl PartialEq<StringDeku> for std::borrow::Cow<'_, str> {
    fn eq(&self, other: &StringDeku) -> bool {
        self == &other.0
    }
}

#[cfg(test)]
mod test {
    use std::borrow::Cow;

    use crate::StringDeku;
    use rstest::rstest;

    #[rstest]
    #[case::str("from str")]
    #[case::str(Cow::from("from str"))]
    fn test_from_eq<T>(#[case] value: T)
    where
        T: Into<StringDeku> + PartialEq<StringDeku> + std::fmt::Debug + Clone,
        StringDeku: PartialEq<T>,
    {
        let str_deku: StringDeku = value.clone().into();

        assert_eq!(value, str_deku);
        assert_eq!(str_deku, value);
    }

    #[rstest]
    #[case::str("from str")]
    #[case::str(Cow::from("from str"))]
    fn test_from_ne<T>(#[case] value: T)
    where
        T: Into<StringDeku> + PartialEq<StringDeku> + std::fmt::Debug + Clone,
        StringDeku: PartialEq<T>,
    {
        let str_deku: StringDeku = "other value".into();

        assert_ne!(value, str_deku);
        assert_ne!(str_deku, value);
    }
}
