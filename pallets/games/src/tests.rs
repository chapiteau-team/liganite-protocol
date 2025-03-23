use crate::{
    mock::*, BuyerOrders, Error, Event, HoldReason, OwnedGames, PublishedGames, PublisherOrders,
};
use frame_support::{assert_noop, assert_ok, traits::fungible};
use liganite_primitives::{
    testing::bounded_vec,
    types::{GameDetails, OrderDetails},
};
use sp_runtime::TokenError;

#[test]
fn test_game_add() {
    new_test_ext().execute_with(|| {
        let game_id = 1;
        let details = GameDetails { name: bounded_vec(b"Example Game"), ..Default::default() };
        assert_ok!(Games::game_add(RuntimeOrigin::signed(PUBLISHER), game_id, details.clone()));

        assert_eq!(PublishedGames::<Test>::get(PUBLISHER, game_id), Some(details));
        System::assert_last_event(Event::GameAdded { publisher: PUBLISHER, game_id }.into());
    })
}

#[test]
fn test_game_add_game_already_exists() {
    new_test_ext().execute_with(|| {
        let game_id = 1;
        let details = GameDetails { name: bounded_vec(b"Example Game"), ..Default::default() };
        PublishedGames::<Test>::insert(PUBLISHER, game_id, details.clone());

        assert_noop!(
            Games::game_add(RuntimeOrigin::signed(PUBLISHER), game_id, details),
            Error::<Test>::GameAlreadyExists
        );
    });
}

#[test]
fn test_game_add_invalid_publisher() {
    new_test_ext().execute_with(|| {
        let game_id = 1;
        let details = GameDetails { name: bounded_vec(b"Example Game"), ..Default::default() };
        assert_noop!(
            Games::game_add(RuntimeOrigin::signed(INVALID_PUBLISHER), game_id, details),
            Error::<Test>::InvalidPublisher
        );
    });
}

#[test]
fn test_game_add_empty_name() {
    new_test_ext().execute_with(|| {
        let game_id = 1;
        let details = GameDetails { name: bounded_vec(b""), ..Default::default() };
        assert_noop!(
            Games::game_add(RuntimeOrigin::signed(PUBLISHER), game_id, details),
            Error::<Test>::GameDetailsInvalid
        );
    });
}

#[test]
fn test_order_place() {
    new_test_ext().execute_with(|| {
        let game_id = 1;
        let price = 12345;
        let details =
            GameDetails { name: bounded_vec(b"Example Game"), price, ..Default::default() };
        PublishedGames::<Test>::insert(PUBLISHER, game_id, details.clone());

        assert_ok!(Games::order_place(RuntimeOrigin::signed(FUNDED_BUYER), PUBLISHER, game_id));

        let expected = OrderDetails { deposit: price };
        assert_eq!(BuyerOrders::<Test>::get(FUNDED_BUYER, (PUBLISHER, game_id)), Some(expected));
        assert_eq!(PublisherOrders::<Test>::get(PUBLISHER, game_id), Some(FUNDED_BUYER));
        assert_eq!(
            <Balances as fungible::Inspect<_>>::balance(&FUNDED_BUYER),
            INITIAL_BALANCE - price
        );
        assert_eq!(
            <Balances as fungible::hold::Inspect<_>>::balance_on_hold(
                &HoldReason::GamePayment.into(),
                &FUNDED_BUYER
            ),
            price
        );
        System::assert_last_event(
            Event::OrderPlaced { buyer: FUNDED_BUYER, publisher: PUBLISHER, game_id }.into(),
        );
    })
}

#[test]
fn test_order_place_no_funds() {
    new_test_ext().execute_with(|| {
        let game_id = 1;
        let price = 12345;
        let details =
            GameDetails { name: bounded_vec(b"Example Game"), price, ..Default::default() };
        PublishedGames::<Test>::insert(PUBLISHER, game_id, details.clone());

        assert_noop!(
            Games::order_place(RuntimeOrigin::signed(NON_FUNDED_BUYER), PUBLISHER, game_id),
            TokenError::FundsUnavailable
        );
    })
}

#[test]
fn test_order_place_invalid_game() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            Games::order_place(RuntimeOrigin::signed(FUNDED_BUYER), PUBLISHER, 1),
            Error::<Test>::GameNotFound
        );
    })
}

#[test]
fn test_order_place_already_placed() {
    new_test_ext().execute_with(|| {
        let game_id = 1;
        let price = 12345;
        let details =
            GameDetails { name: bounded_vec(b"Example Game"), price, ..Default::default() };
        PublishedGames::<Test>::insert(PUBLISHER, game_id, details.clone());

        assert_ok!(Games::order_place(RuntimeOrigin::signed(FUNDED_BUYER), PUBLISHER, game_id));

        assert_noop!(
            Games::order_place(RuntimeOrigin::signed(FUNDED_BUYER), PUBLISHER, game_id),
            Error::<Test>::OrderAlreadyPlaced
        );
    })
}

#[test]
fn test_order_place_owned_game() {
    new_test_ext().execute_with(|| {
        let game_id = 1;
        OwnedGames::<Test>::insert(FUNDED_BUYER, (PUBLISHER, game_id), ());

        assert_noop!(
            Games::order_place(RuntimeOrigin::signed(FUNDED_BUYER), PUBLISHER, game_id),
            Error::<Test>::GameAlreadyExists
        );
    })
}

#[test]
fn test_order_fulfill() {
    new_test_ext().execute_with(|| {
        let game_id = 1;
        let price = 12345;
        let details =
            GameDetails { name: bounded_vec(b"Example Game"), price, ..Default::default() };
        PublishedGames::<Test>::insert(PUBLISHER, game_id, details.clone());

        assert_ok!(Games::order_place(RuntimeOrigin::signed(FUNDED_BUYER), PUBLISHER, game_id));

        assert_ok!(Games::order_fulfill(RuntimeOrigin::signed(PUBLISHER), game_id, FUNDED_BUYER));

        assert_eq!(BuyerOrders::<Test>::get(FUNDED_BUYER, (PUBLISHER, game_id)), None);
        assert_eq!(PublisherOrders::<Test>::get(PUBLISHER, game_id), None);
        assert_eq!(OwnedGames::<Test>::get(FUNDED_BUYER, (PUBLISHER, game_id)), Some(()));
        assert_eq!(
            <Balances as fungible::Inspect<_>>::balance(&PUBLISHER),
            INITIAL_BALANCE + price
        );
        assert_eq!(
            <Balances as fungible::Inspect<_>>::balance(&FUNDED_BUYER),
            INITIAL_BALANCE - price
        );
        assert_eq!(
            <Balances as fungible::hold::Inspect<_>>::balance_on_hold(
                &HoldReason::GamePayment.into(),
                &FUNDED_BUYER
            ),
            0
        );
        System::assert_last_event(
            Event::OrderFulfilled { buyer: FUNDED_BUYER, publisher: PUBLISHER, game_id }.into(),
        );
    })
}

#[test]
fn test_order_fulfill_missing_order() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            Games::order_fulfill(RuntimeOrigin::signed(PUBLISHER), 1, FUNDED_BUYER),
            Error::<Test>::OrderNotFound
        );
    })
}

#[test]
fn test_order_fulfill_invalid_game() {
    new_test_ext().execute_with(|| {
        let game_id = 1;
        let price = 12345;
        let details =
            GameDetails { name: bounded_vec(b"Example Game"), price, ..Default::default() };
        PublishedGames::<Test>::insert(PUBLISHER, game_id, details.clone());

        assert_ok!(Games::order_place(RuntimeOrigin::signed(FUNDED_BUYER), PUBLISHER, game_id));

        let invalid_game_id = game_id + 1;
        assert_noop!(
            Games::order_fulfill(RuntimeOrigin::signed(PUBLISHER), invalid_game_id, FUNDED_BUYER),
            Error::<Test>::OrderNotFound
        );
    })
}
