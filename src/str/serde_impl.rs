//! "Transparency" shim implementations for `StringDeku`.

use crate::StringDeku;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

impl Serialize for StringDeku {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Serialize::serialize(&*(self.0), serializer)
    }
}

impl<'de> Deserialize<'de> for StringDeku {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(StringDeku(Deserialize::deserialize(deserializer)?))
    }

    #[inline]
    fn deserialize_in_place<D>(
        deserializer: D,
        place: &mut Self,
    ) -> Result<(), D::Error>
    where
        D: Deserializer<'de>,
    {
        Deserialize::deserialize_in_place(deserializer, &mut (place.0))
    }
}

#[cfg(test)]
mod test {
    use crate::StringDeku;
    use rstest::rstest;
    use serde::de::Deserialize;
    use serde_json;

    #[rstest]
    fn serialize() {
        let str = "from str";
        let str_deku: StringDeku = str.into();
        let formatted = serde_json::to_string(&str_deku).expect("Encode Ok");
        let expected = "\"from str\"";
        assert_eq!(&formatted, expected);
    }

    #[rstest]
    fn deserialize() {
        let str = "\"to str\"";
        let parsed: StringDeku = serde_json::from_str(str).expect("Parse successful");
        let expected = "to str";
        assert_eq!(expected, parsed);
    }

    #[rstest]
    fn deserialize_fail() {
        let str = "123";
        serde_json::from_str::<StringDeku>(str).expect_err("Parse failed");
    }

    #[rstest]
    fn deserialize_in_place() {
        let str = "\"to str\"";
        let mut de = serde_json::de::Deserializer::from_str(str);
        let mut parsed: StringDeku = Default::default();
        StringDeku::deserialize_in_place(&mut de, &mut parsed)
            .expect("Parse successful");
        let expected = "to str";
        assert_eq!(expected, parsed);
    }
}
