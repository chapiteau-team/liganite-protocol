use crate::{MAX_NAME_SIZE, MAX_URL_SIZE};
use frame_support::pallet_prelude::*;

pub type Name = BoundedVec<u8, ConstU32<MAX_NAME_SIZE>>;
pub type Url = BoundedVec<u8, ConstU32<MAX_URL_SIZE>>;

pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
pub type PublisherId<T> = AccountIdOf<T>;
pub type GameId = u32;

#[derive(Default, Clone, Eq, PartialEq, Debug, Encode, Decode, MaxEncodedLen, TypeInfo)]
#[scale_info(skip_type_params(T))]
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
#[scale_info(skip_type_params(T))]
pub struct GameDetails {
    /// The name of the game
    pub name: Name,
}

impl GameDetails {
    pub fn is_valid(&self) -> bool {
        !self.name.is_empty()
    }
}
