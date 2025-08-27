//! Serde implementations for `VecDeku`
use crate::{InternalValue as _, VecDeku};
use alloc::vec::Vec;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

impl<T> Serialize for VecDeku<T>
where
    T: Sized + Clone + Serialize,
{
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Serialize::serialize(self.internal_ref(), serializer)
    }
}

impl<'de, T> Deserialize<'de> for VecDeku<T>
where
    T: Sized + Clone + PartialEq + PartialOrd + Deserialize<'de>,
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data: Vec<T> = Deserialize::deserialize(deserializer)?;
        Ok(data.into())
    }

    #[inline]
    fn deserialize_in_place<D>(
        deserializer: D,
        place: &mut Self,
    ) -> Result<(), D::Error>
    where
        D: Deserializer<'de>,
    {
        Deserialize::deserialize_in_place(deserializer, place.internal_mut())
    }
}

#[cfg(test)]
mod test {
    use crate::VecDeku;
    use rstest::rstest;
    use serde::de::Deserialize as _;

    const TEST_INPUT: [u8; 3] = [1, 2, 3];
    const TEST_INPUT_ENCODED: &str = "[1,2,3]";
    const TEST_INPUT_ENCODED_INVALID: &str = "\"123\"";

    #[rstest]
    fn serialize() {
        let value: VecDeku<u8> = TEST_INPUT.as_slice().into();
        let formatted = serde_json::to_string(&value).expect("Encode Ok");
        assert_eq!(formatted, TEST_INPUT_ENCODED);
    }

    #[rstest]
    fn deserialize() {
        let parsed: VecDeku<u8> =
            serde_json::from_str(TEST_INPUT_ENCODED).expect("Parse successful");
        assert_eq!(parsed, TEST_INPUT.as_slice());
    }

    #[rstest]
    fn deserialize_fail() {
        serde_json::from_str::<VecDeku<u8>>(TEST_INPUT_ENCODED_INVALID)
            .expect_err("Parse failed");
    }

    #[rstest]
    fn deserialize_in_place() {
        let mut de = serde_json::de::Deserializer::from_str(TEST_INPUT_ENCODED);
        let mut parsed: VecDeku<u8> = VecDeku::default();
        VecDeku::<u8>::deserialize_in_place(&mut de, &mut parsed)
            .expect("Parse successful");
        assert_eq!(parsed, TEST_INPUT.as_slice());
    }
}
