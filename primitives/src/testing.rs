use core::fmt::Debug;
use frame_support::pallet_prelude::*;

/// Creates a `BoundedVec` from a slice, panicking if the slice is too long.
/// Therefore, this should only be used for testing.
pub fn bounded_vec<T, const N: u32>(slice: &[T]) -> BoundedVec<T, ConstU32<N>>
where
    T: Clone + Debug,
{
    BoundedVec::try_from(slice.to_vec()).expect("Slice exceeds the maximum length")
}
