use crate::types::PublisherDetails;

pub trait PublisherManager {
    type PublisherId;

    fn is_valid_publisher(publisher_id: &Self::PublisherId) -> bool;

    fn insert_publisher(publisher_id: &Self::PublisherId, details: &PublisherDetails);
}
