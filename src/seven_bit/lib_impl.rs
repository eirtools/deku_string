//! Implementations for 7-bit integers.
use super::deku_impl::int7bit_deku_shim_implementation;

use crate::{InternalValue, serde_shim_implementation, std_shim_implementation};

use super::{SevenBitU8, SevenBitU16, SevenBitU32, SevenBitU64, SevenBitU128};

/// macro to implement `type::new()` method
macro_rules! new_impl {
    (
        module_name: $module_name: ident,
        local_type: $local_type: ident,
        internal_type: $internal_type: ident,
        test_input: $test_input: expr,
    ) => {
        mod $module_name {
            use crate::$local_type;

            impl $local_type {
                /// Construct new struct
                #[inline]
                #[must_use]
                pub const fn new(input: $internal_type) -> Self {
                    Self(input)
                }
            }

            #[cfg(test)]
            mod test {
                use crate::$local_type;
                use rstest::rstest;

                #[rstest]
                fn new_impl() {
                    let input: $internal_type = $test_input;
                    let local: $local_type = $local_type::new(input);

                    assert_eq!(input, local);
                }
            }
        }
    };
}
/// All implementations at once.
// macro could take only bit length.
//
// make a PR if there's any way to make it possible without additional dependencies
macro_rules! shim_impl {
    (
        local_type: $local_type: ident,
        internal_type: $internal_type: ident,
        module_name_std: $module_name_std: ident,
        module_name_new_impl: $module_name_new_impl: ident,
        module_name_deku: $module_name_deku: ident,
        module_name_serde: $module_name_serde: ident,
        module_name_deref: $module_name_deref: ident,
    ) => {
        impl InternalValue for $local_type {
            type InternalType = $internal_type;

            #[inline]
            fn internal_move(self) -> Self::InternalType {
                self.0
            }

            #[inline]
            fn internal_mut(&mut self) -> &mut Self::InternalType {
                &mut self.0
            }

            #[inline]
            fn internal_ref(&self) -> &Self::InternalType {
                &self.0
            }
        }

        int7bit_deku_shim_implementation! {
            module_name: $module_name_deku,
            local_type: $local_type,
            internal_type: $internal_type,
        }

        std_shim_implementation! {
            module_name: $module_name_std,
            local_type: $local_type,
            internal_type: $internal_type,
            deref_type: $internal_type,
            test_input: 42,
            test_input_other: 64,
            test_input_less: 12,
            test_input_greater: 86,
        }

        serde_shim_implementation! {
            module_name: $module_name_serde,
            local_type: $local_type,
            internal_type: $internal_type,
            test_input: 42,
            test_input_encoded: "42",
            test_input_encoded_invalid: "\"123\"",
        }

        new_impl! {
            module_name: $module_name_new_impl,
            local_type: $local_type,
            internal_type: $internal_type,
            test_input: 42,
        }

        mod $module_name_deref {
            use crate::{InternalValue as _, $local_type};
            use core::ops::DerefMut;

            impl DerefMut for $local_type {
                #[inline]
                fn deref_mut(&mut self) -> &mut Self::Target {
                    self.internal_mut()
                }
            }

            #[cfg(test)]
            mod test {
                use crate::$local_type;
                use rstest::rstest;

                #[rstest]
                fn deref_mut() {
                    let mut local: $local_type = $local_type::new(42);
                    *local = 12;
                    assert_eq!(12, local);
                }
            }
        }
    };
}

shim_impl!(
    local_type: SevenBitU8,
    internal_type: u8,
    module_name_std: std_impl_8,
    module_name_new_impl: new_impl_8,
    module_name_deku: deku_impl_8,
    module_name_serde: serde_impl_8,
    module_name_deref: deref_impl_8,
);

shim_impl!(
    local_type: SevenBitU16,
    internal_type: u16,
    module_name_std: std_impl_16,
    module_name_new_impl: new_impl_16,
    module_name_deku: deku_impl_16,
    module_name_serde: serde_impl_16,
    module_name_deref: deref_impl_16,
);

shim_impl!(
    local_type: SevenBitU32,
    internal_type: u32,
    module_name_std: std_impl_32,
    module_name_new_impl: new_impl_32,
    module_name_deku: deku_impl_32,
    module_name_serde: serde_impl_32,
    module_name_deref: deref_impl_32,
);

shim_impl!(
    local_type: SevenBitU64,
    internal_type: u64,
    module_name_std: std_impl_64,
    module_name_new_impl: new_impl_64,
    module_name_deku: deku_impl_64,
    module_name_serde: serde_impl_64,
    module_name_deref: deref_impl_64,
);

shim_impl!(
    local_type: SevenBitU128,
    internal_type: u128,
    module_name_std: std_impl_128,
    module_name_new_impl: new_impl_128,
    module_name_deku: deku_impl_128,
    module_name_serde: serde_impl_128,
    module_name_deref: deref_impl_128,
);
