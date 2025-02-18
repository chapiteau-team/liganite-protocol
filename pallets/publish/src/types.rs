use crate::*;

pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
pub type PublisherId<T> = AccountIdOf<T>;

#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, MaxEncodedLen, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct PublisherDetails {
    /// The name of the publisher
    pub name: Name,
    /// The URL of the publisher
    pub url: Url,
}
