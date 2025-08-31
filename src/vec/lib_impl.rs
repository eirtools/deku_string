//! Crate-related implementations for [`crate::VecDeku`].
use alloc::vec::Vec;

use crate::InternalValue;

use super::VecDeku;

impl<T> InternalValue for VecDeku<T>
where
    T: Sized + Clone,
{
    type InternalType = Vec<T>;

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
