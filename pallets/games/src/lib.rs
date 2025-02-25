// We make sure this pallet uses `no_std` for compiling to Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
use liganite_primitives::{
    publisher::PublisherManager,
    tags::TAGS,
    types::{GameDetails, GameId, PublisherId, Tag, TagId},
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

    #[pallet::genesis_config]
    #[derive(frame_support::DefaultNoBound)]
    pub struct GenesisConfig<T: Config> {
        #[serde(skip)]
        _marker: PhantomData<T>,
    }

    /// Build genesis storage. The tag storage is populated here.
    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            TAGS.iter().enumerate().for_each(|(i, tag)| {
                let tag = Tag::try_from(tag.as_bytes().to_vec())
                    .expect("Failed to create tag at genesis build");
                Tags::<T>::insert(i as TagId, tag);
            })
        }
    }

    /// The pallet's configuration trait.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// A type representing the weights required by the dispatchables of this pallet.
        type WeightInfo: WeightInfo;
        /// The overarching runtime event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// Used to operate on publishers.
        type PublisherManager: PublisherManager<PublisherId = PublisherId<Self>>;
    }

    /// Storage for the game details. Is a map of PublisherId -> GameId -> GameDetails.
    #[pallet::storage]
    pub type PublishedGames<T> = StorageDoubleMap<
        _,
        Twox64Concat,
        PublisherId<T>,
        Blake2_128Concat,
        GameId,
        GameDetails,
        OptionQuery,
    >;

    /// Storage for the game tags. Is a map of TagId -> Tag.
    #[pallet::storage]
    pub type Tags<T> = CountedStorageMap<_, Blake2_128Concat, TagId, Tag, OptionQuery>;

    /// Events.
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A game has been added.
        GameAdded {
            /// The publisher of the game.
            publisher: PublisherId<T>,
            /// The game id.
            game_id: GameId,
        },
    }

    /// Errors.
    #[pallet::error]
    pub enum Error<T> {
        /// The publisher is invalid.
        InvalidPublisher,
        /// The game already exists.
        GameAlreadyExists,
        /// The game details are invalid.
        GameDetailsInvalid,
    }

    /// Dispatchable functions ([`Call`]s).
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::game_add())]
        pub fn game_add(
            origin: OriginFor<T>,
            game_id: GameId,
            details: GameDetails,
        ) -> DispatchResult {
            let publisher = ensure_signed(origin)?;
            ensure!(
                T::PublisherManager::is_valid_publisher(&publisher),
                Error::<T>::InvalidPublisher
            );
            ensure!(
                !PublishedGames::<T>::contains_key(&publisher, game_id),
                Error::<T>::GameAlreadyExists
            );
            ensure!(details.is_valid(), Error::<T>::GameDetailsInvalid);

            PublishedGames::<T>::insert(&publisher, game_id, details);
            Self::deposit_event(Event::GameAdded { publisher, game_id });
            Ok(())
        }
    }
}
