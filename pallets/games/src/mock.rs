use crate as liganite_games;
use frame_support::{
    derive_impl,
    traits::{ConstU64, VariantCountOf},
};
use liganite_primitives::{
    publisher::PublisherManager,
    types::{PublisherDetails, PublisherId},
};
use sp_runtime::BuildStorage;

type Block = frame_system::mocking::MockBlock<Test>;
type Balance = u64;

#[frame_support::runtime]
mod runtime {
    // The main runtime
    #[runtime::runtime]
    // Runtime Types to be generated
    #[runtime::derive(
        RuntimeCall,
        RuntimeEvent,
        RuntimeError,
        RuntimeOrigin,
        RuntimeFreezeReason,
        RuntimeHoldReason,
        RuntimeSlashReason,
        RuntimeLockId,
        RuntimeTask
    )]
    pub struct Test;

    #[runtime::pallet_index(0)]
    pub type System = frame_system::Pallet<Test>;

    #[runtime::pallet_index(1)]
    pub type Balances = pallet_balances::Pallet<Test>;

    #[runtime::pallet_index(2)]
    pub type Games = liganite_games::Pallet<Test>;
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
    type Block = Block;
    type AccountData = pallet_balances::AccountData<u64>;
}

impl pallet_balances::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeHoldReason = RuntimeHoldReason;
    type RuntimeFreezeReason = RuntimeFreezeReason;
    type WeightInfo = ();
    type Balance = Balance;
    type DustRemoval = ();
    type ExistentialDeposit = ConstU64<1>;
    type AccountStore = System;
    type ReserveIdentifier = [u8; 8];
    type FreezeIdentifier = RuntimeFreezeReason;
    type MaxLocks = ();
    type MaxReserves = ();
    type MaxFreezes = VariantCountOf<RuntimeFreezeReason>;
    type DoneSlashHandler = ();
}

pub struct MockPublisherManager;

pub const INVALID_PUBLISHER: PublisherId<Test> = 0;

impl PublisherManager for MockPublisherManager {
    type PublisherId = PublisherId<Test>;

    fn is_valid_publisher(publisher_id: &Self::PublisherId) -> bool {
        publisher_id != &INVALID_PUBLISHER
    }

    fn insert_publisher(_publisher_id: &Self::PublisherId, _details: &PublisherDetails) {}
}

impl liganite_games::Config for Test {
    type WeightInfo = ();
    type RuntimeEvent = RuntimeEvent;
    type PublisherManager = MockPublisherManager;
    type Currency = Balances;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    frame_system::GenesisConfig::<Test>::default().build_storage().unwrap().into()
}
