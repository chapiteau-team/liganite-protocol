#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for liganite_publisher.
pub trait WeightInfo {
	fn deposit_set() -> Weight;
	fn publisher_register(a: u32, b: u32, ) -> Weight;
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn deposit_set() -> Weight {
		Weight::from_parts(9_000_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}

	fn publisher_register(_a: u32, _b: u32, ) -> Weight {
		Weight::from_parts(9_000_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
