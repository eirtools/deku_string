//! Standard library implementations for `VecDeku`.
use core::cmp::Ordering;
use core::fmt::{Debug, Formatter, Result as FmtResult};
use core::hash::{Hash, Hasher};
use core::ops::{Deref, DerefMut};

use alloc::vec::Vec;

use crate::{InternalValue as _, VecDeku};

impl<T> VecDeku<T>
where
    T: Sized + Clone,
{
    /// Construct new `VecDeku` from a slice
    #[inline]
    pub fn new(data: &[T]) -> Self {
        Self(data.to_vec())
    }
}

impl<T> DerefMut for VecDeku<T>
where
    T: Sized + Clone,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

impl<T> Deref for VecDeku<T>
where
    T: Sized + Clone,
{
    type Target = [T];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T> Debug for VecDeku<T>
where
    T: Sized + Clone + Debug,
{
    /// Formats as plain String
    #[inline]
    #[allow(
        clippy::min_ident_chars,
        reason = "Conflict with trait definition, <https://github.com/rust-lang/rust-clippy/issues/15365>"
    )]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Debug::fmt(self.internal_ref(), f)
    }
}

impl<T> From<&[T]> for VecDeku<T>
where
    T: Sized + Clone,
{
    #[inline]
    fn from(input: &[T]) -> Self {
        Self::new(input)
    }
}

impl<T> From<Vec<T>> for VecDeku<T>
where
    T: Sized + Clone,
{
    #[inline]
    fn from(input: Vec<T>) -> Self {
        Self::new(&input)
    }
}

impl<T> From<VecDeku<T>> for Vec<T>
where
    T: Sized + Clone,
{
    #[inline]
    fn from(local: VecDeku<T>) -> Self {
        local.internal_move()
    }
}

impl<T> AsRef<[T]> for VecDeku<T>
where
    T: Sized + Clone,
{
    #[inline]
    fn as_ref(&self) -> &[T] {
        self.internal_ref().as_ref()
    }
}

impl<T> AsMut<Vec<T>> for VecDeku<T>
where
    T: Sized + Clone,
{
    #[inline]
    fn as_mut(&mut self) -> &mut Vec<T> {
        self.internal_mut()
    }
}

impl<T> Hash for VecDeku<T>
where
    T: Sized + Clone + PartialEq + PartialOrd + Hash,
{
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.internal_ref().hash(state);
    }
}

impl<T> PartialEq<&[T]> for VecDeku<T>
where
    T: Sized + Clone + PartialEq,
{
    #[inline]
    fn eq(&self, other: &&[T]) -> bool {
        self.internal_ref() == other
    }
}

impl<T> PartialEq<VecDeku<T>> for &[T]
where
    T: Sized + Clone + PartialEq,
{
    #[inline]
    fn eq(&self, other: &VecDeku<T>) -> bool {
        self == other.internal_ref()
    }
}

impl<T> PartialEq<Vec<T>> for VecDeku<T>
where
    T: Sized + Clone + PartialEq,
{
    #[inline]
    fn eq(&self, other: &Vec<T>) -> bool {
        self.internal_ref() == other
    }
}

impl<T> PartialEq<VecDeku<T>> for Vec<T>
where
    T: Sized + Clone + PartialEq,
{
    #[inline]
    fn eq(&self, other: &VecDeku<T>) -> bool {
        self == other.internal_ref()
    }
}

impl<T> PartialOrd<Vec<T>> for VecDeku<T>
where
    T: Sized + Clone + PartialOrd,
{
    #[inline]
    fn partial_cmp(&self, other: &Vec<T>) -> Option<Ordering> {
        self.internal_ref().partial_cmp(other)
    }
}

impl<T> PartialOrd<VecDeku<T>> for Vec<T>
where
    T: Sized + Clone + PartialOrd,
{
    #[inline]
    fn partial_cmp(&self, other: &VecDeku<T>) -> Option<Ordering> {
        self.partial_cmp(other.internal_ref())
    }
}

#[cfg(test)]
mod test {
    use alloc::format;
    use alloc::vec::Vec;
    use core::cmp::Ordering;

    use crate::VecDeku;
    use rstest::rstest;

    const TEST_INPUT: [u8; 3] = [1, 2, 3];
    const TEST_INPUT_OTHER: [u8; 3] = [3, 2, 1];
    const TEST_INPUT_LESS: [u8; 3] = [0, 0, 0];
    const TEST_INPUT_GREATER: [u8; 3] = [9, 9, 9];

    #[rstest]
    fn debug() {
        let input: Vec<u8> = TEST_INPUT.into();
        let local: VecDeku<u8> = input.clone().into();

        let formatted = format!("{local:?}");
        let formatted_input = format!("{input:?}");

        assert_eq!(formatted, formatted_input);
    }

    #[rstest]
    fn deref() {
        let input: Vec<u8> = TEST_INPUT.into();
        let local: VecDeku<u8> = input.clone().into();
        assert_eq!(input, *local);
    }

    #[rstest]
    #[allow(clippy::indexing_slicing, reason = "Checked indexing")]
    fn deref_mut() {
        let mut local: VecDeku<u8> = VecDeku::new(TEST_INPUT.as_slice());
        let x = &mut *local;
        x[0] = 4;
        assert_eq!([4, 2, 3], *local);
    }

    #[rstest]
    fn from_eq() {
        let input = TEST_INPUT;
        let local: VecDeku<u8> = VecDeku::new(&input);

        assert_eq!(local, local);
        assert_eq!(input.as_slice(), local);
        assert_eq!(local, input.as_slice());
    }

    #[rstest]
    fn from_ne() {
        let input = TEST_INPUT;
        let input_other = TEST_INPUT_OTHER;
        let local: VecDeku<u8> = VecDeku::new(&input);

        let local_other: VecDeku<u8> = VecDeku::new(&input_other);

        assert_ne!(local, local_other);
        assert_ne!(local_other, local);

        assert_ne!(local, input_other.as_slice());
        assert_ne!(input_other.as_slice(), local);
    }

    #[rstest]
    fn from_eq_vec() {
        let input: Vec<u8> = [1, 2, 3].to_vec();
        let local: VecDeku<u8> = input.clone().into();

        assert_eq!(local, local);
        assert_eq!(input, local);
        assert_eq!(local, input);
    }

    #[rstest]
    fn from_ne_vec() {
        let input: Vec<u8> = [1, 2, 3].to_vec();
        let input_other: Vec<u8> = [3, 2, 1].to_vec();

        let local: VecDeku<u8> = input.as_slice().into();

        let local_other: VecDeku<u8> = input_other.clone().into();

        assert_ne!(local, local_other);
        assert_ne!(local_other, local);

        assert_ne!(local, input_other);
        assert_ne!(input_other, local);
    }

    #[rstest]
    fn vec_as_ref_into() {
        let mut local: VecDeku<u8> = VecDeku::new(&TEST_INPUT);

        let mut vec: Vec<u8> = local.clone().into();

        assert_eq!(local.as_ref(), &vec);
        assert_eq!(local.as_mut(), &vec);

        let slice: &[u8] = vec.as_mut();

        assert_eq!(local.as_ref(), slice);
        assert_eq!(local.as_mut(), slice);
    }

    #[rstest]
    fn hash() {
        use core::hash::{Hash, Hasher as _};
        use std::collections::hash_map::DefaultHasher;

        fn calculate_hash<T: Hash + ?Sized>(value: &T) -> u64 {
            let mut hasher = DefaultHasher::new();
            value.hash(&mut hasher);
            hasher.finish()
        }

        let input: Vec<u8> = TEST_INPUT.into();
        let local: VecDeku<u8> = input.clone().into();

        assert_eq!(calculate_hash(&input), calculate_hash(&local));
    }

    #[rstest]
    fn partial_cmp() {
        let input: Vec<u8> = TEST_INPUT.into();
        let input_less: Vec<u8> = TEST_INPUT_LESS.into();
        let input_greater: Vec<u8> = TEST_INPUT_GREATER.into();

        let local: VecDeku<u8> = input.clone().into();
        let local_less: VecDeku<u8> = input_less.clone().into();
        let local_greater: VecDeku<u8> = input_greater.clone().into();

        assert_eq!(local.partial_cmp(&input), Some(Ordering::Equal));
        assert_eq!(input.partial_cmp(&local), Some(Ordering::Equal));

        assert_eq!(local.partial_cmp(&input_less), Some(Ordering::Greater));
        assert_eq!(local.partial_cmp(&local_less), Some(Ordering::Greater));
        assert_eq!(local_greater.partial_cmp(&local), Some(Ordering::Greater));
        assert_eq!(input_greater.partial_cmp(&local), Some(Ordering::Greater));

        assert_eq!(local.partial_cmp(&input_greater), Some(Ordering::Less));
        assert_eq!(local.partial_cmp(&local_greater), Some(Ordering::Less));
        assert_eq!(local_less.partial_cmp(&local), Some(Ordering::Less));
        assert_eq!(local_less.partial_cmp(&local), Some(Ordering::Less));
    }
}
