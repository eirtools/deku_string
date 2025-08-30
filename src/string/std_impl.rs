//! Additional standard library implementations for [`crate::StringDeku`].

use core::ops::DerefMut;

use crate::{InternalValue as _, StringDeku};
use alloc::borrow::Cow;

impl DerefMut for StringDeku {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.internal_mut()
    }
}

impl From<&str> for StringDeku {
    #[inline]
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

impl From<Cow<'_, str>> for StringDeku {
    #[inline]
    fn from(value: Cow<'_, str>) -> Self {
        Self(value.into())
    }
}

impl PartialEq<StringDeku> for &str {
    #[inline]
    fn eq(&self, other: &StringDeku) -> bool {
        self == &other.0
    }
}

impl<'a> PartialEq<&'a str> for StringDeku {
    #[inline]
    fn eq(&self, other: &&'a str) -> bool {
        &self.0 == other
    }
}

impl<'a> PartialEq<Cow<'a, str>> for StringDeku {
    #[inline]
    fn eq(&self, other: &Cow<'a, str>) -> bool {
        &self.0 == other
    }
}

impl PartialEq<StringDeku> for Cow<'_, str> {
    #[inline]
    fn eq(&self, other: &StringDeku) -> bool {
        self == &other.0
    }
}

#[cfg(test)]
mod test {
    use core::fmt::Debug;

    use alloc::borrow::Cow;

    use crate::StringDeku;
    use rstest::rstest;

    #[rstest]
    fn deref_mut() {
        let mut local: StringDeku = StringDeku::new("mut str");
        let x = &mut *local;
        x.make_ascii_uppercase();
        assert_eq!("MUT STR", local);
    }

    #[rstest]
    #[case::str("from str")]
    #[case::str(Cow::from("from str"))]
    fn test_from_eq<T>(#[case] value: T)
    where
        T: Into<StringDeku> + PartialEq<StringDeku> + Debug + Clone,
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
        T: Into<StringDeku> + PartialEq<StringDeku> + Debug + Clone,
        StringDeku: PartialEq<T>,
    {
        let str_deku: StringDeku = "other value".into();

        assert_ne!(value, str_deku);
        assert_ne!(str_deku, value);
    }
}
