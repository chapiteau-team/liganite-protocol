#![allow(unused_parens)]
#![allow(unused_imports)]

use core::marker::PhantomData;
use frame_support::{
    traits::Get,
    weights::{constants::RocksDbWeight, Weight},
};

/// Weight functions needed for liganite_games.
pub trait WeightInfo {
    fn game_add(a: u32, b: u32) -> Weight;
    fn buy_free() -> Weight;
    fn buy_instant() -> Weight;
    fn order_place() -> Weight;
    fn order_cancel() -> Weight;
    fn order_fulfill() -> Weight;

    fn game_buy() -> Weight {
        Self::buy_free().max(Self::buy_instant()).max(Self::order_place())
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    fn game_add(_a: u32, _b: u32) -> Weight {
        Weight::from_parts(9_000_000, 0).saturating_add(RocksDbWeight::get().writes(1_u64))
    }

    fn buy_free() -> Weight {
        Weight::from_parts(9_000_000, 0).saturating_add(RocksDbWeight::get().writes(1_u64))
    }

    fn buy_instant() -> Weight {
        Weight::from_parts(9_000_000, 0).saturating_add(RocksDbWeight::get().writes(1_u64))
    }

    fn order_place() -> Weight {
        Weight::from_parts(9_000_000, 0).saturating_add(RocksDbWeight::get().writes(1_u64))
    }

    fn order_cancel() -> Weight {
        Weight::from_parts(9_000_000, 0).saturating_add(RocksDbWeight::get().writes(1_u64))
    }

    fn order_fulfill() -> Weight {
        Weight::from_parts(9_000_000, 0).saturating_add(RocksDbWeight::get().writes(1_u64))
    }
}
