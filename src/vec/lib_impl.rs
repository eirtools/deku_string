use alloc::vec::Vec;

use crate::InternalValue;

use super::VecDeku;

impl<T> InternalValue for VecDeku<T>
where
    T: Sized + Clone + PartialEq + PartialOrd,
{
    type InternalType = Vec<T>;

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
