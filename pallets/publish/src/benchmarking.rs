//! Benchmarking setup for liganite-publish

use super::*;

#[allow(unused)]
use crate::Pallet as Publish;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use liganite_primitives::{testing::bounded_vec, MAX_NAME_SIZE, MAX_URL_SIZE};
use scale_info::prelude::vec;

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn publisher_add(a: Linear<0, MAX_NAME_SIZE>, b: Linear<0, MAX_URL_SIZE>) {
        let name = bounded_vec(&vec![b'a'; a as usize]);
        let url = bounded_vec(&vec![b'b'; b as usize]);
        let details = PublisherDetails { name, url };
        let caller: T::AccountId = whitelisted_caller();

        #[extrinsic_call]
        _(RawOrigin::Signed(caller.clone()), details.clone());

        assert_eq!(Publishers::<T>::get(caller), Some(details));
    }

    impl_benchmark_test_suite!(Publish, mock::new_test_ext(), mock::Test);
}
