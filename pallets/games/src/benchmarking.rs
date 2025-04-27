//! Benchmarking setup for liganite-games

use super::*;

#[allow(unused)]
use crate::Pallet as Games;
use frame_benchmarking::v2::*;
use frame_support::sp_runtime::traits::{Bounded, CheckedDiv};
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
    let initial_balance = CurrencyOf::<T>::max_value()
        .checked_div(&2u32.into())
        .expect("never fails; qed");
    T::Currency::set_balance(account, CurrencyOf::<T>::from(initial_balance));
}

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn game_add(a: Linear<1, MAX_NAME_SIZE>, b: Linear<0, MAX_TAGS_PER_GAME>) {
        let name = bounded_vec(&vec![b'a'; a as usize]);
        let tags = bounded_vec(&(0..b as TagId).collect::<Vec<_>>());
        let url = bounded_vec(b"https://publisher.com");

        for tag in tags.iter() {
            Tags::<T>::insert(*tag, Tag::default());
        }

        let publisher = whitelisted_caller();
        T::PublisherManager::insert_publisher(
            &publisher,
            &PublisherDetails { name: name.clone(), url },
        );

        let game_id = 1;
        let details = GameDetails {
            name,
            tags,
            distribution: Distribution::Instant {
                price: CurrencyOf::<T>::from(1_000u32),
                cid: bounded_vec(b"bafkrgqe3nmefcemf5kx23jw4mpkux4kswm4umtvwmc2j6cf3ftvkv7flwpfrwe7nvlhyzvvn7wtfhec5nkkwxv3hvzjivkw6yv3f3og5edhcldy"),
            },
        };

        #[extrinsic_call]
        _(RawOrigin::Signed(publisher.clone()), game_id, details.clone());

        assert_eq!(PublishedGames::<T>::get(publisher, game_id), Some(details));
    }

    #[benchmark]
    fn buy_free() {
        let publisher = get_account::<T>(0);
        let game_id = 10;
        let game_details = GameDetails {
            name: bounded_vec(&vec![b'a'; MAX_NAME_SIZE as usize]),
            tags: bounded_vec(&vec![TagId::default(); MAX_TAGS_PER_GAME as usize]),
            distribution: Distribution::Free {
                cid: bounded_vec(b"bafkrgqe3nmefcemf5kx23jw4mpkux4kswm4umtvwmc2j6cf3ftvkv7flwpfrwe7nvlhyzvvn7wtfhec5nkkwxv3hvzjivkw6yv3f3og5edhcldy")
            },
        };
        PublishedGames::<T>::insert(&publisher, game_id, game_details);
        let buyer = whitelisted_caller();
        prefund_account::<T>(&buyer);

        #[extrinsic_call]
        game_buy(RawOrigin::Signed(buyer.clone()), publisher.clone(), game_id);

        assert_eq!(OwnedGames::<T>::get(&buyer, (&publisher, game_id)), Some(()));
    }

    #[benchmark]
    fn buy_instant() {
        let publisher = get_account::<T>(0);
        let game_id = 10;
        let price = CurrencyOf::<T>::from(2_000_000_000u32);
        let game_details = GameDetails {
            name: bounded_vec(&vec![b'a'; MAX_NAME_SIZE as usize]),
            tags: bounded_vec(&vec![TagId::default(); MAX_TAGS_PER_GAME as usize]),
            distribution: Distribution::Instant {
                price,
                cid: bounded_vec(b"bafkrgqe3nmefcemf5kx23jw4mpkux4kswm4umtvwmc2j6cf3ftvkv7flwpfrwe7nvlhyzvvn7wtfhec5nkkwxv3hvzjivkw6yv3f3og5edhcldy")
            },
        };
        PublishedGames::<T>::insert(&publisher, game_id, game_details);
        let buyer = whitelisted_caller();
        prefund_account::<T>(&buyer);

        #[extrinsic_call]
        game_buy(RawOrigin::Signed(buyer.clone()), publisher.clone(), game_id);

        assert_eq!(OwnedGames::<T>::get(&buyer, (&publisher, game_id)), Some(()));
    }

    #[benchmark]
    fn order_place() {
        let publisher = get_account::<T>(0);
        let game_id = 10;
        let price = CurrencyOf::<T>::from(2_000_000_000u32);
        let game_details = GameDetails {
            name: bounded_vec(&vec![b'a'; MAX_NAME_SIZE as usize]),
            tags: bounded_vec(&vec![TagId::default(); MAX_TAGS_PER_GAME as usize]),
            distribution: Distribution::Individual { price },
        };
        PublishedGames::<T>::insert(&publisher, game_id, game_details);
        let buyer = whitelisted_caller();
        prefund_account::<T>(&buyer);

        #[extrinsic_call]
        game_buy(RawOrigin::Signed(buyer.clone()), publisher.clone(), game_id);

        let expected = OrderDetails { deposit: price };
        assert_eq!(BuyerOrders::<T>::get(&buyer, (&publisher, game_id)), Some(expected));
        assert_eq!(PublisherOrders::<T>::get(&publisher, game_id), Some(buyer));
    }

    #[benchmark]
    fn order_cancel() {
        let publisher = get_account::<T>(0);
        let game_id = 10;
        let price = CurrencyOf::<T>::from(2_000_000_000u32);
        let game_details = GameDetails {
            name: bounded_vec(&vec![b'a'; MAX_NAME_SIZE as usize]),
            tags: bounded_vec(&vec![TagId::default(); MAX_TAGS_PER_GAME as usize]),
            distribution: Distribution::Individual { price },
        };
        PublishedGames::<T>::insert(&publisher, game_id, game_details);

        let buyer = whitelisted_caller();
        prefund_account::<T>(&buyer);
        BuyerOrders::<T>::insert(&buyer, (&publisher, game_id), &OrderDetails { deposit: price });
        PublisherOrders::<T>::insert(&publisher, game_id, &buyer);

        #[extrinsic_call]
        _(RawOrigin::Signed(buyer.clone()), publisher.clone(), game_id);

        assert_eq!(BuyerOrders::<T>::get(&buyer, (&publisher, game_id)), None);
        assert_eq!(PublisherOrders::<T>::get(&publisher, game_id), None);
    }

    #[benchmark]
    fn order_fulfill() {
        let publisher = whitelisted_caller();
        prefund_account::<T>(&publisher);
        let game_id = 10;
        let buyer = get_account::<T>(0);
        let price = CurrencyOf::<T>::from(2_000_000_000u32);
        let game_details = GameDetails {
            name: bounded_vec(&vec![b'a'; MAX_NAME_SIZE as usize]),
            tags: bounded_vec(&vec![TagId::default(); MAX_TAGS_PER_GAME as usize]),
            distribution: Distribution::Individual { price },
        };
        PublishedGames::<T>::insert(&publisher, game_id, game_details);
        BuyerOrders::<T>::insert(&buyer, (&publisher, game_id), &OrderDetails { deposit: price });
        PublisherOrders::<T>::insert(&publisher, game_id, &buyer);

        #[extrinsic_call]
        _(RawOrigin::Signed(publisher.clone()), game_id, buyer.clone());

        assert_eq!(BuyerOrders::<T>::get(&buyer, (&publisher, game_id)), None);
        assert_eq!(PublisherOrders::<T>::get(&publisher, game_id), None);
        assert_eq!(OwnedGames::<T>::get(&buyer, (&publisher, game_id)), Some(()));
    }

    impl_benchmark_test_suite!(Games, mock::new_test_ext(), mock::Test);
}
