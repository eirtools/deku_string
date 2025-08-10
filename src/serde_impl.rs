//! "Transparency" shim implementations for `StringDeku`.

use crate::StringDeku;
use serde::{Serialize, Deserialize,Deserializer, Serializer};

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
    fn deserialize_in_place<D>(deserializer: D, place: &mut Self) -> Result<(), D::Error>
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
        let parsed: StringDeku = serde_json::from_str(str).expect("Parse sucessful");
        let expected = "to str";
        let str_value: String = parsed.clone().into();
        assert_eq!(expected, str_value);
    }
}
