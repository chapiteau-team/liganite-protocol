use crate as liganite_games;
use frame_support::{
    derive_impl,
    traits::{ConstU64, VariantCountOf},
};
use liganite_primitives::{
    publisher::PublisherManager,
    testing::bounded_vec,
    types::{BuyerId, PublisherDetails, PublisherId},
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
    pub type Publish = liganite_publish::Pallet<Test>;

    #[runtime::pallet_index(3)]
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

impl liganite_publish::Config for Test {
    type WeightInfo = ();
    type RuntimeEvent = RuntimeEvent;
    type RuntimeHoldReason = RuntimeHoldReason;
    type Currency = Balances;
}

impl liganite_games::Config for Test {
    type WeightInfo = ();
    type RuntimeEvent = RuntimeEvent;
    type RuntimeHoldReason = RuntimeHoldReason;
    type Currency = Balances;
    type PublisherManager = Publish;
}

pub const PUBLISHER_DEPOSIT: Balance = 1_000_000;
pub const INITIAL_BALANCE: Balance = 1_000_000_000;

pub const INVALID_PUBLISHER: PublisherId<Test> = 0;
pub const PUBLISHER: PublisherId<Test> = 1;
pub const FUNDED_BUYER: BuyerId<Test> = 11;
pub const NON_FUNDED_BUYER: BuyerId<Test> = 12;

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut storage = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();

    pallet_balances::GenesisConfig::<Test> {
        balances: vec![(PUBLISHER, INITIAL_BALANCE), (FUNDED_BUYER, INITIAL_BALANCE)],
    }
    .assimilate_storage(&mut storage)
    .unwrap();

    liganite_publish::GenesisConfig::<Test> { publisher_deposit: PUBLISHER_DEPOSIT }
        .assimilate_storage(&mut storage)
        .unwrap();

    let mut ext = sp_io::TestExternalities::new(storage);
    ext.execute_with(|| {
        Publish::insert_publisher(
            &PUBLISHER,
            &PublisherDetails {
                name: bounded_vec(b"PUBLISHER"),
                url: bounded_vec(b"https://publisher.mock"),
            },
        )
    });

    // Go past genesis block so events get deposited
    ext.execute_with(|| System::set_block_number(1));
    ext
}
