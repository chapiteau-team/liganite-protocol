use crate::{mock::*, Error, Event, PublisherDeposit, Publishers};
use frame_support::{assert_noop, assert_ok};
use liganite_primitives::{
    publisher::PublisherManager, testing::bounded_vec, types::PublisherDetails,
};
use sp_runtime::TokenError;

#[test]
fn test_deposit_set() {
    new_test_ext().execute_with(|| {
        assert_eq!(PublisherDeposit::<Test>::get(), PUBLISHER_DEPOSIT);

        let new_deposit = PUBLISHER_DEPOSIT * 3;
        assert_ok!(Publish::deposit_set(RuntimeOrigin::root(), new_deposit));

        assert_eq!(PublisherDeposit::<Test>::get(), new_deposit);
    });
}

#[test]
fn test_publisher_register() {
    new_test_ext().execute_with(|| {
        let details = PublisherDetails {
            name: bounded_vec(b"Example Publisher"),
            url: bounded_vec(b"https://example.com"),
        };
        assert_ok!(Publish::publisher_register(
            RuntimeOrigin::signed(FUNDED_PUBLISHER),
            details.clone()
        ));

        assert_eq!(Publishers::<Test>::get(FUNDED_PUBLISHER), Some(details));
        System::assert_last_event(Event::PublisherAdded { publisher: FUNDED_PUBLISHER }.into());
    });
}

#[test]
fn test_publisher_register_already_exists() {
    new_test_ext().execute_with(|| {
        let details = PublisherDetails {
            name: bounded_vec(b"Example Publisher"),
            url: bounded_vec(b"https://example.com"),
        };
        Publishers::<Test>::insert(FUNDED_PUBLISHER, details.clone());

        assert_noop!(
            Publish::publisher_register(RuntimeOrigin::signed(FUNDED_PUBLISHER), details),
            Error::<Test>::PublisherAlreadyExists
        );
    });
}

#[test]
fn test_publisher_register_empty_name() {
    new_test_ext().execute_with(|| {
        let details =
            PublisherDetails { name: bounded_vec(b""), url: bounded_vec(b"https://example.com") };

        assert_noop!(
            Publish::publisher_register(RuntimeOrigin::signed(FUNDED_PUBLISHER), details),
            Error::<Test>::PublisherDetailsInvalid
        );
    });
}

#[test]
fn test_publisher_register_no_funds() {
    new_test_ext().execute_with(|| {
        let details = PublisherDetails {
            name: bounded_vec(b"Example Publisher"),
            url: bounded_vec(b"https://example.com"),
        };
        assert_noop!(
            Publish::publisher_register(RuntimeOrigin::signed(NON_FUNDED_PUBLISHER), details),
            TokenError::FundsUnavailable
        );
    });
}

#[test]
fn test_publisher_manager_is_valid_publisher() {
    new_test_ext().execute_with(|| {
        let details = PublisherDetails {
            name: bounded_vec(b"Example Publisher"),
            url: bounded_vec(b"https://example.com"),
        };
        Publishers::<Test>::insert(FUNDED_PUBLISHER, details);

        assert!(Publish::is_valid_publisher(&FUNDED_PUBLISHER));
        assert!(!Publish::is_valid_publisher(&2));
    })
}
