//! Benchmarking setup for liganite-publish

use super::*;

#[allow(unused)]
use crate::Pallet as Publish;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use scale_info::prelude::vec;

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn publisher_add(a: Linear<0, MAX_URL_SIZE>) {
        let len = a as usize;
        let details = PublisherDetails { url: Url::try_from(vec![b'a'; len]).unwrap() };
        let caller: T::AccountId = whitelisted_caller();

        #[extrinsic_call]
        _(RawOrigin::Signed(caller.clone()), details.clone());

        assert_eq!(Publishers::<T>::get(caller), Some(details));
    }

    impl_benchmark_test_suite!(Publish, mock::new_test_ext(), mock::Test);
}
