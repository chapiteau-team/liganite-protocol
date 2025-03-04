//! Benchmarking setup for liganite-games

use super::*;

#[allow(unused)]
use crate::Pallet as Games;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use liganite_primitives::{
    testing::bounded_vec, types::PublisherDetails, MAX_NAME_SIZE, MAX_TAGS_PER_GAME,
};
use scale_info::prelude::{vec, vec::Vec};

const SEED: u32 = 0;

fn get_account<T: Config>(index: u32) -> T::AccountId {
    account("account", index, SEED)
}

fn prefund_account<T: Config>(account: &T::AccountId) {
    T::Currency::set_balance(account, CurrencyOf::<T>::from(100_000_000u32));
}

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn game_add(a: Linear<1, MAX_NAME_SIZE>, b: Linear<0, MAX_TAGS_PER_GAME>) {
        let name = bounded_vec(&vec![b'a'; a as usize]);
        let tags = bounded_vec(&(0..b as TagId).collect::<Vec<_>>());

        for tag in tags.iter() {
            Tags::<T>::insert(*tag, Tag::default());
        }

        let publisher = whitelisted_caller();
        T::PublisherManager::insert_publisher(
            &publisher,
            &PublisherDetails { name: name.clone(), ..Default::default() },
        );

        let game_id = 1;
        let details = GameDetails { name, tags, price: CurrencyOf::<T>::from(1_000u32) };

        #[extrinsic_call]
        _(RawOrigin::Signed(publisher.clone()), game_id, details.clone());

        assert_eq!(PublishedGames::<T>::get(publisher, game_id), Some(details));
    }

    #[benchmark]
    fn game_order() {
        let publisher = get_account::<T>(0);
        let game_id = 1;
        let price = CurrencyOf::<T>::from(1_234u32);
        let game_details = GameDetails {
            name: bounded_vec(&vec![b'a'; MAX_NAME_SIZE as usize]),
            tags: bounded_vec(&vec![TagId::default(); MAX_TAGS_PER_GAME as usize]),
            price,
        };
        PublishedGames::<T>::insert(&publisher, game_id, game_details);
        let buyer = whitelisted_caller();
        prefund_account::<T>(&buyer);

        #[extrinsic_call]
        _(RawOrigin::Signed(buyer.clone()), publisher.clone(), game_id);

        let expected = OrderDetails { deposit: price };
        assert_eq!(BuyerOrders::<T>::get(&buyer, (&publisher, game_id)), Some(expected));
        assert_eq!(PublisherOrders::<T>::get(&publisher, game_id), Some(buyer));
    }

    impl_benchmark_test_suite!(Games, mock::new_test_ext(), mock::Test);
}
