use crate::mock::*;
use frame_support::{assert_noop, assert_ok};
// Learn more about testing substrate runtime modules
// https://substrate.dev/docs/en/knowledgebase/runtime/tests
// type Module = super::Module<Test>;
type Timestamp = pallet_timestamp::Module<Test>;
type System = frame_system::Module<Test>;
type Error = super::Error<Test>;

fn events() -> Vec<Event> {
    let evt = System::events()
        .into_iter()
        .map(|evt| evt.event)
        .collect::<Vec<_>>();
    System::reset_events();
    evt
}
#[allow(dead_code)]
fn last_event() -> Event {
    System::events().pop().expect("Event expected").event
}

#[test]
fn it_works_for_default_value() {
    new_test_ext().execute_with(|| {
        System::set_block_number(2);
        // Dispatch a signed extrinsic.
        assert_ok!(TemplateModule::do_something(Origin::signed(1), 42));
        assert_eq!(
            events(),
            vec![Event::template(super::RawEvent::SomethingStored(42, 1))]
        );

        // Read pallet storage and assert an expected result.
        assert_eq!(TemplateModule::something(), Some(42));
        System::set_block_number(3);
        assert_eq!(events(), vec![]);
    });
}

#[test]
fn it_create_new_account() {
    new_test_ext().execute_with(|| {
        let account = TemplateModule::account_registry(2);
        assert!(!account.is_enable());

        Timestamp::set_timestamp(5000);
        assert_ok!(TemplateModule::account_add(
            Origin::signed(1),
            2,
            super::USER_ROLE
        ));

        let account = TemplateModule::account_registry(2);
        assert!(account.is_enable());

        let age = account.age(20000);
        assert_eq!(age, 15000);

        assert_ok!(TemplateModule::do_something(Origin::signed(2), 20));
        System::set_block_number(2);
        assert_eq!(TemplateModule::something(), Some(20));
    });
}

#[test]
fn it_disable_account() {
    new_test_ext().execute_with(|| {
        assert_ok!(TemplateModule::account_add(
            Origin::signed(1),
            2,
            super::USER_ROLE
        ));

        assert_ok!(TemplateModule::account_disable(Origin::signed(1), 2));
    });
}

#[test]
fn it_try_disable_themself() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            TemplateModule::account_disable(Origin::signed(1), 1),
            Error::InvalidAction
        );
    });
}

#[test]
fn it_try_create_by_user() {
    new_test_ext().execute_with(|| {
        assert_ok!(TemplateModule::account_add(
            Origin::signed(1),
            2,
            super::USER_ROLE
        ));
        assert_noop!(
            TemplateModule::account_add(Origin::signed(2), 3, super::USER_ROLE),
            Error::NotAuthorized
        );
    });
}

#[test]
fn it_balance() {
    new_test_ext().execute_with(|| {
        assert_eq!(<pallet_balances::Module<Test>>::free_balance(1), 100000);
        assert_eq!(<pallet_balances::Module<Test>>::free_balance(2), 0);
        assert_ok!(TemplateModule::account_add(
            Origin::signed(1),
            3,
            super::USER_ROLE
        ));

        assert_ok!(<pallet_balances::Module<Test>>::transfer(
            Origin::signed(1),
            2,
            50000
        ));

        assert_eq!(<pallet_balances::Module<Test>>::free_balance(1), 50000);
        assert_eq!(<pallet_balances::Module<Test>>::free_balance(2), 50000);
    });
}
