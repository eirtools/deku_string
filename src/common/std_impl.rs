//! "Transparency" shim implementations for `InternalValue` types.

/// Macro to generate std transparent implementation for internal types and tests for them.
///
/// `module_name`: Module name produced
/// `local_type`: Local type to create shims for. It must implement `InternalValue` trait.
/// `internal_type`: Internal type for the local type
/// `test_input`: common test input for various tests
/// `test_input_other`: test input not equal to `test_input`
/// `test_input_less`: test input less than `test_input`
/// `test_input_greater`: test input greater that `test_input`
macro_rules! std_shim_implementation {
    (
        module_name: $module_name: ident,
        local_type: $local_type: ident,
        internal_type: $internal_type: ty,
        deref_type: $deref_type: ty,
        test_input: $test_input: expr,
        test_input_other: $test_input_other: expr,
        test_input_less: $test_input_less: expr,
        test_input_greater: $test_input_greater: expr,
    ) => {
        mod $module_name {
            use crate::{InternalValue as _, $local_type};
            use core::cmp::Ordering;
            use core::fmt::{Debug, Display, Formatter, Result as FmtResult};
            use core::hash::{Hash, Hasher};
            use core::ops::Deref;

            impl Deref for $local_type {
                type Target = $deref_type;

                fn deref(&self) -> &Self::Target {
                    self.internal_ref()
                }
            }

            impl Display for $local_type {
                #[inline]
                fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
                    Display::fmt(self.internal_ref(), fmt)
                }
            }

            impl Debug for $local_type {
                /// Formats as plain String
                #[inline]
                fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> FmtResult {
                    Debug::fmt(self.internal_ref(), fmt)
                }
            }

            impl From<$internal_type> for $local_type {
                #[inline]
                fn from(input: $internal_type) -> $local_type {
                    $local_type::new(input)
                }
            }

            impl From<$local_type> for $internal_type {
                #[inline]
                fn from(local: $local_type) -> Self {
                    local.internal_move()
                }
            }

            impl Hash for $local_type {
                fn hash<H: Hasher>(&self, state: &mut H) {
                    self.internal_ref().hash(state);
                }
            }

            impl PartialOrd<$internal_type> for $local_type {
                fn partial_cmp(&self, input: &$internal_type) -> Option<Ordering> {
                    self.internal_ref().partial_cmp(input)
                }
            }

            impl PartialOrd<$local_type> for $internal_type {
                fn partial_cmp(&self, other: &$local_type) -> Option<Ordering> {
                    self.partial_cmp(other.internal_ref())
                }
            }

            impl PartialEq<$internal_type> for $local_type {
                fn eq(&self, input: &$internal_type) -> bool {
                    self.internal_ref() == input
                }
            }

            impl PartialEq<$local_type> for $internal_type {
                fn eq(&self, other: &$local_type) -> bool {
                    self == other.internal_ref()
                }
            }

            #[cfg(test)]
            mod test {
                use alloc::format;
                use core::cmp::Ordering;

                use crate::$local_type;
                use rstest::rstest;

                #[rstest]
                fn display() {
                    let input: $internal_type = $test_input;
                    let local: $local_type = input.clone().into();

                    let formatted = format!("{local}");
                    let formatted_input = format!("{input}");

                    assert_eq!(formatted, formatted_input);
                }

                #[rstest]
                fn debug() {
                    let input: $internal_type = $test_input;
                    let local: $local_type = input.clone().into();

                    let formatted = format!("{local:?}");
                    let formatted_input = format!("{input:?}");

                    assert_eq!(formatted, formatted_input);
                }

                #[rstest]
                fn deref() {
                    let input: $internal_type = $test_input;
                    let local: $local_type = input.clone().into();
                    assert_eq!(input, *local);
                }

                #[rstest]
                fn eq() {
                    let input: $internal_type = $test_input;
                    let local: $local_type = input.clone().into();

                    let other_into: $internal_type = local.clone().into();

                    assert_eq!(local, local);
                    assert_eq!(input, local);
                    assert_eq!(local, input);

                    assert_eq!(other_into, input);
                    assert_eq!(input, other_into);
                }

                #[rstest]
                fn ne() {
                    let input: $internal_type = $test_input;
                    let local: $local_type = input.into();

                    let input_other: $internal_type = $test_input_other;
                    let local_other: $local_type = input_other.clone().into();

                    assert_ne!(local, local_other);
                    assert_ne!(local_other, local);

                    assert_ne!(local, input_other);
                    assert_ne!(input_other, local);
                }

                #[rstest]
                fn hash() {
                    #[allow(unused_imports)]
                    use alloc::collections::TryReserveError as _; // ensure alloc is linked in tests
                    use core::hash::{Hash, Hasher as _};
                    use std::collections::hash_map::DefaultHasher;

                    fn calculate_hash<T: Hash + ?Sized>(value: &T) -> u64 {
                        let mut hasher = DefaultHasher::new();
                        value.hash(&mut hasher);
                        hasher.finish()
                    }

                    let input: $internal_type = $test_input;
                    let local: $local_type = input.clone().into();

                    assert_eq!(calculate_hash(&input), calculate_hash(&local));
                }

                #[rstest]
                fn partial_cmp() {
                    let input: $internal_type = $test_input;
                    let input_less: $internal_type = $test_input_less;
                    let input_greater: $internal_type = $test_input_greater;

                    let local: $local_type = input.clone().into();
                    let local_less: $local_type = input_less.clone().into();
                    let local_greater: $local_type = input_greater.clone().into();

                    assert_eq!(local.partial_cmp(&input), Some(Ordering::Equal));
                    assert_eq!(input.partial_cmp(&local), Some(Ordering::Equal));

                    assert_eq!(local.partial_cmp(&input_less), Some(Ordering::Greater));
                    assert_eq!(local.partial_cmp(&local_less), Some(Ordering::Greater));
                    assert_eq!(
                        local_greater.partial_cmp(&local),
                        Some(Ordering::Greater)
                    );
                    assert_eq!(
                        input_greater.partial_cmp(&local),
                        Some(Ordering::Greater)
                    );

                    assert_eq!(local.partial_cmp(&input_greater), Some(Ordering::Less));
                    assert_eq!(local.partial_cmp(&local_greater), Some(Ordering::Less));
                    assert_eq!(local_less.partial_cmp(&local), Some(Ordering::Less));
                    assert_eq!(local_less.partial_cmp(&local), Some(Ordering::Less));
                }
            }
        }
    };
}

pub(crate) use std_shim_implementation;
