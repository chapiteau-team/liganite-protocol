// We make sure this pallet uses `no_std` for compiling to Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod types;
use types::*;

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

    /// Storage for the publish details.
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
            who: PublisherId<T>,
        },
    }

    /// Errors.
    #[pallet::error]
    pub enum Error<T> {
        /// The publish already exists.
        PublisherAlreadyExists,
    }

    /// Dispatchable functions ([`Call`]s).
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::publisher_add())]
        pub fn publisher_add(origin: OriginFor<T>, details: PublisherDetails) -> DispatchResult {
            let who = ensure_signed(origin)?;

            if Publishers::<T>::contains_key(&who) {
                return Err(Error::<T>::PublisherAlreadyExists.into());
            }

            Publishers::<T>::insert(&who, details);
            Self::deposit_event(Event::PublisherAdded { who });
            Ok(())
        }
    }
}
