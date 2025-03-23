// We make sure this pallet uses `no_std` for compiling to Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    pallet_prelude::*,
    traits::fungible::{hold::Mutate as FunHoldMutate, Inspect as FunInspect, Mutate as FunMutate},
};
use frame_system::pallet_prelude::*;
use liganite_primitives::{
    publisher::PublisherManager,
    tags::TAGS,
    types::{AccountIdOf, BuyerId, GameDetails, GameId, OrderDetails, PublisherId, Tag, TagId},
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

type CurrencyOf<T> = <<T as Config>::Currency as FunInspect<AccountIdOf<T>>>::Balance;
type GameDetailsOf<T> = GameDetails<CurrencyOf<T>>;
type OrderDetailsOf<T> = OrderDetails<CurrencyOf<T>>;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::traits::tokens::{
        Fortitude::Polite, Precision::BestEffort, Restriction::Free,
    };
    use liganite_primitives::types::GlobalGameId;

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

    /// A reason for the pallet placing a hold on funds.
    #[pallet::composite_enum]
    pub enum HoldReason {
        /// The game payment.
        GamePayment,
    }

    /// The pallet's configuration trait.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// A type representing the weights required by the dispatchables of this pallet.
        type WeightInfo: WeightInfo;

        /// The overarching runtime event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Overarching hold reason.
        type RuntimeHoldReason: From<HoldReason>;

        /// Used to operate on currencies.
        type Currency: FunMutate<Self::AccountId>
            + FunHoldMutate<Self::AccountId, Reason = Self::RuntimeHoldReason>;

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
        GameDetailsOf<T>,
        OptionQuery,
    >;

    /// Storage for the game tags. Is a map of TagId -> Tag.
    #[pallet::storage]
    pub type Tags<T> = CountedStorageMap<_, Blake2_128Concat, TagId, Tag, OptionQuery>;

    /// Storage for the game orders. Is a map of PublisherId -> GameId -> BuyerId.
    #[pallet::storage]
    pub type PublisherOrders<T> = StorageDoubleMap<
        _,
        Twox64Concat,
        PublisherId<T>,
        Blake2_128Concat,
        GameId,
        BuyerId<T>,
        OptionQuery,
    >;

    /// Storage for the game orders. Is a map of BuyerId -> GlobalGameId -> OrderDetails.
    #[pallet::storage]
    pub type BuyerOrders<T> = StorageDoubleMap<
        _,
        Twox64Concat,
        BuyerId<T>,
        Blake2_128Concat,
        GlobalGameId<T>,
        OrderDetailsOf<T>,
        OptionQuery,
    >;

    /// Storage for the game ownership. Is a map of BuyerId -> GlobalGameId -> ().
    #[pallet::storage]
    pub type OwnedGames<T> = StorageDoubleMap<
        _,
        Twox64Concat,
        BuyerId<T>,
        Blake2_128Concat,
        GlobalGameId<T>,
        (),
        OptionQuery,
    >;

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
        /// An order has been placed.
        OrderPlaced {
            /// The buyer of the game.
            buyer: BuyerId<T>,
            /// The publisher of the game.
            publisher: PublisherId<T>,
            /// The game id.
            game_id: GameId,
        },
        /// An order has been fulfilled.
        OrderFulfilled {
            /// The buyer of the game.
            buyer: BuyerId<T>,
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
        /// The game is not found.
        GameNotFound,
        /// The game already exists.
        GameAlreadyExists,
        /// The game details are invalid.
        GameDetailsInvalid,
        /// The order is already placed.
        OrderAlreadyPlaced,
        /// The order is not found.
        OrderNotFound,
    }

    /// Dispatchable functions ([`Call`]s).
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Adds a new game to the system.
        ///
        /// This function adds a game by storing their details in the `PublishedGames` storage. It
        /// checks that the game does not already exist in the system before adding
        /// them. A `GameAdded` event is emitted once the game is successfully added.
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::game_add(details.name.len() as u32, details.tags.len() as u32))]
        pub fn game_add(
            origin: OriginFor<T>,
            game_id: GameId,
            details: GameDetailsOf<T>,
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
            ensure!(
                details.is_valid(|x| Tags::<T>::contains_key(x)),
                Error::<T>::GameDetailsInvalid
            );

            PublishedGames::<T>::insert(&publisher, game_id, details);

            Self::deposit_event(Event::GameAdded { publisher, game_id });
            Ok(())
        }

        /// Places an order for a game.
        ///
        /// This function places an order for a game by storing their details in the `BuyerOrders`
        /// and `PublisherOrders` storage. It checks that the game exists in the system before
        /// adding them. A `OrderPlaced` event is emitted once the order is successfully added.
        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::order_place())]
        pub fn order_place(
            origin: OriginFor<T>,
            publisher: PublisherId<T>,
            game_id: GameId,
        ) -> DispatchResult {
            let buyer = ensure_signed(origin)?;
            ensure!(
                !BuyerOrders::<T>::contains_key(&buyer, (&publisher, game_id)),
                Error::<T>::OrderAlreadyPlaced
            );
            ensure!(
                !OwnedGames::<T>::contains_key(&buyer, (&publisher, game_id)),
                Error::<T>::GameAlreadyExists
            );

            let game_details =
                PublishedGames::<T>::get(&publisher, game_id).ok_or(Error::<T>::GameNotFound)?;

            T::Currency::hold(&HoldReason::GamePayment.into(), &buyer, game_details.price)?;

            let order = OrderDetails { deposit: game_details.price };
            BuyerOrders::<T>::insert(&buyer, (&publisher, game_id), &order);
            PublisherOrders::<T>::insert(&publisher, game_id, &buyer);

            Self::deposit_event(Event::OrderPlaced { buyer, publisher, game_id });
            Ok(())
        }

        /// Fulfills an order for a game.
        ///
        /// This function is triggered by the publisher when they want to fulfill an order.
        /// It checks that the order exists, transfers the deposit from the buyer to the publisher,
        /// and then removes the order from the system, adding the game to the owned games list
        /// for the buyer. A `OrderFulfilled` event is emitted once the order is fulfilled.
        #[pallet::call_index(2)]
        #[pallet::weight(T::WeightInfo::order_fulfill())]
        pub fn order_fulfill(
            origin: OriginFor<T>,
            game_id: GameId,
            buyer: BuyerId<T>,
        ) -> DispatchResult {
            let publisher = ensure_signed(origin)?;

            let order = BuyerOrders::<T>::get(&buyer, (&publisher, game_id))
                .ok_or(Error::<T>::OrderNotFound)?;

            T::Currency::transfer_on_hold(
                &HoldReason::GamePayment.into(),
                &buyer,
                &publisher,
                order.deposit,
                BestEffort,
                Free,
                Polite,
            )?;

            BuyerOrders::<T>::remove(&buyer, (&publisher, game_id));
            PublisherOrders::<T>::remove(&publisher, game_id);
            OwnedGames::<T>::insert(&buyer, (&publisher, game_id), ());

            Self::deposit_event(Event::OrderFulfilled { buyer, publisher, game_id });
            Ok(())
        }
    }
}
