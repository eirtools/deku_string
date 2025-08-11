// Plain trait implementations for `InternalValue` are preferred.
//
// Make a PR or give a hint if you know how.
pub(crate) mod serde_impl;
pub(crate) mod std_impl;

pub(crate) trait InternalValue {
    type InternalType;

    fn internal_move(self) -> Self::InternalType;
    fn internal_ref(&self) -> &Self::InternalType;
    fn internal_mut(&mut self) -> &mut Self::InternalType;
}
