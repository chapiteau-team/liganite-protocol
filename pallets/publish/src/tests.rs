use crate::{mock::*, types::PublisherDetails, Error, Event, Publishers};
use frame_support::{assert_noop, assert_ok};
use liganite_primitives::testing::bounded_vec;

#[test]
fn test_publisher_add() {
    new_test_ext().execute_with(|| {
        // Go past genesis block so events get deposited
        System::set_block_number(1);

        let publisher = 1;
        let details = PublisherDetails {
            name: bounded_vec(b"Example Publisher"),
            url: bounded_vec(b"https://example.com"),
        };
        assert_ok!(Publish::publisher_add(RuntimeOrigin::signed(publisher), details.clone()));

        assert_eq!(Publishers::<Test>::get(publisher), Some(details));
        System::assert_last_event(Event::PublisherAdded { publisher }.into());
    });
}

#[test]
fn test_publisher_add_already_exists() {
    new_test_ext().execute_with(|| {
        let publisher = 1;
        let details = PublisherDetails {
            name: bounded_vec(b"Example Publisher"),
            url: bounded_vec(b"https://example.com"),
        };
        Publishers::<Test>::insert(publisher, details.clone());

        assert_noop!(
            Publish::publisher_add(RuntimeOrigin::signed(publisher), details),
            Error::<Test>::PublisherAlreadyExists
        );
    });
}
