use crate::{validate, MAX_CID_SIZE, MAX_NAME_SIZE, MAX_TAGS_PER_GAME, MAX_TAG_SIZE, MAX_URL_SIZE};
use frame_support::pallet_prelude::*;

pub type Name = BoundedVec<u8, ConstU32<MAX_NAME_SIZE>>;
pub type Url = BoundedVec<u8, ConstU32<MAX_URL_SIZE>>;
pub type Tag = BoundedVec<u8, ConstU32<MAX_TAG_SIZE>>;
pub type GameTags = BoundedVec<TagId, ConstU32<MAX_TAGS_PER_GAME>>;
pub type Cid = BoundedVec<u8, ConstU32<MAX_CID_SIZE>>;

pub type GameId = u16;
pub type GlobalGameId<T> = (PublisherId<T>, GameId);
pub type TagId = u16;

pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
pub type PublisherId<T> = AccountIdOf<T>;
pub type BuyerId<T> = AccountIdOf<T>;

#[derive(
    Clone, Eq, PartialEq, Debug, Encode, Decode, DecodeWithMemTracking, MaxEncodedLen, TypeInfo,
)]
pub struct PublisherDetails {
    /// The name of the publisher
    pub name: Name,
    /// The URL of the publisher
    pub url: Url,
}

impl PublisherDetails {
    pub fn is_valid(&self) -> bool {
        validate::is_non_empty_string(&self.name) && validate::is_url(&self.url)
    }
}

#[derive(
    Clone, Eq, PartialEq, Debug, Encode, Decode, DecodeWithMemTracking, MaxEncodedLen, TypeInfo,
)]
#[scale_info(skip_type_params(Currency))]
pub enum Distribution<Currency> {
    /// The game is distributed free of charge
    Free {
        /// The CID of the game
        cid: Cid,
    },
    /// The game can be purchased instantly. In this case the price is deducted from the buyer's
    /// balance and the game is added to their collection.
    Instant {
        /// The price of the game
        price: Currency,
        /// The CID of the game
        cid: Cid,
    },
    /// The game is distributed in individual form. In this case the price is deposited and order
    /// is created. The publisher can then fulfill the order, providing additional details
    /// about the game's distribution.
    Individual {
        /// The price of the game
        price: Currency,
    },
}

impl<Currency: Zero> Distribution<Currency> {
    pub fn is_valid(&self) -> bool {
        match self {
            Distribution::Free { cid } => validate::is_cid(cid),
            Distribution::Instant { price, cid } => !price.is_zero() && validate::is_cid(cid),
            Distribution::Individual { price } => !price.is_zero(),
        }
    }
}

#[derive(
    Clone, Eq, PartialEq, Debug, Encode, Decode, DecodeWithMemTracking, MaxEncodedLen, TypeInfo,
)]
#[scale_info(skip_type_params(Currency))]
pub struct GameDetails<Currency> {
    /// The name of the game
    pub name: Name,
    /// The tags of the game
    pub tags: GameTags,
    /// The way the game is distributed
    pub distribution: Distribution<Currency>,
}

impl<Currency: Zero> GameDetails<Currency> {
    pub fn is_valid<V: Fn(&TagId) -> bool>(&self, valid_tag: V) -> bool {
        validate::is_non_empty_string(&self.name) &&
            self.distribution.is_valid() &&
            self.tags.iter().all(valid_tag)
    }
}

#[derive(Default, Clone, Eq, PartialEq, Debug, Encode, Decode, MaxEncodedLen, TypeInfo)]
#[scale_info(skip_type_params(Currency))]
pub struct OrderDetails<Currency> {
    /// The deposit held from the buyer
    pub deposit: Currency,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::bounded_vec;

    #[test]
    fn test_publisher_details_is_valid() {
        let details = PublisherDetails {
            name: bounded_vec(b"Example Publisher"),
            url: bounded_vec(b"https://example.com"),
        };

        assert!(details.is_valid());
    }

    #[test]
    fn test_publisher_details_name_is_empty() {
        let details =
            PublisherDetails { name: bounded_vec(b""), url: bounded_vec(b"https://example.com") };

        assert!(!details.is_valid());
    }

    #[test]
    fn test_publisher_details_url_is_empty() {
        let details =
            PublisherDetails { name: bounded_vec(b"Example Publisher"), url: bounded_vec(b"") };

        assert!(!details.is_valid());
    }

    #[test]
    fn test_publisher_details_url_is_invalid() {
        let details = PublisherDetails {
            name: bounded_vec(b"Example Publisher"),
            url: bounded_vec(b"wrong url"),
        };

        assert!(!details.is_valid());
    }

    #[test]
    fn test_game_details_is_valid() {
        let details = GameDetails {
            name: bounded_vec(b"Example Game"),
            tags: bounded_vec(&[1, 2, 3]),
            distribution: Distribution::Instant {
                price: 1234,
                cid: bounded_vec(b"QmYwAPJzv5CZsnAztbCxjRMoa6zFzFG8pGzLFZxojtL8MX"),
            },
        };

        assert!(details.is_valid(|_| true));
    }

    #[test]
    fn test_game_details_name_is_empty() {
        let details = GameDetails {
            name: bounded_vec(b""),
            tags: bounded_vec(&[1, 2, 3]),
            distribution: Distribution::Instant {
                price: 1234,
                cid: bounded_vec(b"QmYwAPJzv5CZsnAztbCxjRMoa6zFzFG8pGzLFZxojtL8MX"),
            },
        };

        assert!(!details.is_valid(|_| true));
    }

    #[test]
    fn test_game_details_tags_are_empty() {
        let details = GameDetails {
            name: bounded_vec(b"Example Game"),
            tags: bounded_vec(&[]),
            distribution: Distribution::Instant {
                price: 1234,
                cid: bounded_vec(b"QmYwAPJzv5CZsnAztbCxjRMoa6zFzFG8pGzLFZxojtL8MX"),
            },
        };

        // empty tags are valid
        assert!(details.is_valid(|_| true));
    }

    #[test]
    fn test_game_details_tags_are_invalid() {
        let details = GameDetails {
            name: bounded_vec(b""),
            tags: bounded_vec(&[1, 2, 3]),
            distribution: Distribution::Instant {
                price: 1234,
                cid: bounded_vec(b"QmYwAPJzv5CZsnAztbCxjRMoa6zFzFG8pGzLFZxojtL8MX"),
            },
        };

        assert!(!details.is_valid(|_| false));
    }

    #[test]
    fn test_game_details_distribution_is_invalid() {
        let details = GameDetails {
            name: bounded_vec(b""),
            tags: bounded_vec(&[1, 2, 3]),
            distribution: Distribution::Instant {
                price: 0,
                cid: bounded_vec(b"QmYwAPJzv5CZsnAztbCxjRMoa6zFzFG8pGzLFZxojtL8MX"),
            },
        };

        assert!(!details.is_valid(|_| true));
    }
}
