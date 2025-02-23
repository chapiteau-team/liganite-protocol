use crate::{mock::*, Error, Event, PublishedGames};
use frame_support::{assert_noop, assert_ok};
use liganite_primitives::{testing::bounded_vec, types::GameDetails};

#[test]
fn test_game_add() {
    new_test_ext().execute_with(|| {
        // Go past genesis block so events get deposited
        System::set_block_number(1);

        let publisher = 1;
        let game_id = 1;
        let details = GameDetails { name: bounded_vec(b"Example Game") };
        assert_ok!(Games::game_add(RuntimeOrigin::signed(publisher), game_id, details.clone()));

        assert_eq!(PublishedGames::<Test>::get(publisher, game_id), Some(details));
        System::assert_last_event(Event::GameAdded { publisher, game_id }.into());
    })
}

#[test]
fn test_game_add_game_already_exists() {
    new_test_ext().execute_with(|| {
        let publisher = 1;
        let game_id = 1;
        let details = GameDetails { name: bounded_vec(b"Example Game") };
        PublishedGames::<Test>::insert(publisher, game_id, details.clone());

        assert_noop!(
            Games::game_add(RuntimeOrigin::signed(publisher), game_id, details),
            Error::<Test>::GameAlreadyExists
        );
    });
}

#[test]
fn test_game_add_invalid_publisher() {
    new_test_ext().execute_with(|| {
        let publisher = INVALID_PUBLISHER;
        let game_id = 1;
        let details = GameDetails { name: bounded_vec(b"Example Game") };
        assert_noop!(
            Games::game_add(RuntimeOrigin::signed(publisher), game_id, details),
            Error::<Test>::InvalidPublisher
        );
    });
}

#[test]
fn test_game_add_empty_name() {
    new_test_ext().execute_with(|| {
        let publisher = 1;
        let game_id = 1;
        let details = GameDetails { name: bounded_vec(b"") };
        assert_noop!(
            Games::game_add(RuntimeOrigin::signed(publisher), game_id, details),
            Error::<Test>::GameDetailsInvalid
        );
    });
}
