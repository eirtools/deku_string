//!
//! "Transparency" shim implementations for `StringDeku`.
use crate::StringDeku;

impl core::fmt::Display for StringDeku {
    /// Formats as plain String
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(&*(self.0), f)
    }
}

impl core::fmt::Debug for StringDeku {
    /// Formats as plain String
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&*(self.0), f)
    }
}

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

impl From<String> for StringDeku {
    fn from(value: String) -> StringDeku {
        StringDeku(value)
    }
}

impl From<StringDeku> for String {
    fn from(value: StringDeku) -> Self {
        value.0
    }
}

impl std::hash::Hash for StringDeku {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl PartialOrd<String> for StringDeku {
    fn partial_cmp(&self, other: &String) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
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

impl PartialEq<String> for StringDeku {
    fn eq(&self, other: &String) -> bool {
        &self.0 == other
    }
}

impl PartialEq<StringDeku> for String {
    fn eq(&self, other: &StringDeku) -> bool {
        self == &other.0
    }
}

#[cfg(test)]
mod test {
    use std::borrow::Cow;
    use std::cmp::Ordering;

    use crate::StringDeku;
    use rstest::rstest;

    #[rstest]
    fn display() {
        let str = "from str";
        let str_deku: StringDeku = str.into();
        let formatted = format!("{str_deku}");
        assert_eq!(formatted, str);
    }

    #[rstest]
    fn debug() {
        let str = "from str";
        let str_deku: StringDeku = str.into();
        let formatted = format!("{str_deku:?}");
        let expected = format!("{str:?}");
        assert_eq!(formatted, expected);
    }

    #[rstest]
    #[case::str("from str")]
    #[case::str(String::from("from str"))]
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
    #[case::str(String::from("from str"))]
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

    #[rstest]
    fn test_hash() {
        use std::hash::{DefaultHasher, Hash, Hasher};

        fn calculate_hash<T: Hash + ?Sized>(t: &T) -> u64 {
            let mut s = DefaultHasher::new();
            t.hash(&mut s);
            s.finish()
        }

        let str = "from str";
        let str_deku: StringDeku = str.into();
        assert_eq!(calculate_hash(str), calculate_hash(&str_deku));
    }

    #[rstest]
    fn into_string() {
        let str = "from str";
        let str_deku: StringDeku = str.into();
        let str_value: String = str_deku.clone().into();

        assert_eq!(str, str_value);
        assert_eq!(str, str_deku);
        assert_eq!(
            str_deku.partial_cmp(&"aaa".to_owned()),
            Some(Ordering::Greater)
        );
        assert_eq!(
            str_deku.partial_cmp(&"zzz".to_owned()),
            Some(Ordering::Less)
        );
        assert_eq!(str_deku.partial_cmp(&str_value), Some(Ordering::Equal));
    }
}
