//! Benchmarking setup for liganite-publish

use super::*;

#[allow(unused)]
use crate::Pallet as Publish;
use frame_benchmarking::v2::*;
use frame_support::sp_runtime::traits::Bounded;
use frame_system::RawOrigin;
use liganite_primitives::{testing::bounded_vec, MAX_NAME_SIZE, MAX_URL_SIZE};
use scale_info::prelude::vec;

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
    fn deposit_set() {
        let original_deposit = PublisherDeposit::<T>::get();
        let new_deposit = CurrencyOf::<T>::from(1_234u32);

        #[extrinsic_call]
        _(RawOrigin::Root, new_deposit);

        assert_ne!(original_deposit, new_deposit);
        assert_eq!(PublisherDeposit::<T>::get(), new_deposit);
    }

    #[benchmark]
    fn publisher_register(a: Linear<1, MAX_NAME_SIZE>, b: Linear<0, MAX_URL_SIZE>) {
        let name = bounded_vec(&vec![b'a'; a as usize]);
        let url = bounded_vec(&vec![b'b'; b as usize]);
        let details = PublisherDetails { name, url };
        let caller: T::AccountId = whitelisted_caller();
        prefund_account::<T>(&caller);

        #[extrinsic_call]
        _(RawOrigin::Signed(caller.clone()), details.clone());

        assert_eq!(Publishers::<T>::get(caller), Some(details));
    }

    impl_benchmark_test_suite!(Publish, mock::new_test_ext(), mock::Test);
}
