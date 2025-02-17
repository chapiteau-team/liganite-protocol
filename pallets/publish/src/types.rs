use crate::*;

pub const MAX_URL_SIZE: u32 = 125;

pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
pub type PublisherId<T> = AccountIdOf<T>;
pub type Url = BoundedVec<u8, ConstU32<MAX_URL_SIZE>>;

#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, MaxEncodedLen, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct PublisherDetails {
    /// The URL of the publisher
    pub url: Url,
}
