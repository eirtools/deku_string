//! Transparent serde shim implementations for `InternalValue` types.

/// Macro to generate serde implementation for types and tests for them.
///
/// module is automatically guarded so no need to add additional gate.
///
/// * `module_name`: Module name produced.
/// * `local_type`: Local type to create shims for.
/// * `test_input`: Plain value before serialization with `serde_json`.
/// * `test_input_encoded`: Plain value after serialization with `serde_json`.
/// * `test_input_encoded_invalid`: Plain value for deserialization error test.
macro_rules! serde_shim_implementation {
    (
        module_name: $module_name: ident,
        local_type: $local_type: ident,
        internal_type: $internal_type: ty,
        test_input: $test_input: literal,
        test_input_encoded: $test_input_encoded: literal,
        test_input_encoded_invalid: $test_input_encoded_invalid: literal,
    ) => {
        #[cfg(feature = "serde")]
        mod $module_name {
            use crate::{InternalValue as _, $local_type};
            use serde::{Deserialize, Deserializer, Serialize, Serializer};

            impl Serialize for $local_type {
                #[inline]
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: Serializer,
                {
                    Serialize::serialize(self.internal_ref(), serializer)
                }
            }

            impl<'de> Deserialize<'de> for $local_type {
                #[inline]
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: Deserializer<'de>,
                {
                    let value: $internal_type = Deserialize::deserialize(deserializer)?;
                    Ok($local_type::new(value))
                }

                #[inline]
                fn deserialize_in_place<D>(
                    deserializer: D,
                    place: &mut Self,
                ) -> Result<(), D::Error>
                where
                    D: Deserializer<'de>,
                {
                    Deserialize::deserialize_in_place(
                        deserializer,
                        place.internal_mut(),
                    )
                }
            }

            #[cfg(test)]
            mod test {
                use crate::$local_type;
                use rstest::rstest;
                use serde::de::Deserialize as _;

                #[rstest]
                fn serialize() {
                    let input = $test_input;
                    let value: $local_type = input.into();
                    let formatted = serde_json::to_string(&value).expect("Encode Ok");
                    assert_eq!(formatted, $test_input_encoded);
                }

                #[rstest]
                fn deserialize() {
                    let parsed: $local_type = serde_json::from_str($test_input_encoded)
                        .expect("Parse successful");
                    assert_eq!(parsed, $test_input);
                }

                #[rstest]
                fn deserialize_fail() {
                    serde_json::from_str::<$local_type>($test_input_encoded_invalid)
                        .expect_err("Parse failed");
                }

                #[rstest]
                fn deserialize_in_place() {
                    let mut de =
                        serde_json::de::Deserializer::from_str($test_input_encoded);
                    let mut parsed: $local_type = Default::default();
                    $local_type::deserialize_in_place(&mut de, &mut parsed)
                        .expect("Parse successful");
                    assert_eq!(parsed, $test_input);
                }
            }
        }
    };
}

pub(crate) use serde_shim_implementation;
