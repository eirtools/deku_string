use crate::{
    InternalValue, StringDeku, serde_shim_implementation, std_shim_implementation,
};
use alloc::string::String;

impl InternalValue for StringDeku {
    type InternalType = String;

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
    module_name: serde_impl,
    local_type: StringDeku,
    test_input: "from str",
    test_input_encoded: "\"from str\"",
    test_input_encoded_invalid: "123",
}

std_shim_implementation! {
    module_name: std_impl,
    local_type: StringDeku,
    internal_type: alloc::string::String,
    test_input: "from str".into(),
    test_input_other: "other value".into(),
    test_input_less: "aaa".into(),
    test_input_greater: "zzz".into(),
}
