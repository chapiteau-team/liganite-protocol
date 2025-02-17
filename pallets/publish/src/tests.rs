use crate::{
    mock::*,
    types::{PublisherDetails, Url},
    Error, Event, Publishers,
};
use frame_support::{assert_noop, assert_ok};

#[test]
fn test_publisher_add() {
    new_test_ext().execute_with(|| {
        // Go past genesis block so events get deposited
        System::set_block_number(1);

        let publisher = 1;
        let details =
            PublisherDetails { url: Url::try_from(b"https://example.com".to_vec()).unwrap() };
        assert_ok!(Publish::publisher_add(RuntimeOrigin::signed(publisher), details.clone()));

        assert_eq!(Publishers::<Test>::get(publisher), Some(details));
        System::assert_last_event(Event::PublisherAdded { who: publisher }.into());
    });
}

#[test]
fn test_publisher_add_already_exists() {
    new_test_ext().execute_with(|| {
        let publisher = 1;
        let details =
            PublisherDetails { url: Url::try_from(b"https://example.com".to_vec()).unwrap() };
        Publishers::<Test>::insert(publisher, details.clone());

        assert_noop!(
            Publish::publisher_add(RuntimeOrigin::signed(publisher), details),
            Error::<Test>::PublisherAlreadyExists
        );
    });
}
