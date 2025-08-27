//! Common implementations for internal structures.

// Plain trait implementations for `InternalValue` are preferred.
//
// Make a PR or give a hint if you know how.
pub(crate) mod serde_impl;
pub(crate) mod std_impl;

/// Operations like move, ref and ref mut for inner values.
pub(crate) trait InternalValue {
    /// Internal type for this wrapper
    type InternalType;

    /// Move internal value out of wrapper
    fn internal_move(self) -> Self::InternalType;

    /// Reference internal value as mutable
    fn internal_mut(&mut self) -> &mut Self::InternalType;

    /// Reference internal value
    fn internal_ref(&self) -> &Self::InternalType;
}
