// We make sure this pallet uses `no_std` for compiling to Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
use liganite_primitives::{
    publisher::PublisherManager,
    types::{PublisherDetails, PublisherId},
};

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// The pallet's configuration trait.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// A type representing the weights required by the dispatchables of this pallet.
        type WeightInfo: WeightInfo;

        /// The overarching runtime event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    /// Storage for the publish details. Is a map of PublisherId -> PublisherDetails.
    #[pallet::storage]
    pub type Publishers<T> =
        StorageMap<_, Twox64Concat, PublisherId<T>, PublisherDetails, OptionQuery>;

    /// Events.
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A publish has been added.
        PublisherAdded {
            /// The account which was added.
            publisher: PublisherId<T>,
        },
    }

    /// Errors.
    #[pallet::error]
    pub enum Error<T> {
        /// The publish already exists.
        PublisherAlreadyExists,
        /// The publisher details are invalid.
        PublisherDetailsInvalid,
    }

    /// Dispatchable functions ([`Call`]s).
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Adds a new publisher to the system.
        ///
        /// This function adds a publisher by storing their details in the `Publishers` storage. It
        /// checks that the publisher does not already exist in the system before adding
        /// them. A `PublisherAdded` event is emitted once the publisher is successfully
        /// added.
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::publisher_add(details.name.len() as u32, details.url.len() as u32))]
        pub fn publisher_add(origin: OriginFor<T>, details: PublisherDetails) -> DispatchResult {
            let publisher = ensure_signed(origin)?;
            ensure!(!Publishers::<T>::contains_key(&publisher), Error::<T>::PublisherAlreadyExists);
            ensure!(details.is_valid(), Error::<T>::PublisherDetailsInvalid);

            Publishers::<T>::insert(&publisher, &details);
            Self::deposit_event(Event::PublisherAdded { publisher });
            Ok(())
        }
    }
}

impl<T: Config> PublisherManager for Pallet<T> {
    type PublisherId = PublisherId<T>;

    fn is_valid_publisher(publisher_id: &PublisherId<T>) -> bool {
        Publishers::<T>::contains_key(publisher_id)
    }

    fn insert_publisher(publisher_id: &PublisherId<T>, details: &PublisherDetails) {
        Publishers::<T>::insert(publisher_id, details);
    }
}
