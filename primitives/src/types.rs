use crate::{MAX_NAME_SIZE, MAX_TAGS_PER_GAME, MAX_TAG_SIZE, MAX_URL_SIZE};
use frame_support::pallet_prelude::*;

pub type Name = BoundedVec<u8, ConstU32<MAX_NAME_SIZE>>;
pub type Url = BoundedVec<u8, ConstU32<MAX_URL_SIZE>>;
pub type Tag = BoundedVec<u8, ConstU32<MAX_TAG_SIZE>>;
pub type GameTags = BoundedVec<TagId, ConstU32<MAX_TAGS_PER_GAME>>;

pub type GameId = u16;
pub type GlobalGameId<T> = (PublisherId<T>, GameId);
pub type TagId = u16;

pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
pub type PublisherId<T> = AccountIdOf<T>;
pub type BuyerId<T> = AccountIdOf<T>;

#[derive(Default, Clone, Eq, PartialEq, Debug, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub struct PublisherDetails {
    /// The name of the publisher
    pub name: Name,
    /// The URL of the publisher
    pub url: Url,
}

impl PublisherDetails {
    pub fn is_valid(&self) -> bool {
        !self.name.is_empty()
    }
}

#[derive(Default, Clone, Eq, PartialEq, Debug, Encode, Decode, MaxEncodedLen, TypeInfo)]
#[scale_info(skip_type_params(Currency))]
pub struct GameDetails<Currency> {
    /// The name of the game
    pub name: Name,
    /// The tags of the game
    pub tags: GameTags,
    /// The price of the game
    pub price: Currency,
}

impl<Currency> GameDetails<Currency> {
    pub fn is_valid<V: Fn(&TagId) -> bool>(&self, valid_tag: V) -> bool {
        !self.name.is_empty() && self.tags.iter().all(valid_tag)
    }
}

#[derive(Default, Clone, Eq, PartialEq, Debug, Encode, Decode, MaxEncodedLen, TypeInfo)]
#[scale_info(skip_type_params(Currency))]
pub struct OrderDetails<Currency> {
    pub deposit: Currency,
}
