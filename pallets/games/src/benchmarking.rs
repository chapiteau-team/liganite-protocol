//! Benchmarking setup for liganite-games

use super::*;

#[allow(unused)]
use crate::Pallet as Games;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use liganite_primitives::{testing::bounded_vec, MAX_NAME_SIZE};
use scale_info::prelude::vec;

#[benchmarks]
mod benchmarks {
    use super::*;
    use liganite_primitives::types::PublisherDetails;

    #[benchmark]
    fn game_add(a: Linear<1, MAX_NAME_SIZE>) {
        let name = bounded_vec(&vec![b'a'; a as usize]);

        let caller: T::AccountId = whitelisted_caller();
        T::PublisherManager::insert_publisher(
            &caller,
            &PublisherDetails { name: name.clone(), ..Default::default() },
        );

        let game_id = 1;
        let details = GameDetails { name };

        #[extrinsic_call]
        _(RawOrigin::Signed(caller.clone()), game_id, details.clone());

        assert_eq!(PublishedGames::<T>::get(caller, game_id), Some(details));
    }

    impl_benchmark_test_suite!(Games, mock::new_test_ext(), mock::Test);
}
