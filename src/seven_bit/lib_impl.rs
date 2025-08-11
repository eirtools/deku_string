use super::deku_impl::int7bit_deku_shim_implementation;

use crate::{InternalValue, serde_shim_implementation, std_shim_implementation};

use super::{SevenBitU32, SevenBitU64, SevenBitU128};

impl InternalValue for SevenBitU32 {
    type InternalType = u32;

    fn internal_ref(&self) -> &Self::InternalType {
        &self.0
    }

    fn internal_mut(&mut self) -> &mut Self::InternalType {
        &mut self.0
    }

    fn internal_move(self) -> Self::InternalType {
        self.0
    }
}

impl InternalValue for SevenBitU64 {
    type InternalType = u64;

    fn internal_ref(&self) -> &Self::InternalType {
        &self.0
    }

    fn internal_mut(&mut self) -> &mut Self::InternalType {
        &mut self.0
    }

    fn internal_move(self) -> Self::InternalType {
        self.0
    }
}

impl InternalValue for SevenBitU128 {
    type InternalType = u128;

    fn internal_ref(&self) -> &Self::InternalType {
        &self.0
    }

    fn internal_mut(&mut self) -> &mut Self::InternalType {
        &mut self.0
    }

    fn internal_move(self) -> Self::InternalType {
        self.0
    }
}

serde_shim_implementation! {
    module_name: serde_impl_32,
    local_type: SevenBitU32,
    test_input: 42,
    test_input_encoded: "42",
    test_input_encoded_invalid: "\"123\"",
}

std_shim_implementation! {
    module_name: std_impl_32,
    local_type: SevenBitU32,
    internal_type: u32,
    test_input: 42,
    test_input_other: 64,
    test_input_less: 12,
    test_input_greater: 86,
}

int7bit_deku_shim_implementation! {
    module_name: deku_impl_32,
    local_type: SevenBitU32,
    internal_type: u32,
}

serde_shim_implementation! {
    module_name: serde_impl_64,
    local_type: SevenBitU64,
    test_input: 42,
    test_input_encoded: "42",
    test_input_encoded_invalid: "\"123\"",
}

std_shim_implementation! {
    module_name: std_impl_64,
    local_type: SevenBitU64,
    internal_type: u64,
    test_input: 42,
    test_input_other: 64,
    test_input_less: 12,
    test_input_greater: 86,
}

int7bit_deku_shim_implementation! {
    module_name: deku_impl_64,
    local_type: SevenBitU64,
    internal_type: u64,
}

serde_shim_implementation! {
    module_name: serde_impl_128,
    local_type: SevenBitU128,
    test_input: 42,
    test_input_encoded: "42",
    test_input_encoded_invalid: "\"123\"",
}

std_shim_implementation! {
    module_name: std_impl_128,
    local_type: SevenBitU128,
    internal_type: u128,
    test_input: 42,
    test_input_other: 64,
    test_input_less: 12,
    test_input_greater: 86,
}

int7bit_deku_shim_implementation! {
    module_name: deku_impl_128,
    local_type: SevenBitU128,
    internal_type: u128,
}
